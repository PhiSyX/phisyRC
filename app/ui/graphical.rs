/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;
use std::{io, str::FromStr};

// --------- //
// Structure //
// --------- //

#[allow(clippy::upper_case_acronyms)]
pub struct GUI;

// NOTE(phisyx): on pourrait avoir plusieurs types d'interfaces graphiques.
#[derive(Debug)]
#[derive(Default)]
#[derive(Copy, Clone)]
pub enum TypeGui {
	#[default]
	Tauri,
}

#[derive(Debug)]
pub enum TypeGuiError {
	Invalid,
}

// -------------- //
// Implementation //
// -------------- //

impl GUI {
	pub(crate) fn launch(gui: TypeGui) -> io::Result<()> {
		if cfg!(debug_assertions) {
			match gui {
				| TypeGui::Tauri => Self::use_tauri(),
			}
		} else {
			Ok(())
		}
	}

	fn use_tauri() -> io::Result<()> {
		Ok(())
	}
}

// -------------- //
// Implementation // -> Interface
// -------------- //

impl FromStr for TypeGui {
	type Err = TypeGuiError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s.to_ascii_lowercase().as_str() {
			| "tauri" => TypeGui::Tauri,
			| _ => return Err(Self::Err::Invalid),
		})
	}
}

impl fmt::Display for TypeGuiError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			| Self::Invalid => write!(f, "Type de GUI invalide"),
		}
	}
}
