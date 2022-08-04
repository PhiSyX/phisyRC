/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use irc::IRC;

pub struct IrcDaemon;

impl IrcDaemon {
	pub fn spawn() {
		println!("Lance le réseau IRC en tâche de fond.");

		IRC::run();
	}
}
