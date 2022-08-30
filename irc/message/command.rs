/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;
use std::str::Chars;

use lang::{codepoints::CodePoint, lexer::ParseState, stream::prelude::*};

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum IrcMessageCommand {
	/// Une commande numérique est dotée de 3 chiffres.
	/// Cette commande est obligatoirement poursuivie par un paramètre.
	Numeric {
		/// Code de 3 chiffres.
		code: String,
		/// Les informations supplémentaires de la commande numérique.
		parameters: Vec<String>,
	},

	/// Une commande textuelle est dotée d'une suite de lettre alphabétique.
	/// Cette commande PEUT être suivie de paramètres, mais n'est pas
	/// obligatoire.
	Text {
		/// La commande, par exemple: "PASS"
		command: String,

		/// Les paramètres/arguments de la commande, par exemple:
		///
		/// INPUT = "pass mot de passe"
		///
		/// parameters = ["mot", "de", "passe"]
		parameters: Vec<String>,
	},
}

pub struct IrcMessageCommandParams;

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
pub(super) enum IrcMessageCommandError {
	InputStream,
	InvalidCharacter,
	NumericCodeIsTooLong,
}

// -------------- //
// Implémentation //
// -------------- //

impl IrcMessageCommand {
	/// Analyse d'une commande.
	//
	// BNF <command>: 1*letter / 3digit
	pub(super) fn parse(
		stream: &mut InputStream<Chars<'_>, char>,
	) -> Result<Self, IrcMessageCommandError> {
		#[derive(PartialEq, Eq)]
		enum State {
			Initial,
			Numeric { counter: u8 },
			Text,
		}

		impl State {
			fn increment_counter(&mut self) {
				if let State::Numeric { counter: count } = self {
					*count += 1;
				}
			}
		}

		impl ParseState for State {
			fn switch(&mut self, new_state: State) {
				*self = new_state;
			}
		}

		let mut temporary_buffer = String::new();
		let mut state = State::Initial;

		loop {
			match state {
				| State::Initial => match stream.consume_next()? {
					// Caractère numérique.
					//
					// Ajouter le chiffre au tampon temporaire.
					// Passer à l'état [State::Numeric] avec un compteur de 1.
					| CodePoint::Unit(ch) if ch.is_numeric() => {
						temporary_buffer.push(ch);
						state.switch(State::Numeric { counter: 1 });
					}

					// Caractère alphabétique.
					//
					// Ajouter la version majuscule de la lettre au tampon
					// temporaire. Passer à l'état [State::Text].
					| CodePoint::Unit(ch) if ch.is_alphabetic() => {
						ch.to_uppercase()
							.for_each(|u| temporary_buffer.push(u));
						state.switch(State::Text);
					}

					// Tous les autres points de code.
					//
					// Il s'agit d'une erreur d'analyse.
					| _ => {
						return Err(IrcMessageCommandError::InvalidCharacter)
					}
				},

				| State::Numeric { counter: count } => {
					match stream.consume_next()? {
						// Si le compteur est plus grand que 3, il s'agit d'une
						// erreur.
						| _ if count > 3 => {
							return Err(
								IrcMessageCommandError::NumericCodeIsTooLong,
							)
						}

						// Caractère numérique.
						//
						// Ajouter le chiffre au tampon temporaire.
						// Incrémenter le compteur de 1.
						| CodePoint::Unit(ch) if ch.is_numeric() => {
							temporary_buffer.push(ch);
							state.increment_counter();
						}

						// Espaces blancs.
						//
						// Arrêter l'analyse.
						| codepoint if codepoint.is_whitespace() => {
							stream.reconsume_current();
							break;
						}

						// Tous les autres caractères.
						//
						// Il s'agit d'une erreur.
						| _ => {
							return Err(
								IrcMessageCommandError::InvalidCharacter,
							)
						}
					}
				}

				| State::Text => match stream.consume_next()? {
					// Caractère alphabétique.
					//
					// Ajouter la version majuscule de la lettre au tampon
					// temporaire. Passer à l'état [State::Text].
					| CodePoint::Unit(ch) if ch.is_alphabetic() => {
						ch.to_uppercase()
							.for_each(|u| temporary_buffer.push(u));
					}

					// Espaces blancs.
					// EOF.
					//
					// Arrêter l'analyse.
					| codepoint if codepoint.is_whitespace() => {
						stream.reconsume_current();
						break;
					}
					| CodePoint::EOF => break,

					// Tous les autres points de code.
					//
					// Il s'agit d'une erreur d'analyse.
					| _ => {
						return Err(IrcMessageCommandError::InvalidCharacter)
					}
				},
			}
		}

		assert!(state != State::Initial);

		Ok(match state {
			| State::Numeric { .. } => IrcMessageCommand::Numeric {
				code: temporary_buffer,
				parameters: IrcMessageCommandParams::parse(stream)?,
			},

			| State::Text => IrcMessageCommand::Text {
				command: temporary_buffer,
				parameters: IrcMessageCommandParams::parse(stream)?,
			},

			| State::Initial => {
				unreachable!("capturé par l'assertion plus-haut.")
			}
		})
	}
}

impl IrcMessageCommandParams {
	pub(super) fn parse(
		stream: &mut InputStream<Chars<'_>, char>,
	) -> Result<Vec<String>, IrcMessageCommandError> {
		let mut temporary_buffer = String::new();

		loop {
			match stream.consume_next()? {
				// Espaces blancs.
				//
				// Si le prochain point de code est un U+003A COLON (:),
				// alors re-consommer le point de code, et arrêter l'analyse.
				// Dans les autres cas, ajouter le point de code au tampon
				// temporaire.
				| codepoint if codepoint.is_whitespace() => {
					if stream.peek_next()? == CodePoint::COLON {
						stream.reconsume_current();
						break;
					}

					temporary_buffer.push(codepoint.unit());
				}

				// Saut de ligne.
				// EOF
				//
				// Arrêter l'analyse.
				| codepoint if codepoint.is_newline() => break,
				| CodePoint::EOF => break,

				// Tous les autres points de code valide.
				//
				// Ajouter le point de code au tampon temporaire.
				| codepoint if codepoint.is_valid() => {
					temporary_buffer.push(codepoint.unit());
				}

				// Tous les autres points de code.
				//
				// Il s'agit d'une erreur.
				| _ => return Err(IrcMessageCommandError::InvalidCharacter),
			}
		}

		let middle: Vec<String> = temporary_buffer
			.split_whitespace()
			.map(|s| s.to_owned())
			.collect();

		let mut temporary_buffer = String::new();

		enum TrailingState {
			Initial,
			AfterColon,
		}

		impl ParseState for TrailingState {
			fn switch(&mut self, new_state: Self) {
				*self = new_state;
			}
		}

		let mut state = TrailingState::Initial;

		loop {
			match state {
				| TrailingState::Initial => {
					match stream.consume_next()? {
						// Espaces blancs.
						//
						// Si le prochain point de code est un U+003A COLON (:),
						// ne rien faire. Sinon il s'agit d'une erreur
						// d'analyse.
						| codepoint if codepoint.is_whitespace() => {
							if stream.peek_next()? == CodePoint::COLON {
								continue;
							}

							// NOTE
							return Err(
								IrcMessageCommandError::InvalidCharacter,
							);
						}

						// U+003A COLON (:).
						//
						// Passer à l'état [State::AfterColon].
						| CodePoint::COLON => {
							state.switch(TrailingState::AfterColon);
						}

						| codepoint if codepoint.is_newline() => break,
						| CodePoint::EOF => break,

						| codepoint if codepoint.is_valid() => {
							temporary_buffer.push(codepoint.unit());
						}

						| _ => {
							return Err(
								IrcMessageCommandError::InvalidCharacter,
							)
						}
					}
				}

				| TrailingState::AfterColon => match stream.consume_next()? {
					// Saut de ligne.
					//
					// Arrêter l'analyse.
					| codepoint if codepoint.is_newline() => break,
					| CodePoint::EOF => break,

					// Tous les points de code valide.
					//
					// Ajouter le point de code au tampon temporaire.
					| codepoint if codepoint.is_valid() => {
						temporary_buffer.push(codepoint.unit());
					}

					// Tous les autres points de code.
					//
					// Il s'agit d'une erreur.
					| _ => {
						return Err(IrcMessageCommandError::InvalidCharacter)
					}
				},
			}
		}

		let mut params = middle;
		params.push(temporary_buffer.trim().to_string());
		params.retain(|s| !s.is_empty());

		Ok(params)
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<InputStreamError> for IrcMessageCommandError {
	fn from(_: InputStreamError) -> Self {
		Self::InputStream
	}
}

impl fmt::Display for IrcMessageCommandError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::InputStream => "erreur d'analyse.",
				| Self::InvalidCharacter => "caractère invalide.",
				| Self::NumericCodeIsTooLong => "code numérique trop long.",
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
		source: &str,
	) -> Result<IrcMessageCommand, IrcMessageCommandError> {
		let mut input = InputStream::new(source.chars());
		IrcMessageCommand::parse(&mut input)
	}

	#[test]
	fn test_command_numeric() {
		let input = "001 PhiSyX :Welcome to the Internet Relay Network";
		let output = parse(input).unwrap();
		assert_eq!(
			output,
			IrcMessageCommand::Numeric {
				code: "001".to_owned(),
				parameters: [
					"PhiSyX".to_owned(),
					"Welcome to the Internet Relay Network".to_owned()
				]
				.into()
			}
		);
	}

	#[test]
	fn test_command_text() {
		let input = "NICK NAME";
		let output = parse(input).unwrap();
		assert_eq!(
			output,
			IrcMessageCommand::Text {
				command: "NICK".to_owned(),
				parameters: ["NAME".to_owned()].to_vec()
			}
		);
	}

	#[test]
	fn test_command_invalid() {
		let input = "PING1";
		let output = parse(input);
		assert!(output.is_err());

		let input = "0001 PhiSyX :Welcome to the Internet Relay Network";
		let output = parse(input);
		assert!(output.is_err());
	}
}
