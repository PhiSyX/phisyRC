/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod host;
mod nick;
mod user;

use core::fmt;
use std::str::{Chars, FromStr};

use lang::{codepoints::CodePoint, lexer::ParseState, stream::prelude::*};

pub(super) use self::{
	host::IrcMessagePrefixHostError, nick::IrcMessagePrefixNickError,
	user::IrcMessagePrefixUserError,
};

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
		user: String,
		host: String,
	},

	Server {
		origin: String,
	},
}

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum IrcMessagePrefixError {
	InputStream,
	ParseError,

	InvalidCharacter { found: char, help: &'static str },

	InvalidNick(IrcMessagePrefixNickError),
	InvalidUser(IrcMessagePrefixUserError),
	InvalidHost(IrcMessagePrefixHostError),
	InvalidOrigin(IrcMessagePrefixHostError),
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
	) -> Result<Self, IrcMessagePrefixError> {
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
					// Il s'agit d'une erreur d'analyse.
					| codepoint if codepoint.is_whitespace() => {
						return Err(IrcMessagePrefixError::InvalidCharacter {
							found: codepoint.unit(),
							help: "Un point de code valide est attendu",
						})
					}

					// U+0021 EXCLAMATION MARK (!)
					// U+0040 COMMERCIAL AT (@)
					//
					// Passer à l'état [IrcMessagePrefixState::User].
					| codepoint @ (CodePoint::EXCLAMATION_MARK
					| CodePoint::COMMERCIAL_AT)
						if !builder.is_empty() =>
					{
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

					| CodePoint::EOF if cfg!(test) => break,

					// Tous autres caractères.
					//
					// Il s'agit d'une erreur d'analyse.
					| _ => return Err(IrcMessagePrefixError::ParseError),
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
					| _ => return Err(IrcMessagePrefixError::ParseError),
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
					| _ => return Err(IrcMessagePrefixError::ParseError),
				},
			}
		}

		builder.build()
	}
}

impl IrcMessagePrefix {
	fn check_fields(&self) -> Result<(), IrcMessagePrefixError> {
		match self {
			| IrcMessagePrefix::User { nick, user, host } => {
				if nick.is_empty() {
					return Err(IrcMessagePrefixError::InvalidNick(
						IrcMessagePrefixNickError::IsEmpty,
					));
				}
				if user.is_empty() {
					return Err(IrcMessagePrefixError::InvalidUser(
						IrcMessagePrefixUserError::IsEmpty,
					));
				}

				if host.is_empty() {
					return Err(IrcMessagePrefixError::InvalidHost(
						IrcMessagePrefixHostError::IsEmpty,
					));
				}

				Ok(())
			}
			| IrcMessagePrefix::Server { origin } => {
				if origin.is_empty() {
					return Err(IrcMessagePrefixError::InvalidOrigin(
						IrcMessagePrefixHostError::IsEmpty,
					));
				}
				Ok(host::parse(origin).map(|_| ())?)
			}
		}
	}

	fn set_nick(&mut self, new_nick: impl Into<String>) {
		if let Self::User { nick, .. } = self {
			*nick = new_nick.into();
		}
	}

	fn set_user(&mut self, new_user: impl Into<String>) {
		if let Self::User { user, .. } = self {
			*user = new_user.into();
		}
	}

	fn set_host(&mut self, new_host: impl Into<String>) {
		if let Self::User { host, .. } = self {
			*host = new_host.into();
		}
	}
}

impl Builder {
	fn is_empty(&self) -> bool {
		self.temporary_buffer.is_empty()
	}

	fn push(&mut self, codepoint: CodePoint<char>) {
		self.temporary_buffer.push(codepoint.unit());
	}

	pub(super) fn build(
		self,
	) -> Result<IrcMessagePrefix, IrcMessagePrefixError> {
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
					user: Default::default(),
					host: Default::default(),
				})
			}

			| BuilderState::User => {
				let mut prefix_user = IrcMessagePrefix::User {
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

			| BuilderState::Server => {
				let prefix_server = IrcMessagePrefix::Server {
					origin: self.temporary_buffer,
				};
				prefix_server.check_fields()?;
				Ok(prefix_server)
			}
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

impl From<InputStreamError> for IrcMessagePrefixError {
	fn from(_: InputStreamError) -> Self {
		Self::InputStream
	}
}

impl From<IrcMessagePrefixNickError> for IrcMessagePrefixError {
	fn from(err: IrcMessagePrefixNickError) -> Self {
		Self::InvalidNick(err)
	}
}

impl From<IrcMessagePrefixUserError> for IrcMessagePrefixError {
	fn from(err: IrcMessagePrefixUserError) -> Self {
		Self::InvalidUser(err)
	}
}

impl From<IrcMessagePrefixHostError> for IrcMessagePrefixError {
	fn from(err: IrcMessagePrefixHostError) -> Self {
		Self::InvalidHost(err)
	}
}

impl fmt::Display for IrcMessagePrefixError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::InputStream =>
					"erreur dans le flux d'entrée".to_owned(),
				| Self::ParseError => "erreur d'analyse".to_owned(),
				| Self::InvalidCharacter { found, .. } => format!(
					"le caractère {found} est invalide pour un préfixe."
				),
				| Self::InvalidNick(err) =>
					format!("pseudonyme invalide: {err}"),
				| Self::InvalidUser(err) => format!("identité invalide: {err}"),
				| Self::InvalidHost(err) =>
					format!("nom d'hôte invalide: {err}"),
				| Self::InvalidOrigin(err) =>
					format!("origine invalide: {err}"),
			}
		)
	}
}

impl FromStr for IrcMessagePrefixError {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.ends_with("pseudonyme manquant") {
			return Ok(Self::InvalidNick(IrcMessagePrefixNickError::IsEmpty));
		} else if s.ends_with("nom d'hôte manquant") {
			return Ok(Self::InvalidHost(IrcMessagePrefixHostError::IsEmpty));
		} else if s.ends_with("origine invalide") {
			return Ok(Self::InvalidOrigin(IrcMessagePrefixHostError::IsEmpty));
		} else if s.ends_with("1er caractère invalide") {
			return Ok(Self::InvalidHost(
				IrcMessagePrefixHostError::InvalidFirstCharacter,
			));
		} else if s.ends_with("dernier caractère invalide") {
			return Ok(Self::InvalidHost(
				IrcMessagePrefixHostError::InvalidLastCharacter,
			));
		} else if s.contains("caractère invalide -> ") {
			let x = unsafe {
				s.split_once(" -> ")
					.map(|(_, x)| {
						x.replace("\\s", " ")
							.replace("\\r", "\r")
							.replace("\\n", "\n")
					})
					.unwrap_unchecked()
			};
			let x = x.as_bytes();
			return Ok(Self::InvalidCharacter {
				found: x[0] as char,
				help: "Un point de code valide est attendu",
			});
		}

		Err("Cas non géré")
	}
}

// ---- //
// Test //
// ---- //

#[cfg(test)]
mod tests {
	use super::*;

	fn parse(source: &str) -> Result<IrcMessagePrefix, IrcMessagePrefixError> {
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
				user: "user".to_owned(),
				host: "host".to_owned()
			})
		);
	}

	#[test]
	fn test_server() {
		let output = parse(":localhost");
		assert_eq!(
			output,
			Ok(IrcMessagePrefix::Server {
				origin: "localhost".to_owned()
			})
		);
	}
}
