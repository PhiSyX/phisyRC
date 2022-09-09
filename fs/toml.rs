/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use logger::Stylize;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(serde::Deserialize)]
pub struct TOMLFileLoader<T> {
	/// Clé `next`, qui doit se trouver dans le fichier TOML en question.
	next: T,
}

// -------------- //
// Implémentation //
// -------------- //

impl<T> TOMLFileLoader<T>
where
	T: serde::de::DeserializeOwned,
{
	/// Dé-sérialise un fichier .TOML avec le type Self, qui a donc comme clé
	/// `next`.
	pub fn load_with_next_key(
		filename: impl AsRef<std::path::Path>,
	) -> std::io::Result<T> {
		let content = std::fs::read(&filename)?;
		let data = toml::from_slice::<Self>(&content)?;
		let f = filename.as_ref().display();

		logger::info!(
			"Fichier « {0} » chargé avec succès.",
			f.to_string().magenta()
		);

		Ok(data.next)
	}

	/// Dé-sérialise un fichier .TOML avec le type T.
	pub fn load(filename: impl AsRef<std::path::Path>) -> std::io::Result<T> {
		let content = std::fs::read(&filename)?;
		let data = toml::from_slice::<T>(&content)?;
		let f = filename.as_ref().display();

		logger::info!(
			"Fichier « {0} » chargé avec succès.",
			f.to_string().magenta()
		);

		Ok(data)
	}
}
