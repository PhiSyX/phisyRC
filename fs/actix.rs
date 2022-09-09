/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::path::PathBuf;

// --------- //
// Structure //
// --------- //

pub struct ActixFileLoader;

// -------------- //
// Implémentation //
// -------------- //

impl ActixFileLoader {
	pub fn load(
		mount_path: &str,
		serve_from: impl Into<PathBuf>,
	) -> actix_files::Files {
		actix_files::Files::new(mount_path, serve_from).prefer_utf8(true)
	}
}
