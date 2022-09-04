/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod codec;
mod command;
mod prefix;
mod tags;

use core::fmt;
use std::str::Chars;

use lang::{codepoints::CodePoint, stream::prelude::*};

pub use self::{codec::*, command::*, prefix::*, tags::*};

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
pub struct IrcMessage {
	pub tags: IrcMessageTags,
	pub prefix: Option<IrcMessagePrefix>,
	pub command: IrcMessageCommand,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum IrcMessageError {
	InputStream,
	IsEmpty,
	InvalidTags(IrcMessageTagsError),
	InvalidPrefix(IrcMessagePrefixError),
	InvalidCommand(IrcMessageCommandError),
}

// -------------- //
// Implémentation //
// -------------- //

impl IrcMessage {
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
	) -> Result<Self, IrcMessageError> {
		// NOTE(phisyx): analyse des `<tags>` ; cette partie n'est pas
		// obligatoire.
		let tags = if let CodePoint::COMMERCIAL_AT = input.peek_next()? {
			IrcMessageTags::parse(&mut input)?
		} else {
			IrcMessageTags::default()
		};

		// NOTE(phisyx): analyse du `<prefix>` ; cette partie n'est
		// pas obligatoire.
		let prefix = if let CodePoint::COLON = input.peek_next()? {
			Some(IrcMessagePrefix::parse(&mut input)?)
		} else {
			None
		};

		// NOTE(phisyx): la `<command>` est obligatoire. Le résultat de
		// l'analyse suivante inclus les paramètres, s'il y en a.
		let command = IrcMessageCommand::parse(&mut input)?;

		Ok(Self {
			tags,
			prefix,
			command,
		})
	}

	pub fn parse_from_str(
		raw: impl Into<String>,
	) -> Result<Self, IrcMessageError> {
		let bytestream = ByteStream::new(raw);
		let inputstream = InputStream::new(bytestream.chars());
		Self::parse(inputstream)
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<InputStreamError> for IrcMessageError {
	fn from(_: InputStreamError) -> Self {
		Self::InputStream
	}
}

impl From<IrcMessageTagsError> for IrcMessageError {
	fn from(err: IrcMessageTagsError) -> Self {
		Self::InvalidTags(err)
	}
}

impl From<IrcMessagePrefixError> for IrcMessageError {
	fn from(err: IrcMessagePrefixError) -> Self {
		Self::InvalidPrefix(err)
	}
}

impl From<IrcMessageCommandError> for IrcMessageError {
	fn from(err: IrcMessageCommandError) -> Self {
		Self::InvalidCommand(err)
	}
}

impl fmt::Display for IrcMessageError {
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
