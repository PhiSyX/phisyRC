/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::str::FromStr;
use std::{env, path::Path};

use crate::error::Error;

pub trait Interface {
	fn setup(file: impl AsRef<Path>) -> Result<Self, Error>
	where
		Self: Sized;

	/// Retourne la valeur d'une variable d'environnement.
	fn get_var<T>(key: &'static str) -> Result<T, Error>
	where
		T: FromStr,
	{
		env::var(key)
			.map_err(|_| Error::Missing(key))
			.and_then(|value| {
				value.parse::<T>().map_err(|_| Error::BadFormat(key))
			})
	}

	/// Retourne la valeur d'une variable d'environnement. En cas d'échec une
	/// valeur par défaut est retournée.
	fn get_default_var<T>(
		key: &'static str,
		default: &'static str,
	) -> Result<T, Error>
	where
		T: FromStr,
	{
		let maybe_value = env::var(key);

		let value = match maybe_value {
			| Ok(ref value) => value,
			| Err(_) => default,
		};

		value.parse::<T>().map_err(|_| Error::BadFormat(key))
	}

	/// Retourne la valeur d'une variable d'environnement sous forme de
	/// [Option]. En cas d'échec [None] est retourné.
	fn get_optional_var<T>(key: &'static str) -> Result<Option<T>, Error>
	where
		T: FromStr,
	{
		match env::var(key).ok() {
			| Some(v) => {
				v.parse::<T>().map(Some).map_err(|_| Error::BadFormat(key))
			}
			| None => Ok(None),
		}
	}
}
