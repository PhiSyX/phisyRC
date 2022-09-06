/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::str::Chars;

use lang::{codepoints::CodePoint, lexer::ParseState, stream::prelude::*};

use super::{
	parameters::IrcMessageCommandParameters,
	state::{ParseCommandParametersStepState, ParseCommandState},
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
	step_state: ParseCommandParametersStepState,
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
	/// Analyse d'une commande.
	pub(super) fn analyze(&mut self) -> Result<(), IrcMessageCommandError> {
		loop {
			match self.state {
				| ParseCommandState::Initial => {
					match self.stream.consume_next()? {
						// Point de code de 0 à 9
						//
						// Re-consommer le point de code actuel.
						// Passer à l'état [ParseCommandState::Numeric]
						// en initialisant un compteur de 0.
						| CodePoint::Unit(ch) if ch.is_numeric() => {
							self.stream.reconsume_current();
							self.state.switch(ParseCommandState::Numeric {
								counter: 0,
							})
						}

						// Point de code alphabétique
						//
						// Re-consommer le point de code actuel.
						// Passer à l'état [ParseCommandState::Text].
						| CodePoint::Unit(ch) if ch.is_alphabetic() => {
							self.stream.reconsume_current();
							self.state.switch(ParseCommandState::Text)
						}

						// Tous les points de code valide.
						//
						// Il s'agit d'une erreur d'analyse.
						| codepoint if codepoint.is_valid() => return Err(
							IrcMessageCommandError::InvalidCharacter {
								found: codepoint.unit(),
								help: "Un caractère de commande valide est attendu.",
							},
						),

						// Tous les autres cas.
						//
						// Il s'agit d'une erreur d'analyse.
						| _ => return Err(IrcMessageCommandError::ParseError),
					}
				}

				// La commande est une commande numérique, autrement dit une
				// commande qui commence par un point de code numérique.
				| ParseCommandState::Numeric { counter } => {
					match self.stream.consume_next()? {
						// Peu importe le point de code qui a été capturé,
						// si le compteur va au de-là de 3 chiffre, il s'agit
						// d'une erreur d'analyse, conformément à la
						// spécification IRC.
						| _ if counter > 3 => {
							return Err(
								IrcMessageCommandError::NumericCodeIsTooLong,
							)
						}

						// Point de code numérique
						//
						// Ajouter le point de code au tampon temporaire.
						// Incrémenter le compteur.
						| CodePoint::Unit(ch) if ch.is_numeric() => {
							self.add_character_to_temporary_buffer(ch);
							self.state.increment_counter();
						}

						// Espace blanc.
						//
						// Re-consommer le point de code actuel.
						// Arrêter l'analyse : passer à l'état d'analyse
						// des paramètres.
						| CodePoint::Whitespace(_) => {
							self.stream.reconsume_current();
							break;
						}

						// Saut de ligne
						//
						// Arrêter complètement l'analyse.
						| CodePoint::Newline(_) => return Ok(()),

						// Tous les points de code valide.
						//
						// Il s'agit d'une erreur d'analyse.
						| codepoint if codepoint.is_valid() => {
							return Err(
								IrcMessageCommandError::InvalidCharacter {
									found: codepoint.unit(),
									help: "Un caractère de commande valide est attendu.",
								},
							)
						}

						// Tous les autres points de code
						//
						// Il s'agit d'une erreur d'analyse.
						| _ => return Err(IrcMessageCommandError::ParseError),
					}
				}

				// La commande est une commande normale, autrement dit une
				// commande qui commence par un point de code alphabétique.
				| ParseCommandState::Text => {
					match self.stream.consume_next()? {
						// NOTE(phisyx): La spécification IRC n'autorise pas
						// de chiffres pour les commandes type "text".
						//
						// phisyRC ne va pas suivre cette règle spécifique pour
						// cette fois. Parce que nous somme des guedin's.
						| CodePoint::Unit(ch) if ch.is_alphanumeric() => {
							self.add_character_to_temporary_buffer(ch);
						}

						// Espace blancs
						//
						// Arrêter l'analyse.
						| CodePoint::Whitespace(_) => {
							self.stream.reconsume_current();
							break
						}

						// Saut de ligne
						//
						// Arrêter complètement l'analyse.
						| CodePoint::Newline(_) => return Ok(()),

						// Tous les points de code valide.
						//
						// Il s'agit d'une erreur d'analyse.
						| codepoint if codepoint.is_valid() => {
							return Err(
								IrcMessageCommandError::InvalidCharacter {
									found: codepoint.unit(),
									help: "Un caractère de commande valide est attendu.",
								},
							)
						}

						//  Tous les autres points de code.
						//
						// Il s'agit d'une erreur d'analyse.
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
	/// Méthode de construction de la structure [IrcMessageCommand]
	pub(super) fn finish(
		self,
	) -> Result<IrcMessageCommand, IrcMessageCommandError> {
		match self.state {
			// Être dans état est impossible.
			//
			// Il s'agit d'une erreur d'analyse.
			| ParseCommandState::Initial => {
				assert!(self.state != ParseCommandState::Initial);
				Err(IrcMessageCommandError::ParseError)
			}

			// Construction d'une commande numérique.
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

			// Construction d'une commande normale.
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
			step_state: Default::default(),
			temporary_buffer: Default::default(),
			parameters_buffer: Default::default(),
		}
	}
}

impl<'a, 'b> ParseCommandParametersBuilder<'a, 'b> {
	pub(super) fn analyze(&mut self) -> Result<(), IrcMessageCommandError> {
		loop {
			match self.step_state {
				| ParseCommandParametersStepState::Initial => {
					match self.stream.consume_next()? {
						// EOF
						//
						// NOTE(phisyx): lorsque le code est exécuté pendant les
						// tests. Arrêter complètement l'analyse.
						//
						// Il s'agit d'une erreur d'analyse.
						| CodePoint::EOF if cfg!(test) => return Ok(()),
						| CodePoint::EOF => {
							return Err(
								IrcMessageCommandError::UnterminatedLine,
							);
						}

						// Espace blanc
						//
						// Re-consommer le point de code actuel.
						// Passer à l'état
						// [ParseCommandParametersFirstStepState::HasParameters]
						| codepoint if codepoint.is_whitespace() => {
							self.stream.reconsume_current();
							self.step_state.switch(
								ParseCommandParametersStepState::FirstStep,
							);
						}

						// Tous les points de code valide.
						//
						// Il s'agit d'une erreur d'analyse.
						| codepoint if codepoint.is_valid() => {
							return Err(
								IrcMessageCommandError::InvalidCharacter {
									found: codepoint.unit(),
									help: "Un espace blanc est attendu.",
								},
							)
						}

						// Tous les autres points de code.
						//
						// Il s'agit d'une erreur d'analyse.
						| _ => return Err(IrcMessageCommandError::ParseError),
					}
				}

				| ParseCommandParametersStepState::FirstStep => {
					match self.stream.consume_next()? {
						// Saut de ligne.
						//
						// Re-consommer le point de code. Arrêter l'analyse.
						| codepoint if codepoint.is_newline() => {
							self.stream.reconsume_current();
							self.step_state.switch(ParseCommandParametersStepState::PrepareSecondStep);
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
								self.step_state.switch(ParseCommandParametersStepState::PrepareSecondStep);
							}

							self.add_character_to_temporary_buffer(
								codepoint.unit(),
							);
						}

						// EOF
						//
						// NOTE(phisyx): dans le cas des tests, on veut arrêter
						// l'analyse.
						//
						// Arrêter complètement l'analyse.
						| CodePoint::EOF if cfg!(test) => {
							self.step_state.switch(ParseCommandParametersStepState::PrepareSecondStep);
						}
						| CodePoint::EOF => return Ok(()),

						// Tous les autres points de code valide.
						//
						// Ajouter le point de code au tampon temporaire.
						| codepoint if codepoint.is_valid() => {
							self.add_character_to_temporary_buffer(
								codepoint.unit(),
							);
						}

						// Tous les autres points de code.
						//
						// Il s'agit d'une erreur d'analyse.
						| _ => return Err(IrcMessageCommandError::ParseError),
					}
				}

				// NOTE(phisyx): pour la commande "CAP LS 302: LOL"
				// le tampon temporaire contiendra "LS 302". L'étape
				// suivante se contente de récupérer la suite.
				| ParseCommandParametersStepState::PrepareSecondStep => {
					let middle: Vec<String> = self
						.temporary_buffer
						.split_whitespace() // ["LS", "302"]
						.map(|s| s.to_owned())
						.collect();
					self.parameters_buffer = middle;
					self.temporary_buffer.clear();
					self.step_state
						.switch(ParseCommandParametersStepState::SecondStep);
				}

				| ParseCommandParametersStepState::SecondStep => {
					match self.stream.consume_next()? {
						// Saut de ligne.
						//
						// Re-consommer le point de code actuel.
						// Arrêter l'analyse.
						| codepoint if codepoint.is_newline() => {
							self.stream.reconsume_current();
							self.step_state.switch(
								ParseCommandParametersStepState::Finish,
							);
						}

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
						// [ParseCommandParametersStepState::AfterColon].
						| CodePoint::COLON => {
							self.step_state.switch(
								ParseCommandParametersStepState::AfterColon,
							);
						}

						// EOF
						//
						// NOTE(phisyx): dans le cas des tests, nous voulons
						// arrêter l'analyse.
						//
						// Il s'agit d'une erreur d'analyse.
						| CodePoint::EOF if cfg!(test) => {
							self.step_state.switch(
								ParseCommandParametersStepState::Finish,
							);
						}
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
						// Il s'agit d'une erreur d'analyse.
						| _ => return Err(IrcMessageCommandError::ParseError),
					}
				}

				| ParseCommandParametersStepState::AfterColon => {
					match self.stream.consume_next()? {
						// Saut de ligne.
						//
						// Arrêter l'analyse.
						| codepoint if codepoint.is_newline() => {
							self.stream.reconsume_current();
							self.step_state.switch(
								ParseCommandParametersStepState::Finish,
							);
						}

						// EOF
						//
						// NOTE(phisyx): dans le cas des tests, nous voulons
						// arrêter l'analyse.
						//
						// Il s'agit d'une erreur d'analyse.
						| CodePoint::EOF if cfg!(test) => {
							self.step_state.switch(
								ParseCommandParametersStepState::Finish,
							);
						}
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

				| ParseCommandParametersStepState::Finish => {
					self.parameters_buffer
						.push(self.temporary_buffer.trim().to_string());
					self.parameters_buffer.retain(|s| !s.is_empty());
					break;
				}
			}
		}

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
