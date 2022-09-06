/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod builder;
mod host;
mod nick;
mod state;
mod user;

use core::fmt;
use std::str::{Chars, FromStr};

use lang::stream::prelude::*;

use self::builder::ParsePrefixBuilder;
pub(super) use self::{
	host::IrcMessagePrefixHostError, nick::IrcMessagePrefixNickError,
	user::IrcMessagePrefixUserError,
};

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

// -------------- //
// Implémentation //
// -------------- //

impl IrcMessagePrefix {
	pub fn parse(
		stream: &mut InputStream<Chars<'_>, char>,
	) -> Result<Self, IrcMessagePrefixError> {
		let mut builder = ParsePrefixBuilder::initialize(stream);
		builder.analyze()?;
		builder.finish()
	}

	pub fn parse_from_str(
		raw: impl Into<String>,
	) -> Result<Self, IrcMessagePrefixError> {
		let bytestream = ByteStream::new(raw);
		let mut inputstream = InputStream::new(bytestream.chars());
		Self::parse(&mut inputstream)
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

// -------------- //
// Implémentation // -> Interface
// -------------- //

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
					"le caractère « {found} » est invalide pour un préfixe."
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
