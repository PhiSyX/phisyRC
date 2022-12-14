/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::fmt;

// ---- //
// Type //
// ---- //

type VariableName = &'static str;

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
pub enum Error {
	/// La variable d'environnement est mal formée.
	BadFormat(VariableName),
	/// La variable d'environnement est manquante.
	NotFound(VariableName),
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let reason = match self {
			| Self::BadFormat(name) => format!(
				"Impossible d'analyser la variable d'environnement « {} ».",
				name
			),
			| Self::NotFound(name) => {
				format!(
					"La variable d'environnement « {} » est manquante.",
					name
				)
			}
		};

		write!(f, "{reason}")
	}
}
