/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{env, path::Path, str::FromStr};

use crate::error::Error;

// ---- //
// Type //
// ---- //

type VariableName = &'static str;

// --------- //
// Interface //
// --------- //

pub trait Interface {
	fn setup(file: impl AsRef<Path>) -> Result<Self, Error>
	where
		Self: Sized;

	/// Retourne la valeur d'une variable d'environnement.
	fn get_var<T>(key: VariableName) -> Result<T, Error>
	where
		T: FromStr,
	{
		env::var(key)
			.map_err(|_| Error::NotFound(key))
			.and_then(|v| v.parse().map_err(|_| Error::BadFormat(key)))
	}

	/// Retourne la valeur d'une variable d'environnement. En cas d'échec une
	/// valeur par défaut est retournée.
	fn get_default_var<T>(
		key: VariableName,
		default: &'static str,
	) -> Result<T, Error>
	where
		T: FromStr,
	{
		let maybe_value = env::var(key);

		let value = match maybe_value.as_ref() {
			| Ok(v) => v,
			| Err(_) => default,
		};

		value.parse().map_err(|_| Error::BadFormat(key))
	}

	/// Retourne la valeur d'une variable d'environnement sous forme de
	/// [Option]. En cas d'échec [None] est retourné.
	fn get_optional_var<T>(key: VariableName) -> Result<Option<T>, Error>
	where
		T: FromStr,
	{
		match env::var(key) {
			| Ok(v) => v.parse().map(Some).map_err(|_| Error::BadFormat(key)),
			| Err(_) => Ok(None),
		}
	}
}
