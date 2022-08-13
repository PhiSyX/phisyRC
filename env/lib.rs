/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod error;
mod interface;
mod parser;

pub use self::{error::EnvError, interface::EnvInterface};

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(phisyrc::Env)]
#[allow(non_camel_case_types)]
pub struct phisyrc_env {
	#[var(key = "DEBUG", default = "*")]
	pub debug_filter: String,

	#[var(key = "GUI", default = "flutter")]
	pub gui_to_use: String,
}
