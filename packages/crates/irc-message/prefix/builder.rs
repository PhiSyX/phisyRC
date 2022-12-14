/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::str::Chars;

use lang::{
	codepoints::CodePoint,
	lexer::ParseState,
	stream::{InputStream, StreamIterator},
};

use super::{host, nick, state::State, user};
use crate::{prefix, Prefix};

// --------- //
// Structure //
// --------- //

pub(super) struct Builder<'a, 'b> {
	stream: &'a mut InputStream<Chars<'b>, char>,
	temporary_buffer: String,
	state: State,
}

// -------------- //
// Implémentation //
// -------------- //

impl<'a, 'b> Builder<'a, 'b> {
	pub(super) fn initialize(
		stream: &'a mut InputStream<Chars<'b>, char>,
	) -> Self {
		Self {
			stream,
			state: Default::default(),
			temporary_buffer: Default::default(),
		}
	}
}

impl<'a, 'b> Builder<'a, 'b> {
	pub(super) fn analyze(&mut self) -> Result<(), prefix::Error> {
		loop {
			match self.state {
				| State::Initial => {
					match self.stream.consume_next()? {
						// U+003A COLON (:)
						//
						// Ignorer le caractère.
						| CodePoint::COLON => continue,

						// Espaces blancs
						//
						// Arrêter l'analyse si le tampon temporaire vaut
						// "localhost". Autrement, un préfixe NE PEUT PAS
						// contenir d'espaces blancs. Il s'agira d'une erreur
						// d'analyse.
						| codepoint if codepoint.is_whitespace() => {
							if self.temporary_buffer == "localhost" {
								break;
							}

							return Err(prefix::Error::InvalidCharacter {
								found: codepoint.unit(),
								help: "Un point de code valide est attendu",
							});
						}

						// U+0021 EXCLAMATION MARK (!)
						// U+0040 COMMERCIAL AT (@)
						//
						// Passer à l'état [IrcMessagePrefixState::User].
						| codepoint @ (CodePoint::EXCLAMATION_MARK
						| CodePoint::COMMERCIAL_AT)
							if !self.temporary_buffer.is_empty() =>
						{
							self.add_codepoint_to_temporary_buffer(codepoint);
							self.state.switch(State::User);
						}

						// U+002E FULL STOP (.)
						//
						// Passer à l'état [IrcMessagePrefixState::Server].
						| CodePoint::FULL_STOP => {
							self.add_codepoint_to_temporary_buffer(
								CodePoint::FULL_STOP,
							);
							self.state.switch(State::Server);
						}

						// Insérer tous les points de code valides dans le
						// buffer temporaire.
						| codepoint if codepoint.is_valid() => {
							self.add_codepoint_to_temporary_buffer(codepoint);
						}

						| CodePoint::EOF if cfg!(test) => break,

						// Tous autres caractères.
						//
						// Il s'agit d'une erreur d'analyse.
						| _ => return Err(prefix::Error::Parse),
					}
				}

				| State::User => match self.stream.consume_next()? {
					// Espaces blancs
					//
					// Un préfixe NE PEUT PAS contenir d'espaces blancs.
					// Le préfixe est terminé, sortir de la boucle.
					| codepoint if codepoint.is_whitespace() => break,

					// U+0040 COMMERCIAL AT (@)
					//
					// Ajouter le caractère au buffer temporaire.
					| CodePoint::COMMERCIAL_AT => {
						self.add_codepoint_to_temporary_buffer(
							CodePoint::COMMERCIAL_AT,
						);
					}

					// Tous les caractères valides.
					//
					// Ajouter le caractère au buffer temporaire.
					| codepoint if codepoint.is_valid() => {
						self.add_codepoint_to_temporary_buffer(codepoint);
					}

					// Tous autres caractères.
					//
					// Il s'agit d'une erreur d'analyse.
					| _ => return Err(prefix::Error::Parse),
				},

				| State::Server => {
					match self.stream.consume_next()? {
						// Espaces blancs
						//
						// Un préfixe NE PEUT PAS contenir d'espaces blancs.
						// Le préfixe est terminé, sortir de la boucle.
						| codepoint if codepoint.is_whitespace() => break,

						// Tous les caractères valides.
						//
						// Ajouter le caractère au buffer temporaire.
						| codepoint if codepoint.is_valid() => {
							self.add_codepoint_to_temporary_buffer(codepoint);
						}

						// Tous autres caractères.
						//
						// Il s'agit d'une erreur d'analyse.
						| _ => return Err(prefix::Error::Parse),
					}
				}
			}
		}

		Ok(())
	}

	fn add_codepoint_to_temporary_buffer(
		&mut self,
		codepoint: CodePoint<char>,
	) {
		self.temporary_buffer.push(codepoint.unit())
	}
}

impl<'a, 'b> Builder<'a, 'b> {
	pub(super) fn finish(self) -> Result<Prefix, prefix::Error> {
		match self.state {
			// NOTE(phisyx): cas spécifique. Si on ne le fait pas, on tombe
			// dans le cas de Self::User.
			| State::Initial => {
				if self.temporary_buffer == "localhost" {
					return Ok(Prefix::Server {
						origin: self.temporary_buffer,
					});
				}

				Ok(Prefix::User {
					nick: nick::parse(&self.temporary_buffer)?,
					user: Default::default(),
					host: Default::default(),
				})
			}

			| State::User => {
				let mut prefix_user = Prefix::User {
					nick: Default::default(),
					user: Default::default(),
					host: Default::default(),
				};

				if let Some((raw_nick_user, host)) = self
					.temporary_buffer
					.split_once('@')
					.map(|(l, h)| (l, host::parse(h)))
				{
					if let Some((nick, user)) = raw_nick_user
						.split_once('!')
						.map(|(n, i)| (nick::parse(n), user::parse(i)))
					{
						prefix_user.set_nick(nick?);
						prefix_user.set_user(user?);
					} else {
						prefix_user.set_nick(nick::parse(raw_nick_user)?);
					}

					prefix_user.set_host(host?);
				} else if let Some((raw_nick, raw_user)) =
					self.temporary_buffer.split_once('!')
				{
					prefix_user.set_nick(raw_nick);
					prefix_user.set_user(raw_user);
				}

				prefix_user.check_fields()?;

				Ok(prefix_user)
			}

			| State::Server => {
				let prefix_server = Prefix::Server {
					origin: self.temporary_buffer,
				};
				prefix_server.check_fields()?;
				Ok(prefix_server)
			}
		}
	}
}
