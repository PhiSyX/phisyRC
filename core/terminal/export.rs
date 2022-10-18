/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub use crossterm;
pub use tui;

use crate::format::Interface;

impl<S> Interface for S where S: AsRef<str> {}

pub trait EventLoop: 'static + Send {
	fn quit() -> Self;
}
