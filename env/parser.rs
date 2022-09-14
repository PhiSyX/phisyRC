/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::{
	fs::File,
	io::{self, Read},
	path::Path,
};

// ---- //
// Type //
// ---- //

type LValue<'a> = &'a str;
type RValue<'a> = &'a str;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
pub struct Parser;

#[derive(Debug)]
struct DeclarationStatement<'a>(LValue<'a>, RValue<'a>);

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
enum DeclarationStatementError {
	ParseError,
}

// -------------- //
// Implémentation //
// -------------- //

impl Parser {
	/// Analyse un fichier d'environnement.
	pub fn file(filename: impl AsRef<Path>) -> io::Result<()> {
		let mut file = File::open(filename)?;

		let mut content = String::new();
		file.read_to_string(&mut content)?;

		let decls = Self::parse(&content);
		decls.for_each(|decl| decl.set_env());

		Ok(())
	}

	/// Analyse une chaîne de caractères.
	fn parse(input: &str) -> impl Iterator<Item = DeclarationStatement> + '_ {
		input
			.lines()
			.filter_map(|line| DeclarationStatement::parse(line).ok())
	}
}

impl<'a> DeclarationStatement<'a> {
	/// Définie une variable d'environnement.
	fn set_env(&self) {
		let Self(key, value) = self;
		std::env::set_var(key, value);
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<'a> DeclarationStatement<'a> {
	fn parse<'b: 'a>(
		input: &'b str,
	) -> Result<Self, DeclarationStatementError> {
		input
			.split_once('=')
			.map(|(key, value)| Self(key, value))
			.ok_or(DeclarationStatementError::ParseError)
	}
}
