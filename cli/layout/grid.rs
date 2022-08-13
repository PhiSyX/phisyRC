/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;

use super::{style::Position, Alignment, Cell, Row, Style};

#[derive(Clone, Debug)]
pub struct GridLayout<'d> {
	rows: Vec<Row<'d>>,
	style: Style,
	max_width: usize,
	max_widths: HashMap<usize, usize>,

	separate_rows: bool,
	has_top_boarder: bool,
	has_bottom_boarder: bool,
}

impl<'d> GridLayout<'d> {
	pub fn new() -> Self {
		Self {
			rows: Vec::new(),
			style: Style::blank(),
			max_width: std::usize::MAX,
			max_widths: HashMap::new(),

			separate_rows: true,
			has_top_boarder: true,
			has_bottom_boarder: true,
		}
	}

	pub fn with_style(mut self, style: Style) -> Self {
		self.style = style;
		self
	}

	pub fn without_boarder(mut self) -> Self {
		self.has_top_boarder = false;
		self.has_bottom_boarder = false;
		self
	}

	pub fn define_max_width(mut self, width: usize) -> Self {
		self.max_width = width;
		self
	}

	pub fn add_row(
		&mut self,
		cells: impl IntoIterator<Item = impl Into<Cell<'d>>>,
	) {
		let row = Row::new(cells);
		self.rows.push(row);
	}

	pub fn render(&self) -> String {
		let mut print_buffer = String::new();

		if self.rows.is_empty() {
			return print_buffer;
		}

		let max_widths = self.calculate_max_widths();
		let mut previous_separator = None;
		(0..self.rows.len()).for_each(|index| {
			let row_pos = if index == 0 {
				Position::First
			} else {
				Position::Middle
			};

			let separator = self.rows[index].generate_separator(
				&max_widths,
				&self.style,
				row_pos,
				previous_separator.as_ref(),
			);

			previous_separator.replace(separator.to_owned());

			if self.rows[index].separator
				&& ((index == 0 && self.has_top_boarder)
					|| index != 0 && self.separate_rows)
			{
				Self::buffer_line(&mut print_buffer, separator);
			}

			Self::buffer_line(
				&mut print_buffer,
				&self.rows[index].format(&max_widths, &self.style),
			);
		});

		if self.has_bottom_boarder {
			let separator = self.rows.last().unwrap().generate_separator(
				&max_widths,
				&self.style,
				Position::Last,
				None,
			);

			Self::buffer_line(&mut print_buffer, separator);
		}

		print_buffer
	}
}

impl<'d> GridLayout<'d> {
	fn calculate_max_widths(&self) -> Vec<usize> {
		let total_columns = self
			.rows
			.iter()
			.fold(0, |n, row| core::cmp::max(row.total_columns(), n));

		let (_, mut max_widths) = self.rows.iter().fold(
			(vec![0; total_columns], vec![0; total_columns]),
			|acc, row| {
				let column_widths = row.split_column();

				(0..column_widths.len()).fold(
					(acc.0, acc.1),
					|(mut min, mut max), index| {
						min[index] =
							core::cmp::max(min[index], column_widths[index].1);

						let mut max_width = *self
							.max_widths
							.get(&index)
							.unwrap_or(&self.max_width);

						max_width = core::cmp::max(min[index], max_width);

						max[index] = core::cmp::min(
							max_width,
							core::cmp::max(
								max[index],
								column_widths[index].0 as usize,
							),
						);

						(min, max)
					},
				)
			},
		);

		self.rows.iter().for_each(|row| {
			let mut col_index = 0;

			row.cells.iter().for_each(|cell| {
				let mut total_col_width = 0;
				(col_index..col_index + cell.colspan).for_each(|i| {
					total_col_width += max_widths[i];
				});

				if cell.width() != total_col_width
					&& cell.alignment == Alignment::Center
					&& total_col_width as f32 % 2.0 <= 0.001
				{
					let mut max_col_width = self.max_width;
					if let Some(specific_width) =
						self.max_widths.get(&col_index)
					{
						max_col_width = *specific_width;
					}

					if max_widths[col_index] < max_col_width {
						max_widths[col_index] += 1;
					}
				}
				if cell.colspan > 1 {
					col_index += cell.colspan - 1;
				} else {
					col_index += 1;
				}
			});
		});

		max_widths
	}

	fn buffer_line(buffer: &mut String, line: impl AsRef<str>) {
		buffer.push_str(&format!("{}\n", line.as_ref()));
	}
}

impl<'d> Default for GridLayout<'d> {
	fn default() -> Self {
		GridLayout::new()
	}
}

impl<'d> ToString for GridLayout<'d> {
	fn to_string(&self) -> String {
		self.render()
	}
}
