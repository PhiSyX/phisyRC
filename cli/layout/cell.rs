/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::{borrow::Cow, cmp};

use super::Alignment;

#[derive(Debug)]
#[derive(Clone)]
pub struct Cell<'d> {
	pub(crate) alignment: Alignment,
	pub(crate) colspan: usize,
	data: Cow<'d, str>,
	padding: bool,
}

impl<'d> Cell<'d> {
	pub fn new(data: impl ToString) -> Self {
		Self {
			alignment: Alignment::Left,
			colspan: 1,
			data: data.to_string().into(),
			padding: true,
		}
	}

	pub fn with_alignment(mut self, alignment: Alignment) -> Self {
		self.alignment = alignment;
		self
	}
}

impl<'d> Cell<'d> {
	pub(crate) fn width(&self) -> usize {
		self.wrapped_content(std::usize::MAX)
			.iter()
			.fold(0, |max, s| cmp::max(max, str_len(s)))
	}

	pub(crate) fn split_width(&self) -> f32 {
		self.width() as f32 / self.colspan as f32
	}

	pub(crate) fn min_width(&self) -> usize {
		let max_char_width = self
			.data
			.chars()
			.fold(0, |max, ch| cmp::max(max, ch.len_utf8()));

		if self.padding {
			max_char_width + ' '.len_utf8() * 2
		} else {
			max_char_width
		}
	}

	pub(crate) fn wrapped_content(&self, width: usize) -> Vec<String> {
		let pad_char = if self.padding { ' ' } else { '\0' };

		let mut output = Vec::default();

		let mut temporary_buffer = String::default();

		temporary_buffer.push(pad_char);

		for ch in self.data.chars() {
			if str_len(&temporary_buffer) >= width - pad_char.len_utf8()
				|| ch == '\n'
			{
				temporary_buffer.push(pad_char);
				output.push(temporary_buffer);
				temporary_buffer = String::new();
				temporary_buffer.push(pad_char);

				if ch == '\n' {
					continue;
				}
			}

			temporary_buffer.push(ch);
		}

		temporary_buffer.push(pad_char);

		output.push(temporary_buffer);

		output
	}
}

// TODO(phisyx): strip ansi
pub fn str_len(string: &str) -> usize {
	string.len()
}
