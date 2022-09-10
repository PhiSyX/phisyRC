/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;

use serde::Deserialize;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(serde::Deserialize)]
pub struct ClientWebConfig {
	pub public_dir: String,
	pub server: ClientWebServerConfig,
}

#[derive(Debug)]
#[derive(serde::Deserialize)]
pub struct ClientWebServerConfig {
	pub host: String,
	#[serde(deserialize_with = "ClientWebServerPortConfig::validate")]
	pub port: ClientWebServerPortConfig,
	pub proxy: Option<String>,
}

#[derive(Debug)]
#[derive(serde::Deserialize)]
pub struct ClientWebServerPortConfig(pub u16);

// -------------- //
// Implémentation //
// -------------- //

impl ClientWebServerPortConfig {
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

impl fmt::Display for ClientWebServerPortConfig {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
