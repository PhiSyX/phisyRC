/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::{collections::HashMap, env, path::Path, str::FromStr};

use crate::error::EnvError;

pub trait EnvInterface {
	type Err;

	fn setup(file: impl AsRef<Path>) -> Result<Self, Self::Err>
	where
		Self: Sized;

	/// Retourne la valeur d'une variable d'environnement.
	fn get_var<T: FromStr>(
		key: &'static str,
		map: Option<&HashMap<String, String>>,
	) -> Result<T, EnvError> {
		match map {
			| Some(map) => map.get(key).map(|v| v.to_string()),
			| None => env::var(key).ok(),
		}
		.ok_or(EnvError::Missing(key))
		.and_then(|v| v.parse::<T>().map_err(|_| EnvError::BadFormat(key)))
	}

	/// Retourne la valeur d'une variable d'environnement. En cas d'échec une
	/// valeur par défaut est retournée.
	fn get_default_var<T: FromStr>(
		key: &'static str,
		map: Option<&HashMap<String, String>>,
		default: &'static str,
	) -> Result<T, EnvError> {
		let maybe_var = match map {
			| Some(map) => map.get(key).map(|val| val.to_string()),
			| None => env::var(key).ok(),
		};

		let val = match maybe_var {
			| Some(ref value) => value,
			| None => default,
		};

		val.parse::<T>().map_err(|_| EnvError::BadFormat(key))
	}

	/// Retourne la valeur d'une variable d'environnement sous forme de
	/// [Option]. En cas d'échec [None] est retourné.
	fn get_optional_var<T: FromStr>(
		key: &'static str,
		map: Option<&HashMap<String, String>>,
	) -> Result<Option<T>, EnvError> {
		let maybe_var = match map {
			| Some(map) => map.get(key).map(|val| val.to_string()),
			| None => env::var(key).ok(),
		};

		match maybe_var {
			| Some(v) => v
				.parse::<T>()
				.map(Some)
				.map_err(|_| EnvError::BadFormat(key)),
			| None => Ok(None),
		}
	}
}
