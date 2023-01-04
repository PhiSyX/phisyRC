/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod builder;
mod host;
mod nick;
mod state;
mod user;

use core::fmt;
use std::str::{Chars, FromStr};

use lang::stream::{ByteStream, InputStream, InputStreamError};

use self::builder::Builder;

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum Prefix {
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
pub enum Error {
	InputStream,
	Parse,

	InvalidCharacter { found: char, help: &'static str },

	InvalidNick(nick::Error),
	InvalidUser(user::Error),
	InvalidHost(host::Error),
	InvalidOrigin(host::Error),
}

// -------------- //
// Implémentation //
// -------------- //

impl Prefix {
	pub fn parse(
		stream: &mut InputStream<Chars<'_>, char>,
	) -> Result<Self, Error> {
		let mut builder = Builder::initialize(stream);
		builder.analyze()?;
		builder.finish()
	}

	pub fn parse_from_str(raw: impl Into<String>) -> Result<Self, Error> {
		let bytestream = ByteStream::new(raw);
		let mut inputstream = InputStream::new(bytestream.chars());
		Self::parse(&mut inputstream)
	}
}

impl Prefix {
	fn check_fields(&self) -> Result<(), Error> {
		match self {
			| Prefix::User { nick, user, host } => {
				if nick.is_empty() {
					return Err(Error::InvalidNick(nick::Error::IsEmpty));
				}
				if user.is_empty() {
					return Err(Error::InvalidUser(user::Error::IsEmpty));
				}

				if host.is_empty() {
					return Err(Error::InvalidHost(host::Error::IsEmpty));
				}

				Ok(())
			}
			| Prefix::Server { origin } => {
				if origin.is_empty() {
					return Err(Error::InvalidOrigin(host::Error::IsEmpty));
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

impl From<InputStreamError> for Error {
	fn from(_: InputStreamError) -> Self {
		Self::InputStream
	}
}

impl From<nick::Error> for Error {
	fn from(err: nick::Error) -> Self {
		Self::InvalidNick(err)
	}
}

impl From<user::Error> for Error {
	fn from(err: user::Error) -> Self {
		Self::InvalidUser(err)
	}
}

impl From<host::Error> for Error {
	fn from(err: host::Error) -> Self {
		Self::InvalidHost(err)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::InputStream =>
					"erreur dans le flux d'entrée".to_owned(),
				| Self::Parse => "erreur d'analyse".to_owned(),
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

impl FromStr for Error {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.ends_with("pseudonyme manquant") {
			return Ok(Self::InvalidNick(nick::Error::IsEmpty));
		} else if s.ends_with("nom d'hôte manquant") {
			return Ok(Self::InvalidHost(host::Error::IsEmpty));
		} else if s.ends_with("origine invalide") {
			return Ok(Self::InvalidOrigin(host::Error::IsEmpty));
		} else if s.ends_with("1er caractère invalide") {
			return Ok(Self::InvalidHost(host::Error::InvalidFirstCharacter));
		} else if s.ends_with("dernier caractère invalide") {
			return Ok(Self::InvalidHost(host::Error::InvalidLastCharacter));
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

impl fmt::Display for Prefix {
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

		write!(f, "{output}")
	}
}

// ---- //
// Test //
// ---- //

#[cfg(test)]
mod tests {
	use super::*;

	fn parse(source: &str) -> Result<Prefix, Error> {
		let mut input = InputStream::new(source.chars());
		Prefix::parse(&mut input)
	}

	#[test]
	fn test_user() {
		let output = parse(":nick!user@host ");
		assert_eq!(
			output,
			Ok(Prefix::User {
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
			Ok(Prefix::Server {
				origin: "localhost".to_owned()
			})
		);
	}
}
