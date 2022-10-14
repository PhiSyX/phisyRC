/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::fmt;
use std::{ops, path::PathBuf};

use clap::Parser;
use cli::{EmptyCommand, EmptyFlags, ProcessEnv, CLI, PROJECT_NAME};

// ---- //
// Type //
// ---- //

#[allow(non_camel_case_types)]
type CliApp = CLI<EmptyFlags, Options, Command>;

pub type CommandMakePassword =
	CLI<CommandMakePasswordFlags, CommandMakePasswordOptions, EmptyCommand>;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Parser)]
pub struct Options {
	/// Fichier de configuration du serveur.
	#[clap(short, long, value_parser)]
	pub config: Option<PathBuf>,

	/// Dans quel mode le serveur doit être lancé.
	///
	/// Les modes sont utilisés pour:
	///
	///   1. choisir le fichier d'environnement à utiliser ;
	///
	///   2. le système de log ;
	///
	///   3. des informations renvoyées aux clients concernant des
	///      messages/comportements du programme spécifique à certains
	///      modes ;
	///
	///   4. ...
	///   ———
	#[clap(value_enum)]
	#[clap(
		long,
		ignore_case = true,
		env = "PROCESS_ENV",
		default_value = "development"
	)]
	pub mode: ProcessEnv,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct cli_app(CliApp);

#[derive(Debug)]
#[derive(Parser)]
pub struct CommandMakePasswordFlags {
	/// Mot de passe
	pub password: String,
}

#[derive(Debug)]
#[derive(clap::Parser)]
pub struct CommandMakePasswordOptions {
	/// Algorithme à utiliser pour hacher le mot de passe.
	#[clap(value_enum)]
	#[clap(long, default_value = "argon2")]
	pub algo: PasswordAlgorithm,
	/// La clé secrète de l'application.
	#[clap(env = "APP_SECRET_KEY")]
	pub secret_key: Option<String>,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Parser)]
pub enum Command {
	/// Génération d'un mot de passe avec un algorithme de hachage.
	#[clap(name = "make:password")]
	MakePassword(CommandMakePassword),
}

#[derive(Debug)]
#[derive(Default)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(clap::ValueEnum)]
#[allow(clippy::upper_case_acronyms)]
pub enum PasswordAlgorithm {
	/// Algorithme de hachage Argon2.
	#[default]
	Argon2,
}

// -------------- //
// Implémentation //
// -------------- //

impl cli_app {
	/// Construit la structure [CLI] à partir des arguments de la ligne de
	/// commande (basé sur [std::env::args_os]).
	pub fn arguments() -> Self {
		Self::display_project();
		Self(CliApp::parse())
	}

	/// Affiche le nom du projet dans la console.
	fn display_project() {
		println!("{PROJECT_NAME}");
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl ops::Deref for cli_app {
	type Target = CliApp;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl fmt::Display for PasswordAlgorithm {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let algo = match self {
			| Self::Argon2 => "Argon2",
		};
		write!(f, "{algo}")
	}
}
