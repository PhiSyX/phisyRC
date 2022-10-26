/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub mod command;
pub mod prefix;
pub mod tags;

use core::fmt;
use std::str::Chars;

use lang::{
	codepoints::CodePoint,
	stream::{ByteStream, InputStream, InputStreamError, StreamIterator},
};

use self::{command::Command, prefix::Prefix, tags::Tags};

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(serde::Serialize)]
pub struct Message {
	pub raw: Option<String>,
	pub tags: Tags,
	pub prefix: Option<Prefix>,
	pub command: Command,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum Error {
	InputStream,
	IsEmpty,
	InvalidTags(tags::Error),
	InvalidPrefix(prefix::Error),
	InvalidCommand(command::Error),
}

// -------------- //
// Implémentation //
// -------------- //

impl Message {
	/// Analyse d'un message IRC.
	//
	// BNF <message>    ::= [ "@" tags ] [ ":" prefix SPACE ] command
	//                  ::= [ params ] crlf
	//
	//     <prefix>     ::= servername / ( nickname [ [ "!" user ] "@" host ] )
	//     <command>    ::= 1*letter / 3digit
	//     <params>     ::= *14( SPACE middle ) [ SPACE ":" trailing ]
	//                    = / 14( SPACE middle ) [ SPACE [ ":" ] trailing ]
	//     <middle>     ::= nospcrlfcl *( ":" / nospcrlfcl )
	//     <nospcrlfcl> ::=  %x01-09 / %x0B-0C / %x0E-1F / %x21-39 / %x3B-FF
	//                    ; any octet except NUL, CR, LF, " " and ":"
	//     <trailing    ::= *( ":" / " " / nospcrlfcl )
	//     <SPACE>      ::= %x20 ; space character
	//     <crlf>       ::= %x0D %x0A ; "carriage return" "linefeed"
	//
	// NOTE(phisyx): crlf n'est pas inclus dans notre analyse.
	pub fn parse(
		mut input: InputStream<Chars<'_>, char>,
	) -> Result<Self, Error> {
		// NOTE(phisyx): analyse des `<tags>` ; cette partie n'est pas
		// obligatoire.
		let tags = if let CodePoint::COMMERCIAL_AT = input.peek_next()? {
			Tags::parse(&mut input)?
		} else {
			Tags::default()
		};

		// NOTE(phisyx): analyse du `<prefix>` ; cette partie n'est
		// pas obligatoire.
		let prefix = if let CodePoint::COLON = input.peek_next()? {
			Some(Prefix::parse(&mut input)?)
		} else {
			None
		};

		// NOTE(phisyx): la `<command>` est obligatoire. Le résultat de
		// l'analyse suivante inclus les paramètres, s'il y en a.
		let command = Command::parse(&mut input)?;

		Ok(Self {
			raw: None,
			tags,
			prefix,
			command,
		})
	}

	pub fn parse_from(
		raw: impl Into<ByteStream> + ToString,
	) -> Result<Self, Error> {
		let raw_input = raw.to_string();
		let bytestream = raw.into();
		let inputstream = InputStream::new(bytestream.chars());
		Self::parse(inputstream).map(|m| m.define_raw_msg(raw_input))
	}

	pub fn define_raw_msg(mut self, raw: String) -> Self {
		self.raw.replace(raw);
		self
	}

	#[cfg(feature = "json")]
	pub fn json(&self) -> serde_json::Value {
		serde_json::json!(self)
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

impl From<tags::Error> for Error {
	fn from(err: tags::Error) -> Self {
		Self::InvalidTags(err)
	}
}

impl From<prefix::Error> for Error {
	fn from(err: prefix::Error) -> Self {
		Self::InvalidPrefix(err)
	}
}

impl From<command::Error> for Error {
	fn from(err: command::Error) -> Self {
		Self::InvalidCommand(err)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::IsEmpty => "le flux est vide".to_owned(),
				| Self::InvalidTags(reason) => reason.to_string(),
				| Self::InvalidPrefix(reason) => reason.to_string(),
				| Self::InvalidCommand(reason) => reason.to_string(),
				| Self::InputStream => "erreur d'analyse".to_owned(),
			}
		)
	}
}

impl fmt::Display for Message {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.raw.clone().unwrap_or_default())
	}
}
