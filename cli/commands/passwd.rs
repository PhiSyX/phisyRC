/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;

use super::EmptyCommand;
use crate::CLI;

pub type CommandMakePassword =
	CLI<CommandPasswordFlags, CommandPasswordOptions, EmptyCommand>;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(clap::Parser)]
pub struct CommandPasswordFlags {
	/// Mot de passe
	pub password: String,
}

#[derive(Debug)]
#[derive(clap::Parser)]
pub struct CommandPasswordOptions {
	/// Algorithme à utiliser pour hacher le mot de passe.
	#[clap(arg_enum)]
	#[clap(long, default_value = "argon2")]
	pub algo: PasswordAlgorithm,
}

#[derive(Debug)]
#[derive(Default)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[derive(clap::ArgEnum)]
#[allow(clippy::upper_case_acronyms)]
pub enum PasswordAlgorithm {
	#[default]
	Argon2,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for PasswordAlgorithm {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let algo = match self {
			| Self::Argon2 => "Argon2",
		};
		write!(f, "{algo}")
	}
}
