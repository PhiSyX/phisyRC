/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::{fmt, result::Result as ResultResult};

// ---- //
// Type //
// ---- //

pub type Result<T> = ResultResult<T, Error>;

// ------------------------ //
// Erreurs de l'application //
// ------------------------ //

#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Error {
	IO(std::io::Error),
	Env(env::Error),
	GUI(gui::Error),
	IRC(irc::Error),
}

// -------------- //
// Implémentation // -> Gestion des erreurs (`?`)
// -------------- //

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Self::IO(err)
	}
}

impl From<env::Error> for Error {
	fn from(err: env::Error) -> Self {
		Self::Env(err)
	}
}

impl From<gui::Error> for Error {
	fn from(err: gui::Error) -> Self {
		Self::GUI(err)
	}
}

impl From<irc::Error> for Error {
	fn from(err: irc::Error) -> Self {
		Self::IRC(err)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"Erreur {}",
			match self {
				| Self::IO(io_err) => {
					format!("IO: {io_err}")
				}
				| Self::Env(env_err) => {
					format!("variable d'environnement: {env_err}")
				}
				| Self::GUI(gui_err) => {
					format!("de l'Interface Utilisateur Graphique: {gui_err}")
				}
				| Self::IRC(irc_err) => {
					format!("IRC: {irc_err}")
				}
			}
		)
	}
}
