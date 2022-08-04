/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(clap::Parser)]
pub struct EmptyOptions {}

#[derive(Debug)]
#[derive(clap::Parser)]
pub struct Options {
	/// Dans quel mode le programme doit être lancé.
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
	#[clap(arg_enum)]
	#[clap(
		long,
		ignore_case = true,
		env = "PROCESS_ENV",
		default_value = "development"
	)]
	pub mode: ProcessEnv,
}

// ----------- //
// Énumération //
// ----------- //

/// Les modes dans lesquels le programme peut s'exécuter.
///
/// Les modes sont utilisés pour:
///
///   1. choisir le fichier d'environnement à utiliser ;
///   2. le système de log ;
///   3. des informations renvoyées aux clients concernant des
///      messages/comportement du programme spécifique à certains
///      modes ;
///   4. ...
///
/// La valeur est définie grâce à la variable d'environnement
/// `PROCESS_ENV`. La valeur par défaut de cette énumération est définie
/// dans l'implémentation [Default](ProcessEnv::default()).
#[derive(Debug)]
#[derive(Default)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(clap::ArgEnum)]
#[allow(clippy::upper_case_acronyms)]
pub enum ProcessEnv {
	#[default]
	DEVELOPMENT,
	PRODUCTION,
	TEST,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for ProcessEnv {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mode = match self {
			| Self::DEVELOPMENT => "DEVELOPMENT",
			| Self::PRODUCTION => "PRODUCTION",
			| Self::TEST => "TEST",
		};
		write!(f, "{mode}")
	}
}
