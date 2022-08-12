/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::{env, io, process::Command, str::FromStr};

// --------- //
// Structure //
// --------- //

#[allow(clippy::upper_case_acronyms)]
pub struct GUI;

#[derive(Debug)]
#[derive(Default)]
pub enum TypeGui {
	#[default]
	Flutter,
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
	pub(crate) async fn launch(gui: TypeGui) -> io::Result<()> {
		if cfg!(debug_assertions) {
			match gui {
				| TypeGui::Flutter => Self::use_flutter(),
				| TypeGui::Tauri => Self::use_tauri(),
			}
		} else {
			Ok(())
		}
	}

	fn use_flutter() -> io::Result<()> {
		let flutter_bin =
			env::var("FLUTTER_BIN").unwrap_or_else(|_| "flutter".to_owned());

		Command::new(flutter_bin)
			.current_dir("app/ui/graphical")
			.arg("run")
			.arg("--device-id")
			.arg("windows")
			.spawn()
			.map(|_| ())
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
			| "flutter" => TypeGui::Flutter,
			| "tauri" => TypeGui::Tauri,
			| _ => return Err(Self::Err::Invalid),
		})
	}
}
