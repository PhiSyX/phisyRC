/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::net::SocketAddr;

use shared::serde::{Port, SerdeValidation};

// ---- //
// Type //
// ---- //

pub type IrcdConfigPort = Port;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Deserialize)]
pub struct IrcdConfig {
	/// Information de l'IRCd, du réseau, des administrateurs de l'IRCd, etc...
	pub info: IrcdInfo,

	/// Liste des serveurs IRC à établir.
	#[serde(rename = "listen", default)]
	pub listens: Vec<IrcdListen>,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Deserialize)]
pub struct IrcdInfo {
	pub admin: Vec<String>,
	pub description: String,
	pub name: String,
	pub network_name: String,
}

/// Définie les ports TCP/IP sur lesquels l'IRCd va écouter.
#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Deserialize)]
pub struct IrcdListen {
	/// Nom du serveur IRC.
	pub name: String,

	/// L'IP du serveur IRC.
	pub ip: String,

	/// Il s'agit du port sur lequel nous devons ouvrir la connexion, par
	/// exemple 6667.
	#[serde(deserialize_with = "IrcdConfigPort::validate")]
	pub port: IrcdConfigPort,

	/// Le mot de passe du serveur IRC, s'il doit y en avoir un.
	pub password: Option<IrcdPassword>,

	/// Options WebSocket.
	pub websocket: Option<IrcdConfigWebSocket>,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Deserialize)]
pub struct IrcdPassword {
	#[serde(default, deserialize_with = "SerdeValidation::string_not_empty")]
	/// Le mot de passe.
	pub secret: String,

	/// Algorithme à utiliser.
	#[serde(default)]
	pub algo: IrcdPasswordAlgorithm,
}

#[derive(Debug)]
#[derive(Default)]
#[derive(Clone)]
#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IrcdPasswordAlgorithm {
	/// Mot de passe en texte clair directement dans la configuration. Non
	/// recommandé.
	Plain,

	/// Mot de passe haché avec l'algorithme de hachage Argon2.
	#[default]
	Argon2,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(serde::Deserialize)]
pub struct IrcdConfigWebSocket {
	pub port: IrcdConfigPort,
}

// -------------- //
// Implémentation //
// -------------- //

impl IrcdListen {
	/// Converti une chaîne de caractères sous la forme de "ip:port" en une
	/// adresse socket.
	pub fn addr(&self) -> SocketAddr {
		format!("{}:{}", self.ip, self.port).parse().expect(
			"Devrait être une adresse socket valide à partir d'une \
			chaîne de caractères 'ip:port'.",
		)
	}

	/// Converti une chaîne de caractères sous la forme de "ip:port" en une
	/// adresse socket.
	pub fn ws_addr(&self) -> Option<SocketAddr> {
		self.websocket
			.as_ref()
			.and_then(|cfg| format!("{}:{}", self.ip, cfg.port).parse().ok())
	}
}
