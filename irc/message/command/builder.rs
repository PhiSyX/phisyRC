/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::str::Chars;

use lang::{codepoints::CodePoint, lexer::ParseState, stream::prelude::*};

use super::{
	parameters::IrcMessageCommandParameters,
	state::{
		ParseCommandParametersFirstStepState,
		ParseCommandParametersSecondStepState, ParseCommandState,
	},
};
use crate::{IrcMessageCommand, IrcMessageCommandError};

// --------- //
// Structure //
// --------- //

pub(super) struct ParseCommandBuilder<'a, 'b> {
	stream: &'a mut InputStream<Chars<'b>, char>,
	state: ParseCommandState,
	temporary_buffer: String,
}

pub(super) struct ParseCommandParametersBuilder<'a, 'b> {
	stream: &'a mut InputStream<Chars<'b>, char>,
	first_step_state: ParseCommandParametersFirstStepState,
	second_step_state: ParseCommandParametersSecondStepState,
	temporary_buffer: String,
	parameters_buffer: Vec<String>,
}

// -------------- //
// Implémentation //
// -------------- //

impl<'a, 'b> ParseCommandBuilder<'a, 'b> {
	pub(super) fn initialize(
		stream: &'a mut InputStream<Chars<'b>, char>,
	) -> Self {
		Self {
			stream,
			state: Default::default(),
			temporary_buffer: Default::default(),
		}
	}
}

impl<'a, 'b> ParseCommandBuilder<'a, 'b> {
	pub(super) fn analyze(&mut self) -> Result<(), IrcMessageCommandError> {
		loop {
			match self.state {
				| ParseCommandState::Initial => {
					match self.stream.consume_next()? {
						// Caractère de 0 à 9
						| CodePoint::Unit(ch) if ch.is_numeric() => {
							self.stream.reconsume_current();
							self.state.switch(ParseCommandState::Numeric {
								counter: 0,
							})
						}

						// Caractère alphabétique
						| CodePoint::Unit(ch) if ch.is_alphabetic() => {
							self.stream.reconsume_current();
							self.state.switch(ParseCommandState::Text)
						}

						| CodePoint::Unit(ch) => {
							return Err(
								IrcMessageCommandError::InvalidCharacter {
									found: ch,
									help: "Un caractère de commande valide est attendu."
								},
							);
						}

						| codepoint if codepoint.is_valid() => return Err(
							IrcMessageCommandError::InvalidCharacter {
								found: codepoint.unit(),
								help: "Un caractère de commande valide est attendu.",
							},
						),

						| _ => return Err(IrcMessageCommandError::ParseError),
					}
				}

				| ParseCommandState::Numeric { counter } => {
					match self.stream.consume_next()? {
						| _ if counter > 3 => {
							return Err(
								IrcMessageCommandError::NumericCodeIsTooLong,
							)
						}

						| CodePoint::Unit(ch) if ch.is_numeric() => {
							self.add_character_to_temporary_buffer(ch);
							self.state.increment_counter();
						}

						| CodePoint::Whitespace(_) => {
							self.stream.reconsume_current();
							break;
						}
						| CodePoint::Newline(_) => break,

						| codepoint if codepoint.is_valid() => {
							return Err(
								IrcMessageCommandError::InvalidCharacter {
									found: codepoint.unit(),
									help: "Un caractère de commande valide est attendu.",
								},
							)
						}

						| _ => return Err(IrcMessageCommandError::ParseError),
					}
				}

				| ParseCommandState::Text => {
					match self.stream.consume_next()? {
						| CodePoint::Unit(ch) if ch.is_alphanumeric() => {
							self.add_character_to_temporary_buffer(ch);
						}

						| CodePoint::Whitespace(_) => {
							self.stream.reconsume_current();
							break
						}
						| CodePoint::Newline(_) => break,

						| codepoint if codepoint.is_valid() => {
							return Err(
								IrcMessageCommandError::InvalidCharacter {
									found: codepoint.unit(),
									help: "Un caractère de commande valide est attendu.",
								},
							)
						}

						| _ => return Err(IrcMessageCommandError::ParseError),
					}
				}
			}
		}

		Ok(())
	}

	fn add_character_to_temporary_buffer(&mut self, ch: char) {
		self.temporary_buffer.push(ch);
	}
}

impl<'a, 'b> ParseCommandBuilder<'a, 'b> {
	pub(super) fn finish(
		self,
	) -> Result<IrcMessageCommand, IrcMessageCommandError> {
		assert!(self.state != ParseCommandState::Initial);

		match self.state {
			| ParseCommandState::Initial => {
				Err(IrcMessageCommandError::ParseError)
			}
			| ParseCommandState::Numeric { counter } => {
				// NOTE(phisyx): la condition > 3 est vérifié lors de l'analyse
				// plus-haut.
				if counter < 3 {
					return Err(IrcMessageCommandError::NumericCodeIsTooShort);
				}

				Ok(IrcMessageCommand::Numeric {
					code: self.temporary_buffer,
					parameters: IrcMessageCommandParameters::default(),
				})
			}
			| ParseCommandState::Text => Ok(IrcMessageCommand::Text {
				command: self.temporary_buffer,
				parameters: IrcMessageCommandParameters::default(),
			}),
		}
	}
}

impl<'a, 'b> ParseCommandParametersBuilder<'a, 'b> {
	pub(super) fn initialize(
		stream: &'a mut InputStream<Chars<'b>, char>,
	) -> Self {
		Self {
			stream,
			first_step_state: Default::default(),
			second_step_state: Default::default(),
			temporary_buffer: Default::default(),
			parameters_buffer: Default::default(),
		}
	}
}

impl<'a, 'b> ParseCommandParametersBuilder<'a, 'b> {
	pub(super) fn analyze(&mut self) -> Result<(), IrcMessageCommandError> {
		// Première étape
		loop {
			match self.first_step_state {
				| ParseCommandParametersFirstStepState::Initial => {
					match self.stream.consume_next()? {
						| codepoint if codepoint.is_newline() => {
							return Ok(());
						}
						| CodePoint::EOF if cfg!(test) => return Ok(()),
						| CodePoint::EOF => {
							return Err(
								IrcMessageCommandError::UnterminatedLine,
							);
						}

						| codepoint if codepoint.is_whitespace() => {
							self.stream.reconsume_current();
							self.first_step_state.switch(
								ParseCommandParametersFirstStepState::HasParameters,
							)
						}

						| codepoint if codepoint.is_valid() => {
							return Err(
								IrcMessageCommandError::InvalidCharacter {
									found: codepoint.unit(),
									help: "Un espace blanc est attendu.",
								},
							)
						}

						| _ => return Err(IrcMessageCommandError::ParseError),
					}
				}

				| ParseCommandParametersFirstStepState::HasParameters => {
					match self.stream.consume_next()? {
						// Saut de ligne.
						//
						// Re-consommer le point de code. Arrêter l'analyse.
						| codepoint if codepoint.is_newline() => {
							self.stream.reconsume_current();
							break;
						}

						// Espaces blancs.
						//
						// Si le prochain point de code est un U+003A COLON (:),
						// alors re-consommer le point de code, et arrêter
						// l'analyse. Dans les autres cas, ajouter le point de
						// code au tampon temporaire.
						| codepoint if codepoint.is_whitespace() => {
							if self.stream.peek_next()? == CodePoint::COLON {
								self.stream.reconsume_current();
								break;
							}

							self.add_character_to_temporary_buffer(
								codepoint.unit(),
							);
						}

						| CodePoint::EOF if cfg!(test) => break,

						// EOF
						| CodePoint::EOF => return Ok(()),

						// Tous les autres points de code valide.
						//
						// Ajouter le point de code au tampon temporaire.
						| codepoint if codepoint.is_valid() => {
							self.add_character_to_temporary_buffer(
								codepoint.unit(),
							);
						}

						| _ => return Err(IrcMessageCommandError::ParseError),
					}
				}
			}
		}

		let middle: Vec<String> = self
			.temporary_buffer
			.split_whitespace()
			.map(|s| s.to_owned())
			.collect();

		self.temporary_buffer.clear();

		// Seconde étape
		loop {
			match self.second_step_state {
				| ParseCommandParametersSecondStepState::Initial => {
					match self.stream.consume_next()? {
						| codepoint if codepoint.is_newline() => break,

						// Espaces blancs.
						//
						// Si le prochain point de code est un U+003A COLON (:),
						// ne rien faire. Sinon il s'agit d'une erreur
						// d'analyse.
						| codepoint if codepoint.is_whitespace() => {
							if self.stream.peek_next()? == CodePoint::COLON {
								continue;
							}

							return Err(IrcMessageCommandError::ParseError);
						}

						// U+003A COLON (:).
						//
						// Passer à l'état
						// [ParseCommandParametersSecondStepState::AfterColon].
						| CodePoint::COLON => {
							self.second_step_state
								.switch(ParseCommandParametersSecondStepState::AfterColon);
						}

						| codepoint if codepoint.is_newline() => {
							self.stream.reconsume_current();
							break;
						}
						| CodePoint::EOF if cfg!(test) => break,

						| CodePoint::EOF => {
							return Err(
								IrcMessageCommandError::UnterminatedLine,
							);
						}

						| codepoint if codepoint.is_valid() => {
							self.add_character_to_temporary_buffer(
								codepoint.unit(),
							);
						}

						| _ => return Err(IrcMessageCommandError::ParseError),
					}
				}

				| ParseCommandParametersSecondStepState::AfterColon => {
					match self.stream.consume_next()? {
						// Saut de ligne.
						//
						// Arrêter l'analyse.
						| codepoint if codepoint.is_newline() => {
							self.stream.reconsume_current();
							break;
						}

						| CodePoint::EOF if cfg!(test) => break,
						| CodePoint::EOF => {
							return Err(
								IrcMessageCommandError::UnterminatedLine,
							);
						}

						// Tous les points de code valide.
						//
						// Ajouter le point de code au tampon temporaire.
						| codepoint if codepoint.is_valid() => {
							self.add_character_to_temporary_buffer(
								codepoint.unit(),
							);
						}

						// Tous les autres points de code.
						//
						// Il s'agit d'une erreur.
						| _ => return Err(IrcMessageCommandError::ParseError),
					}
				}
			}
		}

		let mut parameters = middle;
		parameters.push(self.temporary_buffer.trim().to_string());
		parameters.retain(|s| !s.is_empty());

		self.parameters_buffer = parameters;

		Ok(())
	}

	fn add_character_to_temporary_buffer(&mut self, ch: char) {
		self.temporary_buffer.push(ch);
	}
}

impl<'a, 'b> ParseCommandParametersBuilder<'a, 'b> {
	pub(super) fn finish(
		self,
	) -> Result<IrcMessageCommandParameters, IrcMessageCommandError> {
		Ok(IrcMessageCommandParameters(self.parameters_buffer))
	}
}
