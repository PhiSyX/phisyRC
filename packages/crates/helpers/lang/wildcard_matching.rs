/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use regex::RegexBuilder;

use crate::string::Escape;

// ----- //
// Trait //
// ----- //

pub trait WildcardMatching: AsRef<str> {
	/// Compare deux chaînes de caractères avec l'utilisation de caractères
	/// génériques.
	///
	/// Les caractères génériques sont des caractères spéciaux, comme '*'
	/// ou '?'.
	///   - '*' : correspond à aucun caractère ou plusieurs caractères,
	///     n'importe lesquels.
	///   - '?' : correspond à un seul caractère, n'importe lequel.
	///
	/// wm = Wildcard Matching
	fn iswm(&self, pattern: &str) -> bool {
		self.iswm_exclude_special_chars(pattern, [])
	}

	/// Semblable à `iswm`, mais peut exclure certains caractères spéciaux
	/// d'être échappé.
	fn iswm_exclude_special_chars<const E: usize>(
		&self,
		pattern: &str,
		exclude_chars: [char; E],
	) -> bool {
		let build_regexp = pattern
			.escape_all_regexp_chars_excludes(exclude_chars)
			.replace(r"\?", ".")
			.replace(r"\*", ".*");

		let regexp = RegexBuilder::new(&build_regexp)
			.case_insensitive(true)
			.build()
			.unwrap();

		regexp.is_match(self.as_ref())
	}

	/// Compare deux chaînes de caractères avec l'utilisation de caractères
	/// génériques.
	///
	/// Les caractères génériques sont des caractères spéciaux, comme '*'
	/// ou '?'.
	///   - '*' : correspond à aucun caractère ou plusieurs caractères,
	///     n'importe lesquels.
	///   - '?' : correspond à un seul caractère, n'importe lequel.
	///
	/// wm = Wildcard Matching
	/// cs = Case Sensitive
	fn iswmcs(&self, pattern: &str) -> bool {
		self.iswmcs_exclude_special_chars(pattern, [])
	}

	/// Semblable à `iswm_exclude_special_chars`.
	///
	/// cs = Case Sensitive
	fn iswmcs_exclude_special_chars<const E: usize>(
		&self,
		pattern: &str,
		exclude_chars: [char; E],
	) -> bool {
		let build_regexp = pattern
			.escape_all_regexp_chars_excludes(exclude_chars)
			.replace(r"\?", ".")
			.replace(r"\*", ".*");

		let regexp = RegexBuilder::new(&build_regexp)
			.case_insensitive(false)
			.build()
			.unwrap();

		regexp.is_match(self.as_ref())
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

// ---- //
// Test //
// ---- //

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_iswm_simple() {
		assert!("test".iswm("test"));
		assert!("TEST".iswm("test"));
		assert!(!"test".iswm("failed"));
		assert!("test".iswm("tESt"));

		assert!("TEST".iswmcs("TEST"));
		assert!(!"TEST".iswmcs("test"));
		assert!("tESt".iswmcs("tESt"));
		assert!(!"TEST".iswmcs("tESt"));
	}

	#[test]
	fn test_iswm_asterisk() {
		assert!("test".iswm("t*st"));
		assert!(!"t*st".iswm("test"));

		assert!("PhiSyX[absent]".iswm("PhiSyX[*]"));
		assert!("PhiSyX[[[][][]]]".iswm("PhiSyX[*]"));
		assert!("PhiSyX[occupe]".iswm("PhiSyX[occup*]"));
	}

	#[test]
	fn test_iswm_question_mark() {
		assert!("test".iswm("t?st"));
		assert!(!"t?st".iswm("test"));

		assert!(!"PhiSyX[absent]".iswm("PhiSyX[absent?]"));
		assert!("PhiSyX[absente]".iswm("PhiSyX[absent?]"));
		assert!("PhiSyX[occupée]".iswm("PhiSyX[occup?e]"));
		assert!("PhiSyX[bot]".iswm("PhiSyX[???]"));
		assert!("PhiSyX[BOT]".iswm("PhiSyX[???]"));
		assert!("PhiSyX[BOT]".iswmcs("PhiSyX[???]"));
		assert!(!"PhiSyX[bot]".iswm("PhiSyX[??]"));
	}

	#[test]
	fn test_iswm_exclude() {
		const PAT: &str = "*|????[fm]";
		assert!(!"PhiSyX|24ITm__".iswm_exclude_special_chars(PAT, []));

		assert!("PhiSyX|24ITm__".iswm_exclude_special_chars(PAT, ['[', ']']));
		assert!("`PhiSyX|24ITf__".iswm_exclude_special_chars(PAT, ['[', ']']));

		assert!(!"PhiSyX".iswm_exclude_special_chars(PAT, ['[', ']']));

		assert!(
			!"`PhiSyX|24ITM__".iswmcs_exclude_special_chars(PAT, ['[', ']'])
		);
		assert!(
			!"`PhiSyX|24ITF__".iswmcs_exclude_special_chars(PAT, ['[', ']'])
		);
	}

	#[test]
	fn test_iswm_type() {
		let mut build_pattern = String::new();

		build_pattern.push('t');
		build_pattern.push('?');
		build_pattern.push('s');
		build_pattern.push('t');

		assert!(String::from("test").iswm(&build_pattern));
		assert!(std::borrow::Cow::from("test").iswm("t?st"));
	}
}
