/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod color;

pub use self::color::{Color, Interface};

pub(crate) const PREFIX_FORMAT: &str = "\x1b[";
pub(crate) const SUFFIX_FORMAT: &str = "m";

/// Construit le bon format de chaîne pour l'afficher dans le terminal.
pub fn format_text(
	text: impl AsRef<str>,
	open: impl Into<u8>,
	close: impl Into<u8>,
) -> String {
	format!(
		"{pre}{open}{suf}{text}{pre}{close}{suf}",
		open = open.into(),
		text = text.as_ref(),
		close = close.into(),
		pre = PREFIX_FORMAT,
		suf = SUFFIX_FORMAT,
	)
}
