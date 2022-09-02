/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;
use std::net::SocketAddr;

use serde::Deserialize;

use super::serde::SerdeValidation;

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
}

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(serde::Deserialize)]
pub struct IrcdConfigPort(pub u16);

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
}

impl IrcdConfigPort {
	const MAX_USER_PORT: u16 = 49151;
	const MIN_USER_PORT: u16 = 1024;

	/// Les numéros de port sont attribués de différentes manières, sur la
	/// base de trois gammes :
	///     - Ports systèmes (0..1023)
	///     - Ports utilisateurs (1024..49151)
	///     - Ports dynamiques et/ou privés (49152..65535)
	/// On ne veut pas que la configuration puisse définir un port qui
	/// correspond à un port système, ou un port dynamique/privé.
	fn is_valid(&self) -> bool {
		(Self::MIN_USER_PORT..=Self::MAX_USER_PORT).contains(&self.0)
	}

	/// Valide la valeur utilisateur: port réseau.
	/// La valeur DOIT être comprise entre 1024 et 49151.
	fn validate<'de, D: serde::Deserializer<'de>>(
		de: D,
	) -> Result<Self, D::Error> {
		Option::deserialize(de)
			// NOTE(phisyx): le port est invalide en deçà/au delà de
			// `u16::MIN` / `u16::MAX`
			.unwrap_or_default()
			.filter(|port: &Self| port.is_valid())
			.ok_or_else(|| {
				let msg = format!(
					"Le port d'écoute est invalide. Il DOIT être compris entre \
					{} et {}.",
					Self::MIN_USER_PORT, Self::MAX_USER_PORT
				);
				serde::de::Error::custom(msg)
			})
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<IrcdConfigPort> for u16 {
	fn from(config: IrcdConfigPort) -> Self {
		config.0
	}
}

impl fmt::Display for IrcdConfigPort {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
