/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod error;
mod export;

pub use self::export::*;

// --------- //
// Structure //
// --------- //

#[allow(clippy::upper_case_acronyms)]
pub struct GUI;

// -------------- //
// Implémentation //
// -------------- //

impl GUI {
	pub fn launch() -> Result<(), Error> {
		Ok(())
	}
}
