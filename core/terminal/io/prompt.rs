/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::fmt;
use std::{
	io::{self, BufRead},
	str::FromStr,
};

// --------- //
// Interface //
// --------- //

pub trait Prompt {
	fn prompt() -> Self;
}

// -------- //
// Fonction //
// -------- //

/// Invite l'utilisateur à confirmer (Y) ou annuler (N).
pub fn confirm(message: impl fmt::Display) -> bool {
	let mut temporary_buffer = String::new();
	let mut choice = false;

	while temporary_buffer.is_empty() {
		println!("$ {message} (y/n)");

		let stdin = io::stdin().lock().lines().next();
		if let Some(Ok(line)) = stdin {
			temporary_buffer = line.trim().to_lowercase();
			if temporary_buffer == "y" {
				choice = true;
				break;
			} else if temporary_buffer == "n" {
				choice = false;
				break;
			}
			continue;
		} else {
			temporary_buffer.clear();
		}
	}

	choice
}

/// Invite l'utilisateur à saisir une entrée (optionnelle).
pub fn prompt_optional<T>(message: impl fmt::Display) -> Option<T>
where
	T: FromStr,
	T::Err: fmt::Debug,
{
	let mut temporary_buffer = String::new();

	loop {
		println!("$ {message} (optionnel)");
		let stdin = io::stdin().lock().lines().next();
		if let Some(Ok(line)) = stdin {
			temporary_buffer = line.trim().to_owned();
			break;
		} else {
			temporary_buffer.clear();
		}
	}

	if temporary_buffer.is_empty() {
		return None;
	}

	temporary_buffer.parse().ok()
}

/// Invite l'utilisateur à saisir une entrée (requise).
pub fn prompt_required<T>(message: impl fmt::Display) -> T
where
	T: FromStr,
	T::Err: fmt::Debug,
{
	let mut temporary_buffer = String::new();

	while temporary_buffer.is_empty() {
		println!("$ {message}");
		let stdin = io::stdin().lock().lines().next();
		if let Some(Ok(line)) = stdin {
			temporary_buffer = line.trim().to_owned();
		} else {
			temporary_buffer.clear();
		}
	}

	temporary_buffer
		.parse()
		.expect("impossible de convertir vers le type")
}

/// Invite l'utilisateur à saisir une entrée, avec une valeur par défaut dans
/// le cas où il n'y aurait pas de valeur valide.
pub fn prompt_default<T>(
	message: impl fmt::Display,
	default: impl ToString,
) -> T
where
	T: FromStr,
	T::Err: fmt::Debug,
{
	let mut temporary_buffer = String::new();
	loop {
		println!("$ {message} (par défaut: {})", default.to_string());
		let stdin = io::stdin().lock().lines().next();
		if let Some(Ok(line)) = stdin {
			temporary_buffer = line.trim().to_owned();
			break;
		} else {
			temporary_buffer.clear();
		}
	}

	if temporary_buffer.is_empty() {
		temporary_buffer = default.to_string();
		println!("\t > {temporary_buffer}");
		println!();
	}

	temporary_buffer
		.parse()
		.expect("impossible de convertir vers le type")
}
