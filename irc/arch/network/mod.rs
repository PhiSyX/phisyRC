/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod connection;
mod socket;

use std::{collections::HashMap, sync::Arc};

use tokio::{io, sync::RwLock};

pub(crate) use self::{connection::*, socket::*};
use super::{
	components::{IrcServerError, Server},
	Client,
};
use crate::config::IrcdConfig;

// ---- //
// Type //
// ---- //

pub type AtomicIrcNetwork = Arc<RwLock<IrcNetwork>>;

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
pub struct IrcNetwork {
	pub config: IrcdConfig,
	pub clients: HashMap<String, Client>,
	pub servers: HashMap<String, Server>,
	// pub channels: HashMap<String, Channel>,
}

#[derive(Debug)]
pub enum IrcNetworkError {
	IO(io::Error),

	Server(IrcServerError),
}

// -------------- //
// Implémentation //
// -------------- //

impl IrcNetwork {
	/// Crée un nouveau réseau IRC.
	pub fn new(config: &IrcdConfig) -> Result<Self, IrcNetworkError> {
		let network = Self {
			config: config.to_owned(),
			clients: Default::default(),
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

	fn shared(&self) -> AtomicIrcNetwork {
		Arc::new(RwLock::new(self.clone()))
	}
}

impl IrcNetwork {
	/// Tente d'établir les connexions entre les serveurs du réseau.
	pub async fn try_establish_connections(
		self,
	) -> Result<(), IrcNetworkError> {
		for (label, server) in self.servers.into_iter() {
			logger::info!(
				"Tentative d'établir la connexion au serveur « {} ».",
				label
			);

			tokio::spawn(async move {
				server.ping_host().await?;

				loop {
					let connection = server.try_establish_connection().await?;
					server.intercept_messages(connection).await;
				}

				#[allow(unreachable_code)]
				Ok::<(), IrcNetworkError>(())
			});
		}

		loop {
			tokio::time::sleep(std::time::Duration::from_secs(1)).await;
		}
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
