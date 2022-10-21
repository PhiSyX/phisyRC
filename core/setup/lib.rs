/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

#[cfg(feature = "database")]
mod database;
mod export;

#[cfg(feature = "logger")]
mod logger;

pub use self::export::*;

// --------- //
// Interface //
// --------- //

pub trait SetupCliInterface {
	fn process_env(&self) -> cli::ProcessEnv;
}

pub trait SetupEnvInterface {
	fn debug_filter(&self) -> String;
}

// ---- //
// Type //
// ---- //

pub(crate) type Result<T, E = Box<dyn std::error::Error>> =
	core::result::Result<T, E>;
