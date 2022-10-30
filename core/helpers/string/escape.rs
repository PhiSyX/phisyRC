/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::borrow::Cow;

use regex::Regex;

// --------- //
// Interface //
// --------- //

pub trait Escape: AsRef<str> {
	// --------- //
	// Constante //
	// --------- //

	/// Caractère générique d'échappement.
	const ESCAPED_WITH: char = '\\';
	/// Les caractères spéciaux d'une expression régulière.
	const REGEXP_CHARS: &'static str = r"^$\.*+?()[]{}|";

	/// Échappe tous les caractères d'une chaîne par un seul caractère.
	fn escape_all_chars_with(&self, escape_with: char) -> String {
		Self::escape_all_chars_with_excepts_or_excludes(
			self,
			escape_with,
			[],
			[],
		)
	}

	/// Échappe les caractères d'une chaîne, sauf ceux spécifiés comme
	/// exceptions.
	fn escape_all_chars_with_excepts<const E: usize>(
		&self,
		escape_with: char,
		excepts_char: [char; E],
	) -> String {
		Self::escape_all_chars_with_excepts_or_excludes(
			self,
			escape_with,
			excepts_char,
			[],
		)
	}

	/// Échappe les caractères d'une chaîne, sauf ceux spécifiés comme
	/// exceptions. Les caractères d'exclusions seront retirés de la
	/// chaîne.
	fn escape_all_chars_with_excludes<const E: usize>(
		&self,
		escape_with: char,
		excludes_char: [char; E],
	) -> String {
		Self::escape_all_chars_with_excepts_or_excludes(
			self,
			escape_with,
			[],
			excludes_char,
		)
	}

	/// Échappe les caractères spéciaux (d'expression régulière) d'une
	/// chaîne de caractère.
	fn escape_all_regexp_chars(&'_ self) -> Cow<'_, str> {
		self.escape_all_regexp_chars_excludes([])
	}

	/// Voir [Self::escape_all_chars_with_excepts_or_excludes()]
	fn escape_all_regexp_chars_excludes<const E: usize>(
		&self,
		excludes_char: [char; E],
	) -> Cow<str> {
		let regexp = Self::escape_all_chars_with_excepts_or_excludes(
			Self::REGEXP_CHARS,
			Self::ESCAPED_WITH,
			[],
			excludes_char,
		);

		let chars_to_escape = Regex::new(&format!("[{regexp}]")).unwrap();

		chars_to_escape
			.replace_all(self.as_ref(), format!("{}$0", Self::ESCAPED_WITH))
	}

	/// Échappe les caractères d'une chaîne de caractère.
	fn escape_all_chars_with_excepts_or_excludes<
		const E1: usize,
		const E2: usize,
	>(
		this: impl AsRef<str>,
		escape_with: char,
		excepts_char: [char; E1],
		excludes_char: [char; E2],
	) -> String {
		let default_ch = char::default();
		this.as_ref()
			.chars()
			.filter(|ch| !excludes_char.contains(ch))
			.filter_map(|ch| {
				let then: [char; 2] = [default_ch, ch];
				let or_else: [char; 2] = [escape_with, ch];
				excepts_char.contains(&ch).then_some(then).or(Some(or_else))
			})
			.flatten()
			.filter(|ch| default_ch.ne(ch))
			.collect()
	}
}

// ---- //
// Test //
// ---- //

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn escape_chars() {
		assert_eq!("test".escape_all_chars_with('\\'), "\\t\\e\\s\\t");
		assert_eq!("test".escape_all_chars_with('-'), "-t-e-s-t");
	}

	#[test]
	fn escape_chars_excepts() {
		assert_eq!(
			"test".escape_all_chars_with_excepts('\\', ['e', 's']),
			"\\tes\\t"
		);
	}

	#[test]
	fn escape_chars_excludes() {
		assert_eq!(
			"test".escape_all_chars_with_excludes('\\', ['e', 's']),
			"\\t\\t"
		);
	}

	#[test]
	fn escape_regexp() {
		assert_eq!("test".escape_all_regexp_chars(), "test");
		assert_eq!("te*st".escape_all_regexp_chars(), r"te\*st");
	}

	#[test]
	fn escape_regexp_excludes() {
		assert_eq!("te*st".escape_all_regexp_chars_excludes(['*']), "te*st");
	}
}
