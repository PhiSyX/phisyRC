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

use lang::stream::{ByteStream, InputStream, InputStreamError};

use self::builder::ParsePrefixBuilder;
pub(super) use self::{
	host::MessagePrefixHostError, nick::MessagePrefixNickError,
	user::MessagePrefixUserError,
};

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum MessagePrefix {
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
pub enum MessagePrefixError {
	InputStream,
	ParseError,

	InvalidCharacter { found: char, help: &'static str },

	InvalidNick(MessagePrefixNickError),
	InvalidUser(MessagePrefixUserError),
	InvalidHost(MessagePrefixHostError),
	InvalidOrigin(MessagePrefixHostError),
}

// -------------- //
// Implémentation //
// -------------- //

impl MessagePrefix {
	pub fn parse(
		stream: &mut InputStream<Chars<'_>, char>,
	) -> Result<Self, MessagePrefixError> {
		let mut builder = ParsePrefixBuilder::initialize(stream);
		builder.analyze()?;
		builder.finish()
	}

	pub fn parse_from_str(
		raw: impl Into<String>,
	) -> Result<Self, MessagePrefixError> {
		let bytestream = ByteStream::new(raw);
		let mut inputstream = InputStream::new(bytestream.chars());
		Self::parse(&mut inputstream)
	}
}

impl MessagePrefix {
	fn check_fields(&self) -> Result<(), MessagePrefixError> {
		match self {
			| MessagePrefix::User { nick, user, host } => {
				if nick.is_empty() {
					return Err(MessagePrefixError::InvalidNick(
						MessagePrefixNickError::IsEmpty,
					));
				}
				if user.is_empty() {
					return Err(MessagePrefixError::InvalidUser(
						MessagePrefixUserError::IsEmpty,
					));
				}

				if host.is_empty() {
					return Err(MessagePrefixError::InvalidHost(
						MessagePrefixHostError::IsEmpty,
					));
				}

				Ok(())
			}
			| MessagePrefix::Server { origin } => {
				if origin.is_empty() {
					return Err(MessagePrefixError::InvalidOrigin(
						MessagePrefixHostError::IsEmpty,
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

impl From<InputStreamError> for MessagePrefixError {
	fn from(_: InputStreamError) -> Self {
		Self::InputStream
	}
}

impl From<MessagePrefixNickError> for MessagePrefixError {
	fn from(err: MessagePrefixNickError) -> Self {
		Self::InvalidNick(err)
	}
}

impl From<MessagePrefixUserError> for MessagePrefixError {
	fn from(err: MessagePrefixUserError) -> Self {
		Self::InvalidUser(err)
	}
}

impl From<MessagePrefixHostError> for MessagePrefixError {
	fn from(err: MessagePrefixHostError) -> Self {
		Self::InvalidHost(err)
	}
}

impl fmt::Display for MessagePrefixError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::InputStream =>
					"erreur dans le flux d'entrée".to_owned(),
				| Self::ParseError => "erreur d'analyse".to_owned(),
				| Self::InvalidCharacter { found, .. } => format!(
					"le caractère « {found:?} » est invalide pour un préfixe."
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

impl FromStr for MessagePrefixError {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.ends_with("pseudonyme manquant") {
			return Ok(Self::InvalidNick(MessagePrefixNickError::IsEmpty));
		} else if s.ends_with("nom d'hôte manquant") {
			return Ok(Self::InvalidHost(MessagePrefixHostError::IsEmpty));
		} else if s.ends_with("origine invalide") {
			return Ok(Self::InvalidOrigin(MessagePrefixHostError::IsEmpty));
		} else if s.ends_with("1er caractère invalide") {
			return Ok(Self::InvalidHost(
				MessagePrefixHostError::InvalidFirstCharacter,
			));
		} else if s.ends_with("dernier caractère invalide") {
			return Ok(Self::InvalidHost(
				MessagePrefixHostError::InvalidLastCharacter,
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

impl fmt::Display for MessagePrefix {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut output = String::new();

		match self {
			| Self::User { nick, user, host } => {
				output.push_str(nick);
				output.push('!');
				output.push_str(user);
				output.push('@');
				output.push_str(host);
			}
			| Self::Server { origin } => output.push_str(origin),
		};

		write!(f, "{}", output)
	}
}

// ---- //
// Test //
// ---- //

#[cfg(test)]
mod tests {
	use super::*;

	fn parse(source: &str) -> Result<MessagePrefix, MessagePrefixError> {
		let mut input = InputStream::new(source.chars());
		MessagePrefix::parse(&mut input)
	}

	#[test]
	fn test_user() {
		let output = parse(":nick!user@host ");
		assert_eq!(
			output,
			Ok(MessagePrefix::User {
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
			Ok(MessagePrefix::Server {
				origin: "localhost".to_owned()
			})
		);
	}
}
