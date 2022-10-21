/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use terminal::io::{confirm, prompt_default, prompt_optional, Prompt};

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
		let name =
			prompt_default("Nom du serveur", constants::DEFAULT_SERVER_NAME);

		let ip = prompt_default(
			"Adresse IP du serveur",
			constants::DEFAULT_SERVER_IP,
		);

		let port = prompt_default::<u16>(
			"Port de communication TCP/IP",
			constants::DEFAULT_SERVER_PORT,
		);

		let password = prompt_optional("Mot de passe de connexion au serveur");

		let build = Self {
			name,
			ip,
			port: port.into(),
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

impl Default for ServerConfig {
	fn default() -> Self {
		Self {
			name: constants::DEFAULT_SERVER_NAME.into(),
			ip: constants::DEFAULT_SERVER_IP.into(),
			port: constants::DEFAULT_SERVER_PORT.into(),
			password: constants::DEFAULT_SERVER_PASSWORD,
		}
	}
}
