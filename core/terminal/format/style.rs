/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::format::format_text;

// --------- //
// Interface //
// --------- //

pub trait Interface: AsRef<str> {
	fn reset(&self) -> String {
		let reset = Style::Reset;
		reset.to_style(self)
	}

	fn bold(&self) -> String {
		let bold = Style::Bold;
		bold.to_style(self)
	}

	fn dim(&self) -> String {
		let dim = Style::Dim;
		dim.to_style(self)
	}

	fn italic(&self) -> String {
		let italic = Style::Italic;
		italic.to_style(self)
	}

	fn underline(&self) -> String {
		let underline = Style::Underline;
		underline.to_style(self)
	}

	fn blink(&self) -> String {
		let blink = Style::Blink;
		blink.to_style(self)
	}

	fn reverse(&self) -> String {
		let reverse = Style::Reverse;
		reverse.to_style(self)
	}

	fn hidden(&self) -> String {
		let hidden = Style::Hidden;
		hidden.to_style(self)
	}

	fn strikethrough(&self) -> String {
		let strikethrough = Style::StrikeThrough;
		strikethrough.to_style(self)
	}
}

// ----------- //
// Énumération //
// ----------- //

#[non_exhaustive]
pub enum Style {
	Reset,

	Bold,
	Dim,
	Italic,
	Underline,
	Blink,
	Reverse,
	Hidden,
	StrikeThrough,
}

// -------------- //
// Implémentation //
// -------------- //

impl Style {
	pub fn to_style(self, text: impl AsRef<str>) -> String {
		let (open, close): (u8, u8) = self.into();
		stylize(text, open, close)
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

/// Retourne un tuple contenant les formats de début et de fin de la
/// séquence de caractères de style.
impl From<Style> for (u8, u8) {
	fn from(style: Style) -> Self {
		use Style::*;

		match style {
			| Reset => (0, 0),
			| Bold => (1, 22),
			| Dim => (2, 22),
			| Italic => (3, 23),
			| Underline => (4, 24),
			| Blink => (5, 25),
			| Reverse => (7, 27),
			| Hidden => (8, 28),
			| StrikeThrough => (9, 29),
		}
	}
}

// -------- //
// Fonction //
// -------- //

fn stylize(text: impl AsRef<str>, open: u8, close: u8) -> String {
	format_text(text, open, close)
}

// ---- //
// Test //
// ---- //

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_style() {
		let text = "Hello PhiSyX";
		let bold = Style::Bold;
		assert_eq!(bold.to_style(text), "\x1b[1mHello PhiSyX\x1b[22m");
	}

	#[test]
	fn test_style_str() {
		let text = "Hello PhiSyX";
		assert_eq!(text.underline(), "\x1b[4mHello PhiSyX\x1b[24m");
	}

	#[test]
	fn test_style_into() {
		let reverse = Style::Reverse;
		let res: (u8, u8) = reverse.into();
		assert_eq!(res, (7, 27));
	}
}
