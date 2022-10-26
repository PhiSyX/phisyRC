/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod builder;
mod parameters;
mod state;

use core::fmt;
use std::str::{Chars, FromStr};

use lang::stream::{InputStream, InputStreamError};

use self::{
	builder::ParseCommandBuilder, parameters::MessageCommandParameters,
};

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(PartialEq, Eq)]
#[derive(serde::Serialize)]
#[serde(untagged)]
pub enum MessageCommand {
	/// Une commande numérique est dotée de 3 chiffres.
	/// Cette commande est obligatoirement poursuivie par un paramètre.
	Numeric {
		/// Code de 3 chiffres.
		code: String,
		/// Les informations supplémentaires de la commande numérique.
		parameters: MessageCommandParameters,
	},

	/// Une commande textuelle est dotée d'une suite de lettre alphabétique.
	/// Cette commande PEUT être suivie de paramètres, mais n'est pas
	/// obligatoire.
	Text {
		/// La commande, par exemple: "PASS"
		#[serde(rename = "name")]
		command: String,

		/// Les paramètres/arguments de la commande, par exemple:
		///
		/// INPUT = "pass mot de passe"
		///
		/// parameters = ["mot", "de", "passe"]
		parameters: MessageCommandParameters,
	},
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum MessageCommandError {
	InputStream,
	ParseError,
	IsEmpty,
	InvalidCharacter { found: char, help: &'static str },
	NumericCodeIsTooShort,
	NumericCodeIsTooLong,
	UnterminatedLine,
}

// -------------- //
// Implémentation //
// -------------- //

impl MessageCommand {
	/// Analyse d'une commande.
	//
	// BNF <command>: 1*letter / 3digit
	pub(super) fn parse(
		stream: &mut InputStream<Chars<'_>, char>,
	) -> Result<Self, MessageCommandError> {
		let mut builder = ParseCommandBuilder::initialize(stream);
		builder.analyze()?;
		builder.finish().and_then(|mut command| {
			let parameters = Self::parse_parameters(stream)?;
			command.set_parameters(parameters);
			Ok(command)
		})
	}

	fn parse_parameters(
		stream: &mut InputStream<Chars<'_>, char>,
	) -> Result<MessageCommandParameters, MessageCommandError> {
		MessageCommandParameters::parse(stream)
	}
}

impl MessageCommand {
	fn set_parameters(&mut self, new_parameters: MessageCommandParameters) {
		match self {
			| Self::Numeric { parameters, .. } => {
				*parameters = new_parameters;
			}
			| Self::Text { parameters, .. } => {
				*parameters = new_parameters;
			}
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<InputStreamError> for MessageCommandError {
	fn from(_: InputStreamError) -> Self {
		Self::InputStream
	}
}

impl fmt::Display for MessageCommandError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::InputStream => "erreur d'analyse".to_owned(),
				| Self::IsEmpty => "la ligne est vide".to_owned(),
				| Self::ParseError => "erreur d'analyse".to_owned(),
				| Self::InvalidCharacter { found, .. } => format!(
					"le caractère « {found} » est invalide pour la commande."
				),
				| Self::NumericCodeIsTooShort =>
					"le code numérique est trop petit".to_owned(),
				| Self::NumericCodeIsTooLong =>
					"le code numérique est trop long".to_owned(),
				| Self::UnterminatedLine =>
					"la ligne n'est pas terminée".to_owned(),
			}
		)
	}
}

impl FromStr for MessageCommandError {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.ends_with("la ligne est vide") {
			return Ok(Self::IsEmpty);
		} else if s.ends_with("erreur d'analyse") {
			return Ok(Self::ParseError);
		} else if s.contains("caractère invalide -> ") {
			let parts = unsafe {
				s.split_once(" -> ")
					.map(|(_, x)| {
						x.replace("\\s", " ")
							.replace("\\r", "\r")
							.replace("\\n", "\n")
					})
					.unwrap_unchecked()
			};

			let found = parts.as_bytes();

			return Ok(Self::InvalidCharacter {
				found: found[0] as char,
				help: "Un caractère de commande valide est attendu.",
			});
		} else if s.ends_with("code numérique trop court") {
			return Ok(Self::NumericCodeIsTooShort);
		} else if s.ends_with("code numérique trop long") {
			return Ok(Self::NumericCodeIsTooLong);
		} else if s.ends_with("ligne non terminée") {
			return Ok(Self::UnterminatedLine);
		}

		Err("non géré")
	}
}

// ---- //
// Test //
// ---- //

#[cfg(test)]
mod tests {
	use super::*;

	fn parse(source: &str) -> Result<MessageCommand, MessageCommandError> {
		let mut input = InputStream::new(source.chars());
		MessageCommand::parse(&mut input)
	}

	#[test]
	fn test_command_numeric() {
		let input = "001 PhiSyX :Welcome to the Internet Relay Network";
		let output = parse(input).unwrap();
		assert_eq!(
			output,
			MessageCommand::Numeric {
				code: "001".to_owned(),
				parameters: ["PhiSyX", "Welcome to the Internet Relay Network"]
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
			MessageCommand::Text {
				command: "NICK".to_owned(),
				parameters: ["NAME"].into()
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
