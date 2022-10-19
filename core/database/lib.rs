/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod export;

use core::fmt;
use std::str::FromStr;

use postgres::NoTls;

pub use self::export::*;

// ---- //
// Type //
// ---- //

pub type Result<T, E = Error> = core::result::Result<T, E>;

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

pub enum Client {
	Relational(postgres::Client),
	FileSystem(/* todo */),
}

#[derive(Debug)]
pub enum Error {
	IO(std::io::Error),
	Relational(postgres::Error),
}

// -------- //
// Fonction //
// -------- //

/// Connexion et préparation de la base de données.
//
// TODO(phisyx): TLS?
pub async fn connect(
	(ip, port): (impl fmt::Display, impl fmt::Display),
	(username, password): (impl fmt::Display, impl fmt::Display),
	name: impl fmt::Display,
) -> Result<Client> {
	let database_config = format!(
		"host='{}' port='{}' user='{}' password='{}' dbname='{}'",
		ip, port, username, password, name,
	);

	let (client, connection) =
		tokio_postgres::connect(&database_config, NoTls).await?;

	tokio::spawn(async move {
		if let Err(err) = connection.await {
			logger::error!(
				"Erreur lors de la connexion à la base de données: {err}."
			);
		};
	});

	logger::trace!("Préparation de la base donnée...");

	client
		.execute(&format!("CREATE SCHEMA IF NOT EXISTS {}", &name), &[])
		.await?;

	Ok(Client::Relational(client))
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

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Self::IO(err)
	}
}

impl From<postgres::Error> for Error {
	fn from(err: postgres::Error) -> Self {
		Self::Relational(err)
	}
}

impl<'a> From<&'a str> for DatabaseType {
	fn from(s: &'a str) -> Self {
		s.parse().unwrap_or_default()
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let err = match self {
			| Self::IO(err) => err.to_string(),
			| Self::Relational(err) => err.to_string(),
		};
		write!(f, "{err}")
	}
}
