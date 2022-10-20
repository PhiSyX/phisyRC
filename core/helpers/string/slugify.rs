/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use deunicode::deunicode_char;

// --------- //
// Interface //
// --------- //

pub trait Slugify: AsRef<str> {
	fn slugify(&self) -> String {
		let mut bytes = Vec::with_capacity(self.as_ref().len());
		let mut is_prev_dash = true;

		let mut add_byte = |x: u8| match x {
			| b'a'..=b'z' | b'0'..=b'9' => {
				is_prev_dash = false;
				bytes.push(x);
			}
			| b'A'..=b'Z' => {
				is_prev_dash = false;
				bytes.push(x - b'A' + b'a');
			}
			| b'\'' => {}
			| _ if !is_prev_dash => {
				bytes.push(b'-');
				is_prev_dash = true;
			}
			| _ => {}
		};

		self.as_ref().chars().for_each(|ch| {
			if ch.is_ascii() {
				(add_byte)(ch as u8);
			} else {
				deunicode_char(ch)
					.unwrap_or("-")
					.as_bytes()
					.iter()
					.for_each(|&unit| (add_byte)(unit));
			}
		});

		let mut slug = unsafe { String::from_utf8_unchecked(bytes) };
		if slug.ends_with('-') {
			slug.pop();
		}
		slug.shrink_to_fit();
		slug
	}
}
