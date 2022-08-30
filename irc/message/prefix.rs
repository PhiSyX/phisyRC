/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod host;
mod nick;
mod user;

use std::str::Chars;

use lang::{codepoints::CodePoint, lexer::ParseState, stream::prelude::*};

pub(super) use self::{
	host::IrcMessagePrefixHostError, nick::IrcMessagePrefixNickError,
	user::IrcMessagePrefixUserError,
};
use super::IrcMessageError;

// --------- //
// Structure //
// --------- //

#[derive(Default)]
struct Builder {
	temporary_buffer: String,
	state: BuilderState,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum IrcMessagePrefix {
	User {
		nick: String,
		user: Option<String>,
		host: Option<String>,
	},

	Server {
		origin: String,
	},
}

#[derive(Debug)]
#[derive(Default)]
enum BuilderState {
	#[default]
	Initial,

	User,
	Server,
}

// -------------- //
// Implémentation //
// -------------- //

impl IrcMessagePrefix {
	pub fn parse(
		input: &mut InputStream<Chars<'_>, char>,
	) -> Result<Self, IrcMessageError> {
		let mut builder = Builder::default();

		loop {
			match builder.state {
				| BuilderState::Initial => match input.consume_next()? {
					// U+003A COLON (:)
					//
					// Ignorer le caractère.
					| CodePoint::COLON => continue,

					// Espaces blancs
					//
					// Un préfixe NE PEUT PAS contenir d'espaces blancs.
					// Le préfixe est terminé, sortir de la boucle.
					| codepoint if codepoint.is_whitespace() => break,

					// U+0021 EXCLAMATION MARK (!)
					// U+0040 COMMERCIAL AT (@)
					//
					// Passer à l'état [IrcMessagePrefixState::User].
					| codepoint @ (CodePoint::EXCLAMATION_MARK
					| CodePoint::COMMERCIAL_AT) => {
						builder.push(codepoint);
						builder.state.switch(BuilderState::User);
					}

					// U+002E FULL STOP (.)
					//
					// Passer à l'état [IrcMessagePrefixState::Server].
					| CodePoint::FULL_STOP => {
						builder.push(CodePoint::FULL_STOP);
						builder.state.switch(BuilderState::Server);
					}

					// Insérer tous les points de code valides dans le buffer
					// temporaire.
					| codepoint if codepoint.is_valid() => {
						builder.push(codepoint);
					}

					// Tous autres caractères.
					//
					// Il s'agit d'une erreur d'analyse.
					| _ => {
						return Err(IrcMessageError::InvalidPrefix(
							"Erreur d'analyse".to_owned(),
						))
					}
				},

				| BuilderState::User => match input.consume_next()? {
					// Espaces blancs
					//
					// Un préfixe NE PEUT PAS contenir d'espaces blancs.
					// Le préfixe est terminé, sortir de la boucle.
					| codepoint if codepoint.is_whitespace() => break,

					// U+0040 COMMERCIAL AT (@)
					//
					// Ajouter le caractère au buffer temporaire.
					| CodePoint::COMMERCIAL_AT => {
						builder.push(CodePoint::COMMERCIAL_AT);
					}

					// Tous les caractères valides.
					//
					// Ajouter le caractère au buffer temporaire.
					| codepoint if codepoint.is_valid() => {
						builder.push(codepoint);
					}

					// Tous autres caractères.
					//
					// Il s'agit d'une erreur d'analyse.
					| _ => {
						return Err(IrcMessageError::InvalidPrefix(
							"Erreur d'analyse".to_owned(),
						))
					}
				},

				| BuilderState::Server => match input.consume_next()? {
					// Espaces blancs
					//
					// Un préfixe NE PEUT PAS contenir d'espaces blancs.
					// Le préfixe est terminé, sortir de la boucle.
					| codepoint if codepoint.is_whitespace() => break,

					// Tous les caractères valides.
					//
					// Ajouter le caractère au buffer temporaire.
					| codepoint if codepoint.is_valid() => {
						builder.push(codepoint);
					}

					// Tous autres caractères.
					//
					// Il s'agit d'une erreur d'analyse.
					| _ => {
						return Err(IrcMessageError::InvalidPrefix(
							"Erreur d'analyse".to_owned(),
						))
					}
				},
			}
		}

		builder.build()
	}
}

impl IrcMessagePrefix {
	pub(super) fn set_nick(&mut self, new_nick: impl Into<String>) {
		if let Self::User { nick, .. } = self {
			*nick = new_nick.into();
		}
	}

	pub(super) fn set_user(&mut self, new_user: impl Into<String>) {
		if let Self::User { user, .. } = self {
			*user = Some(new_user.into());
		}
	}

	pub(super) fn set_host(&mut self, new_host: impl Into<String>) {
		if let Self::User { host, .. } = self {
			*host = Some(new_host.into());
		}
	}
}

impl Builder {
	pub(super) fn push(&mut self, codepoint: CodePoint<char>) {
		self.temporary_buffer.push(codepoint.unit());
	}

	pub(super) fn build(self) -> Result<IrcMessagePrefix, IrcMessageError> {
		match self.state {
			| BuilderState::Initial => {
				// NOTE(phisyx): cas spécifique. Si on ne le fait pas, on tombe
				// dans le cas de Self::User.
				if self.temporary_buffer == "localhost" {
					return Ok(IrcMessagePrefix::Server {
						origin: self.temporary_buffer,
					});
				}

				Ok(IrcMessagePrefix::User {
					nick: nick::parse(&self.temporary_buffer)?,
					user: None,
					host: None,
				})
			}

			| BuilderState::User => {
				let mut prefix_user = IrcMessagePrefix::User {
					nick: Default::default(),
					user: None,
					host: None,
				};

				if let Some((raw_nick_user, host)) = self
					.temporary_buffer
					.split_once('@')
					.map(|(l, r)| (l, host::parse(r)))
				{
					if let Some((nick, user)) = raw_nick_user
						.split_once('!')
						.map(|(l, r)| (nick::parse(l), user::parse(r)))
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

				Ok(prefix_user)
			}

			| BuilderState::Server => Ok(IrcMessagePrefix::Server {
				origin: self.temporary_buffer,
			}),
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl ParseState for BuilderState {
	fn switch(&mut self, new_state: Self) {
		*self = new_state;
	}
}

// ---- //
// Test //
// ---- //

#[cfg(test)]
mod tests {
	use super::*;

	fn parse(source: &str) -> Result<IrcMessagePrefix, IrcMessageError> {
		let mut input = InputStream::new(source.chars());
		IrcMessagePrefix::parse(&mut input)
	}

	#[test]
	fn test_user() {
		let output = parse(":nick!user@host ");
		assert_eq!(
			output,
			Ok(IrcMessagePrefix::User {
				nick: "nick".to_owned(),
				user: "user".to_owned().into(),
				host: "host".to_owned().into()
			})
		);
	}

	#[test]
	fn test_server() {
		let output = parse(":localhost ");
		assert_eq!(
			output,
			Ok(IrcMessagePrefix::Server {
				origin: "localhost".to_owned()
			})
		);
	}
}
