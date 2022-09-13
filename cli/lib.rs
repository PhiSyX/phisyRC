/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod art;
mod commands;
mod export;
mod flags;
pub mod layout;
mod options;

use clap::{Args, Parser, Subcommand};
pub use export::*;

pub mod app {
	use clap::Parser;

	pub use super::{commands::*, flags::*, options::*};
	use crate::{CLI, PROJECT_NAME};

	#[allow(non_camel_case_types)]
	pub type phisyrc_cli = CLI<Flags, Options, Command>;

	// -------------- //
	// Implementation //
	// -------------- //

	impl phisyrc_cli {
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
}

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
