/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod banner;
mod export;
mod process;

use clap::{Args, Parser, Subcommand};

pub use self::export::*;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Parser)]
#[clap(version)]
pub struct CLI<Flags, Options, Command>
where
	Flags: Parser + Args,
	Options: Parser + Args,
	Command: Parser + Subcommand,
{
	/// Les drapeaux.
	#[clap(flatten)]
	pub flags: Flags,
	/// Les options.
	#[clap(flatten)]
	pub options: Options,
	/// Le commande.
	#[clap(subcommand)]
	pub command: Option<Command>,
}

#[derive(Debug)]
#[derive(Parser)]
pub struct EmptyFlags {}

#[derive(Debug)]
#[derive(Parser)]
pub struct EmptyOptions {}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Parser)]
pub enum EmptyCommand {}
