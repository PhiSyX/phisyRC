/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::fmt;

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum EnvError {
	/// La variable d'environnement est mal formée.
	BadFormat(&'static str),

	/// La variable d'environnement est manquante.
	Missing(&'static str),
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for EnvError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::Missing(name) => {
					format!(
						"La variable d'environnement '{}' est manquante.",
						name
					)
				}
				| Self::BadFormat(name) => {
					format!(
						"Impossible d'analyser la variable d'environnement '{}'.",
						name
					)
				}
			}
		)
	}
}
