/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod arch;
mod commands;
pub mod config;
mod daemon;
mod export;
mod macros;
mod message;
mod types;

use std::path::Path;

pub(crate) use self::arch::*;
pub use self::export::*;

// --------- //
// Structure //
// --------- //

pub struct Irc;

// -------------- //
// Implémentation //
// -------------- //

impl Irc {
	pub async fn run(config_file: impl AsRef<Path>) -> Result<()> {
		logger::info!("Lancement de l'IRC...");

		let config = fs::TOMLFileLoader::load(config_file)?;

		logger::debug!(
			"Sortie de dé-sérialisation du fichier de configuration \n {0:#?}",
			&config
		);

		let network = Network::new(&config)?;
		network.try_establish_connections().await?;

		loop {
			tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
		}

		// Ok(())
	}
}
