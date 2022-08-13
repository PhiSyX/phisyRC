/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::{
	app::{EmptyCommand, EmptyFlags},
	CLI,
};

pub type CommandClient = CLI<EmptyFlags, CommandClientOptions, EmptyCommand>;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(clap::Parser)]
pub struct CommandClientOptions {
	/// Lance l'application de Chat en mode graphique.
	#[clap(long, conflicts_with = "tui")]
	pub gui: bool,

	/// Lance l'application de Chat en mode textuel.
	#[clap(long, conflicts_with = "gui")]
	pub tui: bool,
}
