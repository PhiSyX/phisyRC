/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::path::PathBuf;

use crate::{app::EmptyFlags, CLI};

pub type CommandServer =
	CLI<EmptyFlags, CommandServerOptions, SubCommandServer>;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(clap::Parser)]
pub struct CommandServerOptions {
	/// Ouvre les connexions aux serveurs (TCP/UDP, WS, IRC) en tâche de fond.
	#[clap(short, long)]
	pub daemon: bool,

	/// Fichier de configuration server.
	#[clap(long, parse(from_os_str))]
	pub config: Option<PathBuf>,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(clap::Parser)]
pub enum SubCommandServer {
	/// Redémarre les serveurs IRC.
	///
	/// Cette commande ferme les connexions des serveurs IRC connectés
	/// pour ensuite les ré-ouvrir.
	Restart { id: Option<String> },

	/// Met à jour les fichiers de configurations des serveurs IRC.
	///
	/// Cette commande NE DOIT PAS fermer la connexion aux serveurs IRC
	/// connectés (ni ouvrir la connexion s'ils ne sont pas connectés).
	Rehash { id: Option<String> },
}
