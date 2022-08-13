/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Style {
	top: CornerStyle,
	pub(crate) right: char,
	bottom: CornerStyle,
	pub(crate) left: char,

	pub(crate) vertical: char,
	pub(crate) horizontal: char,
	pub(crate) intersection: char,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct CornerStyle {
	left: char,
	right: char,
	intersection: char,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
pub enum Alignment {
	Left,
	Right,
	Center,
}

#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
pub(crate) enum Position {
	First,
	Middle,
	Last,
}

// -------------- //
// Implémentation //
// -------------- //

impl Style {
	pub fn simple() -> Self {
		Self {
			top: CornerStyle::all('+'),
			bottom: CornerStyle::all('+'),

			left: '+',
			right: '+',

			intersection: '+',
			vertical: '|',
			horizontal: '-',
		}
	}

	pub fn thin() -> Self {
		Self {
			top: CornerStyle {
				left: '┌',
				right: '┐',
				intersection: '┬',
			},
			bottom: CornerStyle {
				left: '└',
				right: '┘',
				intersection: '┴',
			},

			left: '├',
			right: '┤',

			vertical: '│',
			horizontal: '─',
			intersection: '┼',
		}
	}

	pub fn rounded() -> Self {
		Self {
			top: CornerStyle {
				left: '╭',
				right: '╮',
				intersection: '┬',
			},

			bottom: CornerStyle {
				left: '╰',
				right: '╯',
				intersection: '┴',
			},

			left: '├',
			right: '┤',

			vertical: '│',
			horizontal: '─',
			intersection: '┼',
		}
	}

	pub fn blank() -> Self {
		Self {
			top: CornerStyle::all('\0'),
			bottom: CornerStyle::all('\0'),

			left: '\0',
			right: '\0',

			vertical: '\0',
			horizontal: '\0',
			intersection: '\0',
		}
	}
}

impl Style {
	pub(crate) fn start_for_position(&self, position: Position) -> char {
		match position {
			| Position::First => self.top.left,
			| Position::Middle => self.left,
			| Position::Last => self.bottom.left,
		}
	}

	pub(crate) fn end_for_position(&self, position: Position) -> char {
		match position {
			| Position::First => self.top.right,
			| Position::Middle => self.right,
			| Position::Last => self.bottom.right,
		}
	}

	pub(crate) fn intersect_for_position(&self, position: Position) -> char {
		match position {
			| Position::First => self.top.intersection,
			| Position::Middle => self.intersection,
			| Position::Last => self.bottom.intersection,
		}
	}

	pub(crate) fn merge_intersection_for_position(
		&self,
		top: char,
		bottom: char,
		position: Position,
	) -> char {
		if (top == self.horizontal || top == self.bottom.intersection)
			&& bottom == self.intersection
		{
			self.top.intersection
		} else if (top == self.intersection || top == self.top.intersection)
			&& bottom == self.horizontal
		{
			self.bottom.intersection
		} else if top == self.bottom.intersection && bottom == self.horizontal {
			self.horizontal
		} else {
			self.intersect_for_position(position)
		}
	}
}

impl CornerStyle {
	fn all(ch: char) -> Self {
		Self {
			left: ch,
			right: ch,
			intersection: ch,
		}
	}
}
