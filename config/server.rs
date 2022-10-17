/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use terminal::io::{prompt_default, prompt_optional, prompt_required, Prompt};

use crate::Port;

// --------- //
// Structure //
// --------- //

/// Configuration de serveur.
#[derive(Debug)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ServerConfig {
	/// Nom du serveur.
	pub name: String,
	/// Adresse IP.
	pub ip: String,
	/// Port de communication TCP/IP.
	pub port: Port,
	/// Mot de passe de connexion au serveur.
	pub password: Option<String>,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl Prompt for ServerConfig {
	fn prompt() -> Self {
		let name = prompt_default("Nom du serveur", "localhost");
		let ip = prompt_required("Adresse IP du serveur");
		let port = prompt_required::<u16>("Port de communication TCP/IP");
		let password = prompt_optional("Mot de passe de connexion au serveur");

		Self {
			name,
			ip,
			port: port.into(),
			password,
		}
	}
}

impl Default for ServerConfig {
	fn default() -> Self {
		Self {
			name: "localhost".into(),
			ip: "127.0.0.1".into(),
			port: 6667.into(),
			password: Default::default(),
		}
	}
}
