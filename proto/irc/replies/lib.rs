/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod commands;
mod export;
mod numeric;
mod text;

use core::fmt;

pub use self::export::*;

// ---- //
// Type //
// ---- //

pub type Result<T> = core::result::Result<T, Error>;

// --------- //
// Interface //
// --------- //

pub trait IncomingCommand<T> {
	/// Combien de paramètre la commande PEUT/DOIT prendre.
	fn can_take_parameters(cmd_s: impl AsRef<str>) -> usize;

	/// La commande est-elle valide?
	fn is_valid(&self) -> Result<T>;
}

// --------- //
// Structure //
// --------- //

pub enum Reply {
	Numeric(Numeric),
}

#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum Error {
	Numeric(Numeric),
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let err_s = match self {
			| Self::Numeric(n) => {
				format!("Erreur commande numérique: {n}")
			}
		};

		write!(f, "{err_s}")
	}
}
