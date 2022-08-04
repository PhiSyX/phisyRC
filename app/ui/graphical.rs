/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::{env, io, process::Command};

use super::UInterface;

// --------- //
// Structure //
// --------- //

#[allow(clippy::upper_case_acronyms)]
pub struct GUI;

// -------------- //
// Implementation //
// -------------- //

#[async_trait::async_trait]
impl UInterface for GUI {
	async fn launch() -> io::Result<()> {
		if cfg!(debug_assertions) {
			let flutter_bin = env::var("FLUTTER_BIN")
				.unwrap_or_else(|_| "flutter".to_owned());

			Command::new(flutter_bin)
				.current_dir("app/ui/graphical")
				.arg("run")
				.arg("--device-id")
				.arg("windows")
				.spawn()
				.map(|_| ())
		} else {
			Ok(())
		}
	}
}
