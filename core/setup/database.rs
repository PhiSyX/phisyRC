/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::str::FromStr;

use crate::{SetupCliInterface, SetupEnvInterface};

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Default)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum DatabaseType {
	#[default]
	Relational,
	FileSystem,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl FromStr for DatabaseType {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			| "fs" | "filesystem" | "local" => Self::FileSystem,
			| _ => Self::Relational,
		})
	}
}

impl<'a> From<&'a str> for DatabaseType {
	fn from(s: &'a str) -> Self {
		s.parse().unwrap_or_default()
	}
}

pub async fn database<C, E, Ctx>(
	_: (&C, &E),
	ty: impl Into<DatabaseType>,
	_: Ctx,
) -> database::Result<database::Client>
where
	C: SetupCliInterface,
	E: SetupEnvInterface,
{
	match ty.into() {
		| DatabaseType::Relational => {
			let cfg = config::load_or_prompt::<config::DatabaseConfig>(
				constants::CONFIG_DATABASE,
				"Voulez-vous créer la configuration de la base de données?",
			)?;

			Ok(database::connect(
				(cfg.ip, cfg.port),
				(cfg.username, cfg.password),
				cfg.name,
			)
			.await?)
		}
		| DatabaseType::FileSystem => Ok(database::Client::FileSystem()),
	}
}
