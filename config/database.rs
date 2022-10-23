/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::Port;

// --------- //
// Structure //
// --------- //

/// Configuration de la base de données.
#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(phisyrc::Prompt)]
pub struct DatabaseConfig {
	/// Adresse IP de la base de données
	#[prompt(default = "127.0.0.1")]
	pub ip: String,
	/// Port de connexion de la base de données
	#[prompt(default = "5432")]
	pub port: Port,

	/// Nom d'utilisateur.
	#[prompt(default = "root")]
	pub username: String,
	/// Mot de passe.
	#[prompt(default = "root")]
	pub password: String,

	/// Nom de la base de données.
	#[prompt(default = "phisyrc")]
	pub name: String,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

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
