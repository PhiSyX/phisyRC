/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::fmt;

use lang::{
	codepoints::CodePoint,
	lexer::ParseState,
	stream::{ByteStream, InputStream, InputStreamError, StreamIterator},
};

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum MessagePrefixNickError {
	InputStream,

	IsEmpty,
	InvalidFirstCharacter,
	InvalidCharacter,
}

// ------- //
// Parsing //
// ------- //

/// Analyse d'un pseudonyme.
//
// BNF: <nick>   ::= ( letter / special ) *8( letter / digit / special / "-" )
//      <letter>  ::= %x41-5A / %x61-7A        ; A-Z / a-z
//      <number>  ::= %x30-39                  ; 0-9
//      <special> ::= %x5B-60 / %x7B-7D        ; "[", "]", "\", "`", "_", "^",
//                                               "{", "|", "}"
pub(super) fn parse(input: &str) -> Result<String, MessagePrefixNickError> {
	let bytes: ByteStream = ByteStream::from(input);
	let mut stream = InputStream::new(bytes.chars());

	#[derive(Debug)]
	enum State {
		Initial,

		First,
		AfterFirst,
	}

	impl ParseState for State {
		fn switch(&mut self, new_state: Self) {
			*self = new_state;
		}
	}

	let mut state = State::Initial;

	let mut nick = String::new();

	loop {
		match state {
			| State::Initial => {
				if input.is_empty() {
					return Err(MessagePrefixNickError::IsEmpty);
				}

				state.switch(State::First);
			}

			| State::First => match stream.consume_next()? {
				// Point de code alphabétique.
				//
				// Ajouter le caractère au pseudonyme.
				// Passer à l'état [IrcPrefixNickState::AfterFirst].
				| CodePoint::Unit(ch) if ch.is_alphabetic() => {
					nick.push(ch);
					state.switch(State::AfterFirst);
				}

				// U+005B LEFT SQUARE BRACKET ([)
				// U+005D RIGHT SQUARE BRACKET (])
				// U+005C REVERSE SOLIDUS (\)
				// U+0060 GRAVE ACCENT (`)
				// U+005F LOW LINE (_)
				// U+005E CIRCUMFLEX ACCENT (^)
				// U+007B LEFT CURLY BRACKET ({)
				// U+007D RIGHT CURLY BRACKET (})
				//
				// Ajouter le caractère au pseudonyme.
				// Passer à l'état [IrcPrefixNickState::AfterFirst].
				| codepoint @ (CodePoint::LEFT_SQUARE_BRACKET
				| CodePoint::RIGHT_SQUARE_BRACKET
				| CodePoint::REVERSE_SOLIDUS
				| CodePoint::Unit('`')
				| CodePoint::Unit('_')
				| CodePoint::CIRCUMFLEX_ACCENT
				| CodePoint::LEFT_CURLY_BRACKET
				| CodePoint::RIGHT_CURLY_BRACKET) => {
					nick.push(codepoint.unit());
					state.switch(State::AfterFirst);
				}

				// Tous les autres points de code.
				//
				// Il s'agit d'une erreur d'analyse.
				| _ => {
					return Err(MessagePrefixNickError::InvalidFirstCharacter)
				}
			},

			| State::AfterFirst => match stream.consume_next()? {
				// Point de code alphanumérique.
				//
				// Ajouter le point de code au pseudonyme.
				| CodePoint::Unit(ch) if ch.is_alphanumeric() => {
					nick.push(ch);
				}

				// U+005B LEFT SQUARE BRACKET ([)
				// U+005D RIGHT SQUARE BRACKET (])
				// U+005C REVERSE SOLIDUS (\)
				// U+0060 GRAVE ACCENT (`)
				// U+005F LOW LINE (_)
				// U+005E CIRCUMFLEX ACCENT (^)
				// U+007B LEFT CURLY BRACKET ({)
				// U+007D RIGHT CURLY BRACKET (})
				// U+002D HYPHEN-MINUS (-)
				//
				// Ajouter le point de code au pseudonyme.
				| codepoint @ (CodePoint::LEFT_SQUARE_BRACKET
				| CodePoint::RIGHT_SQUARE_BRACKET
				| CodePoint::REVERSE_SOLIDUS
				| CodePoint::Unit('`')
				| CodePoint::Unit('_')
				| CodePoint::CIRCUMFLEX_ACCENT
				| CodePoint::LEFT_CURLY_BRACKET
				| CodePoint::RIGHT_CURLY_BRACKET
				| CodePoint::HYPHEN_MINUS) => {
					nick.push(codepoint.unit());
				}

				| CodePoint::EOF => break,

				// Tous les autres points de code.
				//
				// Il s'agit d'une erreur d'analyse.
				| _ => return Err(MessagePrefixNickError::InvalidCharacter),
			},
		}
	}

	Ok(nick)
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<InputStreamError> for MessagePrefixNickError {
	fn from(_: InputStreamError) -> Self {
		Self::InputStream
	}
}

impl fmt::Display for MessagePrefixNickError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::InputStream => "erreur d'analyse",
				| Self::IsEmpty => "le pseudonyme est vide",
				| Self::InvalidFirstCharacter =>
					"le pseudonyme commence par un caractère invalide",
				| Self::InvalidCharacter =>
					"le pseudonyme contient un caractère invalide",
			}
		)
	}
}

// ---- //
// Test //
// ---- //

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_nick_empty() {
		assert_eq!(parse(""), Err(MessagePrefixNickError::IsEmpty));
	}

	#[test]
	fn test_nick_ok() {
		let input = "PhiSyX";
		let output = parse(input).unwrap();
		assert_eq!(output, input);

		let input = "PhiSyX-";
		let output = parse(input).unwrap();
		assert_eq!(output, input);

		let input = "[PhiSyX-]_";
		let output = parse(input).unwrap();
		assert_eq!(output, input);
	}

	#[test]
	fn test_nick_invalid_first_character() {
		let input = "1PhiSyX";
		let output = parse(input);
		assert_eq!(output, Err(MessagePrefixNickError::InvalidFirstCharacter));

		let input = "-PhiSyX";
		let output = parse(input);
		assert_eq!(output, Err(MessagePrefixNickError::InvalidFirstCharacter));
	}

	#[test]
	fn test_nick_invalid_character() {
		let input = "Phi@SyX";
		let output = parse(input);
		assert_eq!(output, Err(MessagePrefixNickError::InvalidCharacter));
	}
}
