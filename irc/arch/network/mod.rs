/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod socket;

use std::{collections::HashMap, sync::Arc};

use tokio::io;

pub(crate) use self::socket::*;
use super::components::{IrcServerError, Server};
use crate::{arch::AtomicClient, config::IrcdConfig, forever};

// ---- //
// Type //
// ---- //

pub type AtomicNetwork = Arc<Network>;

// --------- //
// Structure //
// --------- //

/// Un réseau IRC est défini par un groupe de serveurs connectés les uns
/// aux autres. Un seul serveur constitue le réseau IRC le plus simple.
///
/// La seule configuration de réseau autorisée pour les serveurs IRC est
/// celle d'un arbre couvrant où chaque serveur agit comme un nœud central
/// pour le reste du réseau qu'il voit.
#[derive(Debug)]
#[derive(Clone)]
pub struct Network {
	pub config: IrcdConfig,
	pub servers: HashMap<String, Server>,
}

#[derive(Debug)]
pub enum IrcNetworkError {
	IO(io::Error),

	Server(IrcServerError),
}

// -------------- //
// Implémentation //
// -------------- //

impl Network {
	/// Crée un nouveau réseau IRC.
	pub fn new(config: &IrcdConfig) -> Result<Self, IrcNetworkError> {
		let network = Self {
			config: config.to_owned(),
			servers: Default::default(),
		};

		network.define_servers()
	}

	/// Définie les serveurs du réseau.
	fn define_servers(mut self) -> Result<Self, IrcNetworkError> {
		let mut servers = HashMap::new();

		for listen in self.config.listens.clone() {
			let server = Server::new(self.shared(), &listen)?;
			servers.insert(server.label.to_owned(), server);
		}

		self.servers = servers;

		Ok(self)
	}

	fn shared(&self) -> AtomicNetwork {
		Arc::new(self.clone())
	}
}

impl Network {
	/// Tente d'établir les connexions entre les serveurs du réseau.
	pub async fn try_establish_connections(
		self,
	) -> Result<(), IrcNetworkError> {
		for (label, mut server) in self.servers.into_iter() {
			logger::info!("Tentative d'établissement de la connexion au serveur « {label} ».");

			forever! {
			server.ping_host().await?;

			loop {
				let socket: SocketStream = server.try_establish_connection().await?;
				let client: AtomicClient = server.new_client(&socket);
				server.intercept_messages(client, socket.codec()).await;
			}

			return Ok::<(), IrcNetworkError>(());
			};
		}

		Ok(())
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<io::Error> for IrcNetworkError {
	fn from(err: io::Error) -> Self {
		Self::IO(err)
	}
}

impl From<IrcServerError> for IrcNetworkError {
	fn from(err: IrcServerError) -> Self {
		Self::Server(err)
	}
}
