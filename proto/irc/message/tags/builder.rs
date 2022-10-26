/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::{collections::HashMap, str::Chars};

use lang::{
	codepoints::CodePoint,
	lexer::ParseState,
	stream::{InputStream, StreamIterator},
};

use super::state::ParseTagsState;
use crate::{MessageTags, MessageTagsError};

// --------- //
// Structure //
// --------- //

pub(super) struct ParseTagsBuilder<'a, 'b> {
	stream: &'a mut InputStream<Chars<'b>, char>,
	state: ParseTagsState,
	temporary_key: String,
	temporary_value: String,
	temporary_map: HashMap<String, String>,
}

// -------------- //
// Implémentation //
// -------------- //

impl<'a, 'b> ParseTagsBuilder<'a, 'b> {
	pub(super) fn initial(
		stream: &'a mut InputStream<Chars<'b>, char>,
	) -> Self {
		Self {
			stream,
			state: Default::default(),
			temporary_key: Default::default(),
			temporary_value: Default::default(),
			temporary_map: Default::default(),
		}
	}
}

impl<'a, 'b> ParseTagsBuilder<'a, 'b> {
	pub(super) fn analyze(&mut self) -> Result<(), MessageTagsError> {
		loop {
			match self.state {
				| ParseTagsState::Initial => {
					match self.stream.consume_next()? {
						| CodePoint::COMMERCIAL_AT => {
							self.temporary_key.clear();
							self.state.switch(ParseTagsState::LeftKey);
						}

						| _ => return Err(
							MessageTagsError::IsNotStartingWithCommercialChar,
						),
					}
				}

				| ParseTagsState::LeftKey => {
					match self.stream.consume_next()? {
						| codepoint if codepoint.is_newline() => {
							return Err(MessageTagsError::InvalidCharacter {
								found: codepoint.unit(),
								help: "Un point de code valide est attendu",
							});
						}

						| CodePoint::EOF if cfg!(test) => {
							if self.temporary_value.is_empty() {
								self.temporary_map.insert(
									self.temporary_key.to_owned(),
									true.to_string(),
								);
							} else {
								self.temporary_map.insert(
									self.temporary_key.to_owned(),
									self.temporary_value.to_owned(),
								);
							}
							break;
						}

						| CodePoint::EOF => {
							return Err(MessageTagsError::ParseError);
						}

						// exemple: "[?]="
						| CodePoint::EQUALS_SIGN => {
							// exemple: "?="
							if self.temporary_key.is_empty() {
								return Err(MessageTagsError::KeyIsEmpty);
							}
							// exemple: "*="
							self.state.switch(ParseTagsState::RightValue);
						}

						| CodePoint::Whitespace(_) => {
							self.temporary_map.insert(
								self.temporary_key.to_owned(),
								true.to_string(),
							);
							break;
						}

						| codepoint if codepoint.is_valid() => {
							self.add_character_to_current_key(codepoint.unit());
						}

						| _ => return Err(MessageTagsError::ParseError),
					}
				}

				| ParseTagsState::RightValue => {
					match self.stream.consume_next()? {
						| CodePoint::SEMICOLON => {
							self.temporary_map.insert(
								self.temporary_key.to_owned(),
								self.temporary_value.to_owned(),
							);
							self.temporary_key.clear();
							self.temporary_value.clear();
							self.state.switch(ParseTagsState::LeftKey);
						}

						| CodePoint::REVERSE_SOLIDUS => {
							match self.stream.peek_next()? {
								| CodePoint::COLON => {
									self.stream.consume_next()?;
									self.add_codepoint_to_current_value(
										CodePoint::SEMICOLON,
									);
								}

								| CodePoint::REVERSE_SOLIDUS => {
									self.stream.consume_next()?;
									self.add_codepoint_to_current_value(
										CodePoint::REVERSE_SOLIDUS,
									);
								}

								| CodePoint::Unit('s') => {
									self.stream.consume_next()?;
									self.add_codepoint_to_current_value(
										CodePoint::Unit(' '),
									);
								}

								| CodePoint::Unit('r') => {
									self.stream.consume_next()?;
									self.add_codepoint_to_current_value(
										CodePoint::Unit('\r'),
									);
								}

								| CodePoint::Unit('n') => {
									self.stream.consume_next()?;
									self.add_codepoint_to_current_value(
										CodePoint::Unit('\n'),
									);
								}

								| _ => self.add_codepoint_to_current_value(
									CodePoint::REVERSE_SOLIDUS,
								),
							}
						}

						| codepoint if codepoint.is_whitespace() => {
							if self.temporary_value.is_empty() {
								return Err(MessageTagsError::ValueIsEmpty);
							}

							self.temporary_map.insert(
								self.temporary_key.to_owned(),
								self.temporary_value.to_owned(),
							);
							break;
						}

						// Pour le test
						| CodePoint::EOF if cfg!(test) => {
							if self.temporary_value.is_empty() {
								return Err(MessageTagsError::ValueIsEmpty);
							}

							self.temporary_map.insert(
								self.temporary_key.to_owned(),
								self.temporary_value.to_owned(),
							);
							break;
						}

						| codepoint if codepoint.is_valid() => {
							self.add_character_to_current_value(
								codepoint.unit(),
							);
						}

						| _ => return Err(MessageTagsError::ParseError),
					}
				}
			}
		}

		Ok(())
	}

	fn add_character_to_current_key(&mut self, ch: char) {
		self.temporary_key.push(ch);
	}

	fn add_codepoint_to_current_value(&mut self, codepoint: CodePoint<char>) {
		self.add_character_to_current_value(codepoint.unit())
	}

	fn add_character_to_current_value(&mut self, ch: char) {
		self.temporary_value.push(ch);
	}
}

impl<'a, 'b> ParseTagsBuilder<'a, 'b> {
	pub(super) fn finish(self) -> Result<MessageTags, MessageTagsError> {
		Ok(MessageTags(self.temporary_map))
	}
}
