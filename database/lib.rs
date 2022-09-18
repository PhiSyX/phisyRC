/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod export;
mod types;

pub use self::export::*;

pub mod app {
	use super::*;

	#[allow(non_camel_case_types)]
	pub struct phisyrc_db {}

	impl phisyrc_db {
		/// Crée la connexion à une base de donnée.
		pub fn new() -> Result<Self, Error> {
			Ok(Self {})
		}
	}
}
