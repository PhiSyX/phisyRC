/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod arch;
mod commands;
mod config;
mod daemon;
mod message;
mod output;

use std::path::Path;

use self::arch::IrcNetwork;
pub(crate) use self::{commands::*, message::*};
pub use self::{daemon::*, output::*};

// --------- //
// Structure //
// --------- //

pub struct IRC;

// -------------- //
// Implémentation //
// -------------- //

impl IRC {
	pub async fn run(config_file: impl AsRef<Path>) -> IrcResult<()> {
		logger::info!("Lancement de l'IRC...");

		let config = config::load(config_file)?;

		let network = IrcNetwork::new(&config)?;
		network.try_establish_connections().await?;

		loop {
			tokio::time::sleep(std::time::Duration::from_secs(1)).await;
		}

		// Ok(())
	}
}
