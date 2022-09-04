/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;

use lang::{codepoints::CodePoint, stream::prelude::*};

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum IrcMessagePrefixUserError {
	InputStream,

	IsEmpty,
	InvalidCharacter,
}

// ------ //
// Parser //
// ------ //

/// Analyse d'un identifiant d'un préfixe.
//
// BNF <user> ::= 1*( %x01-09 / %x0B-0C / %x0E-1F / %x21-3F / %x41-FF )
//				; any octet except NUL, CR, LF, " " and "@"
pub(super) fn parse(input: &str) -> Result<String, IrcMessagePrefixUserError> {
	let bytes: ByteStream = ByteStream::from(input);
	let mut stream = InputStream::new(bytes.chars());

	let mut user = String::new();

	if input.is_empty() {
		return Err(IrcMessagePrefixUserError::IsEmpty);
	}

	loop {
		match stream.consume_next()? {
			// Saut de ligne
			//
			// Il s'agit d'une erreur d'analyse.
			| codepoint if codepoint.is_newline() => {
				return Err(IrcMessagePrefixUserError::InvalidCharacter);
			}

			// U+0040 COMMERCIAL AT (@)
			//
			// Il s'agit d'une erreur d'analyse.
			| CodePoint::COMMERCIAL_AT => {
				return Err(IrcMessagePrefixUserError::InvalidCharacter);
			}

			// Tous les points de code valides
			//
			// Ajouter le point de code au nom d'utilisateur.
			| codepoint if codepoint.is_valid() => {
				user.push(codepoint.unit());
			}

			| CodePoint::EOF => break,

			// Tous les autres points de code.
			//
			// Il s'agit d'une erreur d'analyse.
			| _ => return Err(IrcMessagePrefixUserError::InvalidCharacter),
		}
	}

	Ok(user)
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<InputStreamError> for IrcMessagePrefixUserError {
	fn from(_: InputStreamError) -> Self {
		Self::InputStream
	}
}

impl fmt::Display for IrcMessagePrefixUserError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::InputStream => "erreur d'analyse",
				| Self::IsEmpty => "l'identifiant est vide",
				| Self::InvalidCharacter =>
					"l'identifiant contient un caractère invalide",
			}
		)
	}
}
