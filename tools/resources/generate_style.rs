/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod style;

use core::fmt;
use std::{
	collections::{HashMap, HashSet},
	env,
	fs::File as StdFile,
	io::Write,
	process::ExitCode,
	str,
};

use include_dir::{include_dir, Dir, File};
use once_cell::sync::Lazy;

// ---- //
// Type //
// ---- //

type CSSClassCustomList = HashSet<CSSClassCustom>;

// --------- //
// Constante //
// --------- //

const VUE_DIR: Dir = include_dir!("apps/web/vue");

static CONDITION_VALUE_FOR_PROP: Lazy<
	HashMap<CSSPropertyCustom, ConditionType>,
> = Lazy::new(|| {
	HashMap::from([
		(CSSPropertyCustom::Width, ConditionType::u8),
		(CSSPropertyCustom::MaxWidth, ConditionType::u8),
		(CSSPropertyCustom::MinWidth, ConditionType::u8),
		(CSSPropertyCustom::Height, ConditionType::u8),
		(CSSPropertyCustom::MaxHeight, ConditionType::u8),
		(CSSPropertyCustom::MinHeight, ConditionType::u8),
		(CSSPropertyCustom::Size, ConditionType::u8),
		(CSSPropertyCustom::Gap, ConditionType::u8),
		(CSSPropertyCustom::Padding, ConditionType::u8),
		(CSSPropertyCustom::PaddingTop, ConditionType::u8),
		(CSSPropertyCustom::PaddingRight, ConditionType::u8),
		(CSSPropertyCustom::PaddingBottom, ConditionType::u8),
		(CSSPropertyCustom::PaddingLeft, ConditionType::u8),
		(CSSPropertyCustom::PaddingX, ConditionType::u8),
		(CSSPropertyCustom::PaddingY, ConditionType::u8),
		(CSSPropertyCustom::Margin, ConditionType::u8),
		(CSSPropertyCustom::MarginTop, ConditionType::u8),
		(CSSPropertyCustom::MarginRight, ConditionType::u8),
		(CSSPropertyCustom::MarginBottom, ConditionType::u8),
		(CSSPropertyCustom::MarginLeft, ConditionType::u8),
		(CSSPropertyCustom::MarginX, ConditionType::u8),
		(CSSPropertyCustom::MarginY, ConditionType::u8),
		(CSSPropertyCustom::BorderRadius, ConditionType::u8),
		(CSSPropertyCustom::BorderRadiusTop, ConditionType::u8),
		(CSSPropertyCustom::BorderRadiusTopRight, ConditionType::u8),
		(CSSPropertyCustom::BorderRadiusTopLeft, ConditionType::u8),
		(CSSPropertyCustom::BorderRadiusRight, ConditionType::u8),
		(CSSPropertyCustom::BorderRadiusBottom, ConditionType::u8),
		(
			CSSPropertyCustom::BorderRadiusBottomRight,
			ConditionType::u8,
		),
		(CSSPropertyCustom::BorderRadiusBottomLeft, ConditionType::u8),
		(CSSPropertyCustom::BorderRadiusLeft, ConditionType::u8),
	])
});

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
struct CSSClassCustom(CSSPropertyCustom, String);

#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
enum CSSPropertyCustom {
	Width,
	MaxWidth,
	MinWidth,
	Height,
	MaxHeight,
	MinHeight,
	Size,
	Gap,
	Padding,
	PaddingTop,
	PaddingRight,
	PaddingBottom,
	PaddingLeft,
	PaddingX,
	PaddingY,
	Margin,
	MarginTop,
	MarginRight,
	MarginBottom,
	MarginLeft,
	MarginX,
	MarginY,
	BorderRadius,
	BorderRadiusTop,
	BorderRadiusTopRight,
	BorderRadiusTopLeft,
	BorderRadiusRight,
	BorderRadiusBottom,
	BorderRadiusBottomRight,
	BorderRadiusBottomLeft,
	BorderRadiusLeft,
}

#[allow(non_camel_case_types)]
enum ConditionType {
	u8,
}

// -------------- //
// Implémentation //
// -------------- //

impl CSSClassCustom {
	fn is_valid_value(prop: &CSSPropertyCustom, value: &str) -> bool {
		let condition_type = CONDITION_VALUE_FOR_PROP.get(prop);

		match condition_type {
			| Some(ConditionType::u8) => value.parse::<u8>().is_ok(),
			| None => true,
		}
	}

	fn value(&self) -> String {
		match self.0 {
			| CSSPropertyCustom::BorderRadius
			| CSSPropertyCustom::BorderRadiusTop
			| CSSPropertyCustom::BorderRadiusTopRight
			| CSSPropertyCustom::BorderRadiusTopLeft
			| CSSPropertyCustom::BorderRadiusRight
			| CSSPropertyCustom::BorderRadiusBottom
			| CSSPropertyCustom::BorderRadiusBottomRight
			| CSSPropertyCustom::BorderRadiusBottomLeft
			| CSSPropertyCustom::BorderRadiusLeft => format!("calc(2px * {})", self.1),
			| _ => format!("space({})", self.1),
		}
	}
}

impl CSSPropertyCustom {
	fn name(&self) -> &str {
		match self {
			| Self::Width => "w",
			| Self::MaxWidth => "max-w",
			| Self::MinWidth => "min-w",
			| Self::Height => "h",
			| Self::MaxHeight => "max-h",
			| Self::MinHeight => "min-h",
			| Self::Size => "size",
			| Self::Gap => "gap",
			| Self::Padding => "p",
			| Self::PaddingTop => "pt",
			| Self::PaddingRight => "pr",
			| Self::PaddingBottom => "pb",
			| Self::PaddingLeft => "pl",
			| Self::PaddingX => "px",
			| Self::PaddingY => "py",
			| Self::Margin => "m",
			| Self::MarginTop => "mt",
			| Self::MarginRight => "mr",
			| Self::MarginBottom => "mb",
			| Self::MarginLeft => "ml",
			| Self::MarginX => "mx",
			| Self::MarginY => "my",
			| Self::BorderRadius => "border:radius",
			| Self::BorderRadiusTop => "border-t:radius",
			| Self::BorderRadiusTopRight => "border-tr:radius",
			| Self::BorderRadiusTopLeft => "border-tl:radius",
			| Self::BorderRadiusRight => "border-r:radius",
			| Self::BorderRadiusBottom => "border-b:radius",
			| Self::BorderRadiusBottomRight => "border-br:radius",
			| Self::BorderRadiusBottomLeft => "border-bl:radius",
			| Self::BorderRadiusLeft => "border-l:radius",
		}
	}

	fn property(&self) -> &[&str] {
		match self {
			| Self::Width => &["width"],
			| Self::MaxWidth => &["max-width"],
			| Self::MinWidth => &["min-width"],
			| Self::Height => &["height"],
			| Self::MaxHeight => &["max-height"],
			| Self::MinHeight => &["min-height"],
			| Self::Size => &["width", "height"],
			| Self::Gap => &["grid-gap", "gap"],
			| Self::Padding => &["padding"],
			| Self::PaddingTop => &["padding-top"],
			| Self::PaddingRight => &["padding-right"],
			| Self::PaddingBottom => &["padding-bottom"],
			| Self::PaddingLeft => &["padding-left"],
			| Self::PaddingX => {
				&["padding-left", "padding-right", "padding-inline"]
			}
			| Self::PaddingY => {
				&["padding-top", "padding-bottom", "padding-block"]
			}
			| Self::Margin => &["margin"],
			| Self::MarginTop => &["margin-top"],
			| Self::MarginRight => &["margin-right"],
			| Self::MarginBottom => &["margin-bottom"],
			| Self::MarginLeft => &["margin-left"],
			| Self::MarginX => {
				&["margin-left", "margin-right", "margin-inline"]
			}
			| Self::MarginY => &["margin-top", "margin-bottom", "margin-block"],
			| Self::BorderRadius => &["border-radius"],
			| Self::BorderRadiusTop => {
				&["border-top-left-radius", "border-top-right-radius"]
			}
			| Self::BorderRadiusTopRight => &["border-top-right-radius"],
			| Self::BorderRadiusTopLeft => &["border-top-left-radius"],
			| Self::BorderRadiusRight => {
				&["border-top-right-radius", "border-bottom-right-radius"]
			}
			| Self::BorderRadiusBottom => {
				&["border-bottom-left-radius", "border-bottom-right-radius"]
			}
			| Self::BorderRadiusBottomRight => &["border-bottom-right-radius"],
			| Self::BorderRadiusBottomLeft => &["border-bottom-left-radius"],
			| Self::BorderRadiusLeft => {
				&["border-top-left-radius", "border-bottom-left-radius"]
			}
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl fmt::Display for CSSClassCustom {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let properties = self
			.0
			.property()
			.iter()
			.map(|prop| format!("{prop}: {};", self.value()))
			.collect::<String>();

		let rule = format!(
			r#"@include selector-class("{}={}") {{ {} }}"#,
			self.0.name(),
			self.1,
			properties,
		);

		writeln!(f, "{}", rule)
	}
}

impl str::FromStr for CSSClassCustom {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() < 3 {
			return Err("osef");
		}
		let prop_value = s.split_once('=');
		if let Some((prop, value)) = prop_value {
			if cfg!(debug_assertions) {
				println!("\t {prop}={value}");
			}

			let prop = prop.parse::<CSSPropertyCustom>()?;
			if cfg!(debug_assertions) {
				println!("\t\t {prop:?}");
			}
			if Self::is_valid_value(&prop, value) {
				let value = value.to_owned();
				Ok(Self(prop, value))
			} else {
				Err("Propriété ou valeur invalide.")
			}
		} else {
			Err("propriété non géré pour le moment.")
		}
	}
}

impl str::FromStr for CSSPropertyCustom {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			| "w" => Self::Width,
			| "max-w" => Self::MaxWidth,
			| "min-w" => Self::MinWidth,
			| "h" => Self::Height,
			| "max-h" => Self::MaxHeight,
			| "min-h" => Self::MinHeight,
			| "size" => Self::Size,
			| "gap" => Self::Gap,
			| "p" => Self::Padding,
			| "pt" => Self::PaddingTop,
			| "pr" => Self::PaddingRight,
			| "pb" => Self::PaddingBottom,
			| "pl" => Self::PaddingLeft,
			| "px" => Self::PaddingX,
			| "py" => Self::PaddingY,
			| "m" => Self::Margin,
			| "mt" => Self::MarginTop,
			| "mr" => Self::MarginRight,
			| "mb" => Self::MarginBottom,
			| "ml" => Self::MarginLeft,
			| "mx" => Self::MarginX,
			| "my" => Self::MarginY,
			| "border:radius" => Self::BorderRadius,
			| "border-t:radius" => Self::BorderRadiusTop,
			| "border-tr:radius" => Self::BorderRadiusTopRight,
			| "border-tl:radius" => Self::BorderRadiusTopLeft,
			| "border-r:radius" => Self::BorderRadiusRight,
			| "border-b:radius" => Self::BorderRadiusBottom,
			| "border-br:radius" => Self::BorderRadiusBottomRight,
			| "border-bl:radius" => Self::BorderRadiusBottomLeft,
			| "border-l:radius" => Self::BorderRadiusLeft,
			| _ => return Err("Propriété invalide"),
		})
	}
}

// -------- //
// Fonction //
// -------- //

fn main() -> ExitCode {
	let mut args = env::args();

	if args.len() != 2 {
		return ExitCode::FAILURE;
	}

	args.next();

	let relative_filename = args.next().expect("filename");

	let mut classes = HashSet::new();

	check_file(&VUE_DIR, &mut classes);

	let mut temporary_buffer = String::new();
	for class in classes {
		temporary_buffer.push_str(&class.to_string());
	}

	let mut file = StdFile::create(relative_filename)
		.expect("impossible de créer le fichier scss");

	file.write_all(
		b"/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */\n
",
	)
	.expect("header");

	file.write_all(temporary_buffer.as_bytes())
		.expect("Impossible d'écrire dans le fichier");

	ExitCode::SUCCESS
}

fn check_file(directory: &'static Dir, list: &mut CSSClassCustomList) {
	for dir in directory.dirs() {
		let mut entries = vec![];
		let vue_entries: Vec<_> = dir.find("**/*.vue").expect("?").collect();
		entries.extend(vue_entries);
		let entries = entries;
		for entry in entries {
			if let Some(file) = entry.as_file() {
				let path = file.path().to_string_lossy();
				if path.contains("node_modules") || path.contains("wasm.") {
					continue;
				}
				generate_style(file, list);
			}
		}
	}

	for file in directory.files() {
		if file
			.path()
			.extension()
			.filter(|ext| *ext == "vue")
			.is_none()
		{
			continue;
		}
		generate_style(file, list);
	}
}

fn generate_style(file: &'static File, list: &mut CSSClassCustomList) {
	if cfg!(debug_assertions) {
		println!("Fichier {}", file.path().to_string_lossy());
	}

	let cb = |content: &str| {
		let token_stream = style::Lexer::lex(content.chars());
		style::Parser::parse(token_stream)
	};

	if let Some(custom_css_class) = file.contents_utf8().map(cb) {
		list.extend(custom_css_class);
	}
}
