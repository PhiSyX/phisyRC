/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod commands;
mod export;
mod numeric;
mod text;

pub use self::export::*;

// ---- //
// Type //
// ---- //

pub type Result<T> = core::result::Result<T, Error>;

// --------- //
// Interface //
// --------- //

pub trait IncomingCommand<T> {
	fn can_take_parameters(cmd_s: impl AsRef<str>) -> usize;
	fn is_valid(&self) -> Result<T>;
}

// --------- //
// Structure //
// --------- //

pub enum Reply {
	Numeric(Numeric),
}

#[derive(Debug)]
pub enum Error {
	Numeric(Numeric),
}
