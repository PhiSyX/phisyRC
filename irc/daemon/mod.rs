/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::path::Path;

use super::Irc;
use crate::output::IrcResult;

// --------- //
// Structure //
// --------- //

pub struct IrcDaemon;

// -------------- //
// Implémentation //
// -------------- //

impl IrcDaemon {
	pub async fn spawn(config_file: impl AsRef<Path>) -> IrcResult<()> {
		println!("Lance le réseau IRC en tâche de fond.");

		Irc::run(config_file).await
	}
}
