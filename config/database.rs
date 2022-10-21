/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use terminal::io::{confirm, prompt_default, Prompt};

use crate::Port;

// --------- //
// Structure //
// --------- //

/// Configuration de la base de données.
#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct DatabaseConfig {
	/// Adresse IP.
	pub ip: String,
	/// Port de connexion.
	pub port: Port,

	/// Nom d'utilisateur.
	pub username: String,
	/// Mot de passe.
	pub password: String,

	/// Nom de la base de données.
	pub name: String,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl Prompt for DatabaseConfig {
	fn prompt() -> Self {
		let ip = prompt_default(
			"Adresse IP de la base de données",
			constants::DEFAULT_DATABASE_IP,
		);

		let port = prompt_default::<u16>(
			"Port de connexion à la base de données",
			constants::DEFAULT_DATABASE_PORT,
		);

		let username = prompt_default(
			"Nom d'utilisateur de connexion à la base de données",
			constants::DEFAULT_DATABASE_USERNAME,
		);

		let password = prompt_default(
			"Mot de passe de connexion à la base de données",
			constants::DEFAULT_DATABASE_PASSWORD,
		);

		let name = prompt_default(
			"Nom de la base de données",
			constants::DEFAULT_DATABASE_NAME,
		);

		let build = Self {
			name,
			ip,
			port: port.into(),
			username,
			password,
		};

		println!("Configuration terminée : {:#?}", &build);
		println!();

		if confirm("Êtes vous satisfait de cette configuration?") {
			build
		} else {
			println!("Recommençons...");
			println!();

			Self::prompt()
		}
	}
}

impl Default for DatabaseConfig {
	fn default() -> Self {
		Self {
			ip: constants::DEFAULT_DATABASE_IP.into(),
			port: constants::DEFAULT_DATABASE_PORT.into(),
			username: constants::DEFAULT_DATABASE_USERNAME.into(),
			password: constants::DEFAULT_DATABASE_PASSWORD.into(),
			name: constants::DEFAULT_DATABASE_NAME.into(),
		}
	}
}
