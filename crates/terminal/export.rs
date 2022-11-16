/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

pub use crossterm;
pub use tui;

impl<S> crate::format::color::Interface for S where S: AsRef<str> {}
impl<S> crate::format::style::Interface for S where S: AsRef<str> {}

pub trait EventLoop: 'static + Send + Sync {
	fn input(input: String) -> Self;

	fn quit() -> Self;
}
