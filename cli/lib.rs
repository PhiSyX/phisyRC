/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod commands;
mod flags;
mod options;

use clap::{Args, Parser, Subcommand};

pub use self::{commands::*, flags::*, options::*};

/// Nom du projet.
pub const PROJECT_NAME: &str = "
    / ' _   _ _
 /)/)/_) (// (
/        /    `
";

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Parser)]
#[clap(version)]
pub struct CLI<F, O, C>
where
	F: Parser + Args,
	O: Parser + Args,
	C: Parser + Subcommand,
{
	/// Les drapeaux.
	#[clap(flatten)]
	pub flags: F,

	/// Les options.
	#[clap(flatten)]
	pub options: O,

	/// Le commande.
	#[clap(subcommand)]
	pub command: Option<C>,
}

#[allow(non_camel_case_types)]
pub type phisyrc =
	CLI<self::flags::Flags, self::options::Options, self::commands::Command>;

// -------------- //
// Implementation //
// -------------- //

impl phisyrc {
	/// Construit la structure [CLI] à partir des arguments de la ligne de
	/// commande (basé sur [std::env::args_os]).
	pub fn arguments() -> Self {
		Self::display_project();

		Self::parse()
	}

	/// Affiche le nom du projet dans la console.
	fn display_project() {
		println!("{PROJECT_NAME}");
	}
}
