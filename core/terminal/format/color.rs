/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::format::{format_text, PREFIX_FORMAT, SUFFIX_FORMAT};

// --------- //
// Interface //
// --------- //

/// Cette interface est implémentée par le prototype des chaines de
/// caractères.
pub trait Interface: AsRef<str> {
	/// Applique une couleur RGB de premier plan à un texte.
	fn rgb(&self, r: u8, g: u8, b: u8) -> String {
		let color = Color::Rgb(r, g, b);
		color.to_rgb_fg_string(self)
	}

	/// Applique une couleur RGB d'arrière plan à un texte.
	fn bg_rgb(&self, r: u8, g: u8, b: u8) -> String {
		let color = Color::Rgb(r, g, b);
		color.to_rgb_bg_string(self)
	}

	/// Applique une couleur HEX de premier plan à un texte.
	fn hex(&self, hex: usize) -> String {
		let (r, g, b) = (hex >> 16, hex >> 8 & 255, hex & 255);
		let color = Color::Rgb(r as u8, g as u8, b as u8);
		color.to_rgb_fg_string(self)
	}

	/// Applique une couleur HEX d'arrière plan à un texte.
	fn bg_hex(&self, hex: usize) -> String {
		let (r, g, b) = (hex >> 16, hex >> 8 & 255, hex & 255);
		let color = Color::Rgb(r as u8, g as u8, b as u8);
		color.to_rgb_bg_string(self)
	}

	/// Applique la couleur noire en premier plan à un texte.
	fn black(&self) -> String {
		let black = Color::Black;
		black.to_fg_string(self)
	}

	/// Applique la couleur noire en arrière plan à un texte.
	fn bg_black(&self) -> String {
		let black = Color::Black;
		black.to_bg_string(self)
	}

	/// Applique la couleur blanche claire en premier plan à un texte.
	fn bright_black(&self) -> String {
		let black = Color::BrightBlack;
		black.to_fg_string(self)
	}

	/// Applique la couleur blanche claire en arrière plan à un texte.
	fn bright_bg_black(&self) -> String {
		let black = Color::BrightBlack;
		black.to_bg_string(self)
	}

	/// Applique la couleur rouge en premier plan à un texte.
	fn red(&self) -> String {
		let red = Color::Red;
		red.to_fg_string(self)
	}

	/// Applique la couleur rouge en arrière plan à un texte.
	fn bg_red(&self) -> String {
		let red = Color::Red;
		red.to_bg_string(self)
	}

	/// Applique la couleur rouge claire en premier plan à un texte.
	fn bright_red(&self) -> String {
		let red = Color::BrightRed;
		red.to_fg_string(self)
	}

	/// Applique la couleur rouge claire en arrière plan à un texte.
	fn bright_bg_red(&self) -> String {
		let red = Color::BrightRed;
		red.to_bg_string(self)
	}

	/// Applique la couleur verte en premier plan à un texte.
	fn green(&self) -> String {
		let green = Color::Green;
		green.to_fg_string(self)
	}

	/// Applique la couleur verte en arrière plan à un texte.
	fn bg_green(&self) -> String {
		let green = Color::Green;
		green.to_bg_string(self)
	}

	/// Applique la couleur verte claire en premier plan à un texte.
	fn bright_green(&self) -> String {
		let green = Color::BrightGreen;
		green.to_fg_string(self)
	}

	/// Applique la couleur verte claire en arrière plan à un texte.
	fn bright_bg_green(&self) -> String {
		let green = Color::BrightGreen;
		green.to_bg_string(self)
	}

	/// Applique la couleur jaune en premier plan à un texte.
	fn yellow(&self) -> String {
		let yellow = Color::Yellow;
		yellow.to_fg_string(self)
	}

	/// Applique la couleur jaune en arrière plan à un texte.
	fn bg_yellow(&self) -> String {
		let yellow = Color::Yellow;
		yellow.to_bg_string(self)
	}

	/// Applique la couleur jaune claire en premier plan à un texte.
	fn bright_yellow(&self) -> String {
		let yellow = Color::BrightYellow;
		yellow.to_fg_string(self)
	}

	/// Applique la couleur jaune claire en arrière plan à un texte.
	fn bright_bg_yellow(&self) -> String {
		let yellow = Color::BrightYellow;
		yellow.to_bg_string(self)
	}

	/// Applique la couleur bleue en premier plan à un texte.
	fn blue(&self) -> String {
		let blue = Color::Blue;
		blue.to_fg_string(self)
	}

	/// Applique la couleur bleue en arrière plan à un texte.
	fn bg_blue(&self) -> String {
		let blue = Color::Blue;
		blue.to_bg_string(self)
	}

	/// Applique la couleur bleue claire en premier plan à un texte.
	fn bright_blue(&self) -> String {
		let blue = Color::BrightBlue;
		blue.to_fg_string(self)
	}

	/// Applique la couleur bleue claire en arrière plan à un texte.
	fn bright_bg_blue(&self) -> String {
		let blue = Color::BrightBlue;
		blue.to_bg_string(self)
	}

	/// Applique la couleur magenta en premier plan à un texte.
	fn magenta(&self) -> String {
		let magenta = Color::Magenta;
		magenta.to_fg_string(self)
	}

	/// Applique la couleur magenta en arrière plan à un texte.
	fn bg_magenta(&self) -> String {
		let magenta = Color::Magenta;
		magenta.to_bg_string(self)
	}

	/// Applique la couleur magenta claire en premier plan à un texte.
	fn bright_magenta(&self) -> String {
		let magenta = Color::BrightMagenta;
		magenta.to_fg_string(self)
	}

	/// Applique la couleur magenta claire en arrière plan à un texte.
	fn bright_bg_magenta(&self) -> String {
		let magenta = Color::BrightMagenta;
		magenta.to_bg_string(self)
	}

	/// Applique la couleur cyan en premier plan à un texte.
	fn cyan(&self) -> String {
		let cyan = Color::Cyan;
		cyan.to_fg_string(self)
	}

	/// Applique la couleur cyan en arrière plan à un texte.
	fn bg_cyan(&self) -> String {
		let cyan = Color::Cyan;
		cyan.to_bg_string(self)
	}

	/// Applique la couleur cyan claire en premier plan à un texte.
	fn bright_cyan(&self) -> String {
		let cyan = Color::BrightCyan;
		cyan.to_fg_string(self)
	}

	/// Applique la couleur cyan claire en arrière plan à un texte.
	fn bright_bg_cyan(&self) -> String {
		let cyan = Color::BrightCyan;
		cyan.to_bg_string(self)
	}

	/// Applique la couleur grise en premier plan à un texte.
	fn white(&self) -> String {
		let white = Color::White;
		white.to_fg_string(self)
	}

	/// Applique la couleur grise en arrière plan à un texte.
	fn bg_white(&self) -> String {
		let white = Color::White;
		white.to_bg_string(self)
	}

	/// Applique la couleur grise claire en premier plan à un texte.
	fn bright_white(&self) -> String {
		let white = Color::BrightWhite;
		white.to_fg_string(self)
	}

	/// Applique la couleur grise claire en arrière plan à un texte.
	fn bright_bg_white(&self) -> String {
		let white = Color::BrightWhite;
		white.to_bg_string(self)
	}

	/// Applique la couleur noire en premier plan à un texte.
	fn gray(&self) -> String {
		let gray = Color::Gray;
		gray.to_fg_string(self)
	}

	/// Applique la couleur noire en arrière plan à un texte.
	fn bg_gray(&self) -> String {
		let gray = Color::Gray;
		gray.to_bg_string(self)
	}
}

// ----------- //
// Énumération //
// ----------- //

/// Liste des couleurs supportées du terminal.
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
#[non_exhaustive]
pub enum Color {
	/// Contient une couleur RGB.
	Rgb(
		/// Valeur de rouge. (red)
		u8,
		/// Valeur de verte. (green)
		u8,
		/// Valeur de bleue. (blue)
		u8,
	),

	/// Couleur noire.
	Black,
	/// Couleur noire claire.
	BrightBlack,

	/// Couleur rouge.
	Red,
	/// Couleur rouge claire.
	BrightRed,

	/// Couleur verte.
	Green,
	/// Couleur verte claire.
	BrightGreen,

	/// Couleur jaune.
	Yellow,
	/// Couleur jaune claire.
	BrightYellow,

	/// Couleur bleue.
	Blue,
	/// Couleur bleue claire.
	BrightBlue,

	/// Couleur magenta.
	Magenta,
	/// Couleur magenta claire.
	BrightMagenta,

	/// Couleur cyan.
	Cyan,
	/// Couleur cyan claire.
	BrightCyan,

	/// Couleur blanche.
	White,
	/// Couleur blanche. (ALT)
	BrightWhite,

	/// Couleur grise.
	Gray,
}

// -------------- //
// Implémentation //
// -------------- //

impl Color {
	/// Constante d'arrière plan.
	const BACKGROUND: u8 = 49;
	/// Constante de premier plan.
	const FOREGROUND: u8 = 39;

	/// Retourne un tuple de taille 2 (code ANSI, couleur de premier plan)
	pub fn fg(self) -> (u8, u8) {
		let color = match self {
			| Self::Black => 30,
			| Self::BrightBlack => 90,

			| Self::Red => 31,
			| Self::BrightRed => 91,

			| Self::Green => 32,
			| Self::BrightGreen => 92,

			| Self::Yellow => 33,
			| Self::BrightYellow => 93,

			| Self::Blue => 34,
			| Self::BrightBlue => 94,

			| Self::Magenta => 35,
			| Self::BrightMagenta => 95,

			| Self::Cyan => 36,
			| Self::BrightCyan => 96,

			| Self::White => 37,
			| Self::BrightWhite => 97,

			| Self::Gray => 90,

			| Self::Rgb(r, g, b) => {
				let r = r * 40 / 255;
				let g = g * 40 / 255;
				let b = b * 40 / 255;

				r + g + b
			}
		};

		(color, Self::FOREGROUND)
	}

	/// Retourne un tuple de taille 2 (code ANSI, couleur d'arrière plan)
	pub fn bg(self) -> (u8, u8) {
		let color = match self {
			| Self::Black => 40,
			| Self::BrightBlack => 100,

			| Self::Red => 41,
			| Self::BrightRed => 101,

			| Self::Green => 42,
			| Self::BrightGreen => 102,

			| Self::Yellow => 43,
			| Self::BrightYellow => 103,

			| Self::Blue => 44,
			| Self::BrightBlue => 104,

			| Self::Magenta => 45,
			| Self::BrightMagenta => 105,

			| Self::Cyan => 46,
			| Self::BrightCyan => 106,

			| Self::White => 47,
			| Self::BrightWhite => 107,

			| Self::Gray => 90,

			| Self::Rgb(r, g, b) => {
				let r = r * 40 / 255;
				let g = g * 40 / 255;
				let b = b * 40 / 255;

				r + g + b
			}
		};

		(color, Self::BACKGROUND)
	}

	/// Retourne un tuple de taille 3 (R, G, B)
	pub fn rgb(self) -> (u8, u8, u8) {
		use Color::*;

		match self {
			| Black => (0, 0, 0),
			| BrightBlack => (0, 0, 0),

			| Red => (255, 0, 0),
			| BrightRed => (255, 0, 0),

			| Green => (0, 255, 0),
			| BrightGreen => (0, 255, 0),

			| Yellow => (255, 255, 0),
			| BrightYellow => (255, 255, 0),

			| Blue => (0, 0, 255),
			| BrightBlue => (0, 0, 255),

			| Magenta => (255, 0, 255),
			| BrightMagenta => (255, 0, 255),

			| Cyan => (0, 255, 255),
			| BrightCyan => (0, 255, 255),

			| White => (255, 255, 255),
			| BrightWhite => (255, 255, 255),

			| Gray => (128, 128, 128),

			| Rgb(r, g, b) => (r, g, b),
		}
	}

	// ANSI

	/// Applique la couleur de premier plan associée à un texte.
	pub fn to_fg_string(self, text: impl AsRef<str>) -> String {
		let (open, close) = self.fg();
		colorize(text, open, close)
	}

	/// Applique la couleur d'arrière plan associée à un texte.
	pub fn to_bg_string(self, text: impl AsRef<str>) -> String {
		let (open, close) = self.bg();
		colorize(text, open, close)
	}

	// RGB

	/// Applique la couleur RGB de premier plan associée à un texte.
	pub fn to_rgb_fg_string(self, text: impl AsRef<str>) -> String {
		colorize_rgb(text, Self::FOREGROUND - 1, Self::FOREGROUND, self)
	}

	/// Applique la couleur RGB d'arrière plan associée à un texte.
	pub fn to_rgb_bg_string(self, text: impl AsRef<str>) -> String {
		colorize_rgb(text, Self::BACKGROUND - 1, Self::BACKGROUND, self)
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

/// Retourne un tuple de taille 3 contenant les unités RGB d'une
/// énumération [Color].
impl From<Color> for (u8, u8, u8) {
	fn from(color: Color) -> Self {
		color.rgb()
	}
}

/// Retourne la couleur de premier plan et d'arrière plan au format ANSI
///
/// ((<fg_open>, <fg_close>), (<bg_open>, <bg_close>))
impl From<Color> for ((u8, u8), (u8, u8)) {
	fn from(val: Color) -> Self {
		(val.fg(), val.bg())
	}
}

/// Voir la documentation de [format_text()] pour plus d'informations.
fn colorize(
	text: impl AsRef<str>,
	open: impl Into<u8>,
	close: impl Into<u8>,
) -> String {
	format_text(text, open, close)
}

/// Voir la documentation de [format_text()] pour plus d'informations.
fn colorize_rgb(
	text: impl AsRef<str>,
	open: impl Into<u8>,
	close: impl Into<u8>,
	rgb: Color,
) -> String {
	let (r, g, b) = rgb.into();
	format!(
		"{pre}{open};2;{r};{g};{b}{suf}{text}{pre}{close}{suf}",
		open = open.into(),
		text = text.as_ref(),
		close = close.into(),
		r = r,
		g = g,
		b = b,
		pre = PREFIX_FORMAT,
		suf = SUFFIX_FORMAT,
	)
}

// ---- //
// Test //
// ---- //

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_colorize_rgb() {
		assert_eq!(
			colorize_rgb("Hello PhiSyX", 38, 48, Color::Rgb(255, 0, 0)),
			"\x1b[38;2;255;0;0mHello PhiSyX\x1b[48m",
		);
	}

	#[test]
	fn test_fg_color() {
		let black = Color::Black;
		assert_eq!(black.fg(), (30, 39));

		let green = Color::Green;
		assert_eq!(green.fg(), (32, 39));
	}

	#[test]
	fn test_bg_color() {
		let red = Color::Red;
		assert_eq!(red.bg(), (41, 49));

		let gray = Color::Gray;
		assert_eq!(gray.bg(), (90, 49));
	}

	#[test]
	fn test_color_into() {
		let blue = Color::Black;
		let (fg, bg) = blue.into();
		assert_eq!(fg, (30, 39));
		assert_eq!(bg, (40, 49));
	}

	#[test]
	fn test_frame_color() {
		assert_eq!("Hello PhiSyX".red(), "\x1b[31mHello PhiSyX\x1b[39m");
		assert_eq!("Hello PhiSyX".green(), "\x1b[32mHello PhiSyX\x1b[39m");
		assert_eq!("Hello PhiSyX".blue(), "\x1b[34mHello PhiSyX\x1b[39m");
	}
}
