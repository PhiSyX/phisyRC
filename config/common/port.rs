/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::{fmt, ops};
use std::str::FromStr;

use serde::Deserialize;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct Port(pub u16);

// -------------- //
// Implémentation //
// -------------- //

impl Port {
	const MAX_USER_PORT: u16 = 49151;
	const MIN_USER_PORT: u16 = 1024;

	/// Les numéros de port sont attribués de différentes manières, sur la
	/// base de trois gammes :
	///     - Ports systèmes (0..1023)
	///     - Ports utilisateurs (1024..49151)
	///     - Ports dynamiques et/ou privés (49152..65535)
	/// On ne veut pas que la configuration puisse définir un port qui
	/// correspond à un port système, ou un port dynamique/privé.
	pub fn is_valid(&self) -> bool {
		(Self::MIN_USER_PORT..=Self::MAX_USER_PORT).contains(&self.0)
	}

	/// Valide la valeur utilisateur: port réseau.
	/// La valeur DOIT être comprise entre 1024 et 49151.
	pub fn validate<'de, D: serde::Deserializer<'de>>(
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
					« {} » et « {} ».",
					Self::MIN_USER_PORT, Self::MAX_USER_PORT
				);
				serde::de::Error::custom(msg)
			})
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<Port> for u16 {
	fn from(config: Port) -> Self {
		config.0
	}
}

impl From<u16> for Port {
	fn from(port: u16) -> Self {
		Self(port)
	}
}

impl FromStr for Port {
	type Err = std::num::ParseIntError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.parse::<u16>().map(Self)
	}
}

impl ops::Deref for Port {
	type Target = u16;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl fmt::Display for Port {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.0)
	}
}
