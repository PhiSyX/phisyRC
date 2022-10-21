/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::Result;

pub async fn config<C, E, Ctx>(_: (&C, &E), _: &str, _: Ctx) -> Result<()> {
	#[cfg(feature = "database")]
	config::load_or_prompt::<config::DatabaseConfig>(
		constants::CONFIG_DATABASE,
		"Voulez-vous créer la configuration de la base de données?",
	)?;

	config::load_or_prompt::<config::ServerConfig>(
		constants::CONFIG_SERVER,
		"Voulez-vous créer la configuration serveur?",
	)?;

	Ok(())
}
