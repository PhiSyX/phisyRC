/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// NOTE(phisyx): Pas fan de cette partie de code.

use crate::Result;

// ---- //
// Sync //
// ---- //

pub fn config_interactive() -> Result<()> {
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

// -------- //
// Sync Alt //
// -------- //

pub fn config_interactive_1<C>(_: &C) -> Result<()> {
	config_interactive()
}
pub fn config_interactive_2<C, E>(_: (&C, &E)) -> Result<()> {
	config_interactive()
}

// --------- //
// Async Alt //
// --------- //

pub async fn future_config_interactive() -> Result<()> {
	config_interactive()
}

pub async fn future_config_interactive_1<C, Context>(
	_: &C,
	_: Context,
) -> Result<()> {
	config_interactive()
}
pub async fn future_config_interactive_2<C, E, Context>(
	_: (&C, &E),
	_: Context,
) -> Result<()> {
	config_interactive()
}
