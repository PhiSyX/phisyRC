/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod error;
mod export;
mod interface;
mod parser;

pub use self::export::*;

pub mod app {
	use super::{Error as EnvError, EnvInterface, EnvParser};

	// --------- //
	// Structure //
	// --------- //

	#[derive(Debug)]
	#[derive(phisyrc::Env)]
	#[allow(non_camel_case_types)]
	pub struct phisyrc_env {
		#[var(key = "DEBUG", default = "*")]
		pub debug_filter: String,

		#[var(key = "APP_SECRET_KEY")]
		pub app_secret_key: String,
	}
}
