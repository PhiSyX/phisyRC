/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;
use std::{collections::HashMap, str::Chars};

use lang::{codepoints::CodePoint, lexer::ParseState, stream::prelude::*};

// --------- //
// Structure //
// --------- //

pub(super) struct IrcMessageTags;

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
pub(super) enum IrcMessageTagsError {
	InputStream,

	IsNotStartingWithCommercialChar,

	InvalidCharacter {
		expect: &'static str,
		found: &'static str,
	},

	KeyIsEmpty,
	ValueIsEmpty,
}

// -------------- //
// Implémentation //
// -------------- //

impl IrcMessageTags {
	pub(super) fn parse(
		stream: &mut InputStream<Chars<'_>, char>,
	) -> Result<HashMap<String, String>, IrcMessageTagsError> {
		enum State {
			Initial,
			Key { key: String },
			Value { key: String, value: String },
		}

		impl State {
			fn push_key(&mut self, codepoint: CodePoint<char>) {
				if let Self::Key { key } = self {
					key.push(codepoint.unit());
				}
			}

			fn push_value(&mut self, codepoint: CodePoint<char>) {
				if let Self::Value { value, .. } = self {
					value.push(codepoint.unit());
				}
			}
		}

		impl ParseState for State {
			fn switch(&mut self, new_state: Self) {
				*self = new_state;
			}
		}

		let mut map = HashMap::new();
		let mut state = State::Initial;

		loop {
			match state {
				| State::Initial => match stream.consume_next()? {
					| CodePoint::COMMERCIAL_AT => {
						state.switch(State::Key { key: String::new() });
					}

					| _ => return Err(
						IrcMessageTagsError::IsNotStartingWithCommercialChar,
					),
				},

				| State::Key { ref key } => match stream.consume_next()? {
					| CodePoint::EQUALS_SIGN if !key.is_empty() => state
						.switch(State::Value {
							key: key.to_owned(),
							value: String::new(),
						}),

					| CodePoint::EQUALS_SIGN => {
						return Err(IrcMessageTagsError::InvalidCharacter {
							expect: "<any codepoint except '='>",
							found: "=",
						});
					}

					| codepoint if codepoint.is_whitespace() => {
						if key.is_empty() {
							return Err(IrcMessageTagsError::KeyIsEmpty);
						}

						map.insert(key.to_owned(), Default::default());
						break;
					}

					| codepoint if codepoint.is_valid() => {
						state.push_key(codepoint);
					}

					// Pour le test
					| CodePoint::EOF => {
						if key.is_empty() {
							return Err(IrcMessageTagsError::KeyIsEmpty);
						}

						map.insert(key.to_owned(), Default::default());
						break;
					}

					| _ => return Err(IrcMessageTagsError::InvalidCharacter {
						expect:
							"<any code point except NUL, CR, LF, SPACE, ';'>",
						found: "<invalid code point>",
					}),
				},

				| State::Value { ref key, ref value } => match stream
					.consume_next()?
				{
					| CodePoint::SEMICOLON => {
						map.insert(key.to_owned(), value.to_owned());
						state.switch(State::Key { key: String::new() });
					}

					| CodePoint::REVERSE_SOLIDUS => {
						match stream.peek_next()? {
							| CodePoint::COLON => {
								stream.consume_next()?;
								state.push_value(CodePoint::SEMICOLON);
							}
							| CodePoint::REVERSE_SOLIDUS => {
								stream.consume_next()?;
								state.push_value(CodePoint::REVERSE_SOLIDUS);
							}

							| CodePoint::Unit('s') => {
								stream.consume_next()?;
								state.push_value(CodePoint::Unit(' '));
							}

							| CodePoint::Unit('r') => {
								stream.consume_next()?;
								state.push_value(CodePoint::Unit('\r'));
							}

							| CodePoint::Unit('n') => {
								stream.consume_next()?;
								state.push_value(CodePoint::Unit('\n'));
							}

							| _ => state.push_value(CodePoint::REVERSE_SOLIDUS),
						}
					}

					| codepoint if codepoint.is_whitespace() => {
						if value.is_empty() {
							return Err(IrcMessageTagsError::ValueIsEmpty);
						}

						map.insert(key.to_owned(), value.to_owned());
						break;
					}

					// Pour le test
					| CodePoint::EOF => {
						if value.is_empty() {
							return Err(IrcMessageTagsError::ValueIsEmpty);
						}

						map.insert(key.to_owned(), value.to_owned());
						break;
					}

					| codepoint if codepoint.is_valid() => {
						state.push_value(codepoint);
					}

					| _ => return Err(IrcMessageTagsError::InvalidCharacter {
						expect:
							"<any code point except NUL, CR, LF, SPACE, ';'>",
						found: "<invalid code point>",
					}),
				},
			}
		}

		Ok(map)
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<InputStreamError> for IrcMessageTagsError {
	fn from(_: InputStreamError) -> Self {
		Self::InputStream
	}
}

impl fmt::Display for IrcMessageTagsError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::InputStream => "erreur d'analyse".to_owned(),
				| Self::InvalidCharacter { expect, found } => format!(
					"caractère invalide : attendu {expect}, mais trouvé {found}"
				),
				| Self::IsNotStartingWithCommercialChar =>
					"ne commence pas par un caractère commercial (@)".to_owned(),
				| Self::KeyIsEmpty => "le nom de la clé est vide".to_owned(),
				| Self::ValueIsEmpty => "la valeur est vide".to_owned(),
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

	fn parse(
		input: &str,
	) -> Result<HashMap<String, String>, IrcMessageTagsError> {
		let mut stream = InputStream::new(input.chars());
		IrcMessageTags::parse(&mut stream)
	}

	#[test]
	fn test_tags_ok() {
		let input = "@admin";
		let output = parse(input).unwrap();
		assert_eq!(
			output,
			HashMap::from_iter([("admin".to_owned(), "".to_owned())])
		);

		let input = "@id=1;first-name=Mike";
		let output = parse(input).unwrap();
		assert_eq!(
			output,
			HashMap::from_iter([
				("id".to_owned(), "1".to_owned()),
				("first-name".to_owned(), "Mike".to_owned()),
			])
		);

		let input = "@example.org/foo=bar";
		let output = parse(input).unwrap();
		assert_eq!(
			output,
			HashMap::from_iter([(
				"example.org/foo".to_owned(),
				"bar".to_owned()
			)])
		);

		let input = "@+icon=https://example.com/favicon.png";
		let output = parse(input).unwrap();
		assert_eq!(
			output,
			HashMap::from_iter([(
				"+icon".to_owned(),
				"https://example.com/favicon.png".to_owned()
			)])
		);

		let input =
			"@time=2021-01-27T18:09:19.337Z;msgid=SiCzdPygaGoToMz8Jg9gLS";
		let output = parse(input).unwrap();
		assert_eq!(
			output,
			HashMap::from_iter([
				("time".to_owned(), "2021-01-27T18:09:19.337Z".to_owned()),
				("msgid".to_owned(), "SiCzdPygaGoToMz8Jg9gLS".to_owned()),
			])
		);
	}

	#[test]
	fn test_tags_error() {
		let input = "@=";
		let output = parse(input);
		assert!(output.is_err());

		let input = "@a=";
		let output = parse(input);
		assert!(output.is_err());

		let input = "@a=b;c=d; ";
		let output = parse(input);
		assert!(output.is_err());
	}
}
