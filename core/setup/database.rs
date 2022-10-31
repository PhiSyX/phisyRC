/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// NOTE(phisyx): Pas fan de cette partie de code.

pub async fn future_database_postgres() -> database::Result<database::Client> {
	let cfg = config::load_or_prompt::<config::DatabaseConfig>(
		constants::CONFIG_DATABASE,
		"Voulez-vous créer la configuration de la base de données?",
	)?;

	database::connect(
		(cfg.ip, cfg.port),
		(cfg.username, cfg.password),
		cfg.name,
	)
	.await
}

pub async fn future_database_postgres_1<C, Context>(
	_: &C,
	_: Context,
) -> database::Result<database::Client> {
	future_database_postgres().await
}

pub async fn future_database_postgres_2<C, E, Context>(
	_: (&C, &E),
	_: Context,
) -> database::Result<database::Client> {
	future_database_postgres().await
}
