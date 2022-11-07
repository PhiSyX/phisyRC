/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::Port;

// --------- //
// Structure //
// --------- //

/// Configuration de serveur.
#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
#[derive(phisyrc::Prompt)]
pub struct ServerConfig {
	/// Nom du serveur
	#[prompt]
	pub name: String,

	/// Adresse IP du serveur
	#[prompt(default = "127.0.0.1")]
	pub ip: String,

	/// Port de communication TCP/IP du serveur
	#[prompt(default = "6667")]
	pub tcp_port: Port,

	/// Port de communication WebSocket du serveur
	#[prompt(default = "9667")]
	pub websocket_port: Port,

	/// Mot de passe de connexion au serveur
	#[prompt]
	pub password: Option<String>,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl Default for ServerConfig {
	fn default() -> Self {
		Self {
			name: constants::DEFAULT_SERVER_NAME.into(),
			ip: constants::DEFAULT_SERVER_IP.into(),
			tcp_port: constants::DEFAULT_SERVER_PORT.into(),
			websocket_port: constants::DEFAULT_SERVER_WEBSOCKET_PORT.into(),
			password: constants::DEFAULT_SERVER_PASSWORD,
		}
	}
}
