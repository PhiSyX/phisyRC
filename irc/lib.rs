/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod daemon;

pub use self::daemon::*;

// --------- //
// Structure //
// --------- //

pub struct IRC;

// -------------- //
// Implémentation //
// -------------- //

impl IRC {
	pub fn run() {
		println!("Ouverture de la connexion aux serveurs IRC.");
	}
}
