/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod client;
mod server;

pub use self::{client::*, server::*};

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(clap::Parser)]
pub enum EmptyCommand {}

#[derive(Debug)]
#[derive(clap::Parser)]
pub enum Command {
	/// Donne accès aux options du client.
	Client(CommandClient),

	/// Donne accès aux options du serveur.
	Server(CommandServer),
}
