/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::ops::Range;

use crypto::{digest::Digest, sha1::Sha1};

// --------- //
// Interface //
// --------- //

pub trait SHA1: AsRef<str> {
	fn sha1(&self) -> String {
		let mut hash = Sha1::new();
		hash.input_str(self.as_ref());
		hash.result_str()
	}

	fn sha1_sliced(&self, rng: Range<usize>) -> String {
		let hash = self.sha1();
		hash.get(rng).unwrap_or(&hash).to_owned()
	}
}

// ---- //
// Test //
// ---- //

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_sha1() {
		let s = "test";
		assert_eq!(s.sha1(), "a94a8fe5ccb19ba61c4c0873d391e987982fbbd3");
	}

	#[test]
	fn test_sha1_sliced() {
		assert_eq!("test".sha1_sliced(10..20), "b19ba61c4c");
	}
}
