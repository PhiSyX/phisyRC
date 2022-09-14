/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::{net::SocketAddr, sync::Arc};

use tokio::sync::Mutex;

use super::server::AtomicServer;
use crate::{
	arch::{IrcClient, IrcServer},
	commands::{IncomingUnregisteredCommand, IrcCommandNumeric},
};

// ---- //
// Type //
// ---- //

pub type AtomicPeer = Arc<Mutex<Peer>>;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
pub struct Peer {
	pub addr: SocketAddr,
	pub server: AtomicServer,
	pub ty: Option<EntityType>,
}

#[derive(Debug)]
#[derive(Clone)]
pub enum EntityType {
	Client(IrcClient),
	Server(IrcServer),
}

// -------------- //
// Implémentation //
// -------------- //

impl Peer {
	pub(crate) fn new(server: AtomicServer, addr: SocketAddr) -> Self {
		Self {
			addr,
			server,
			ty: Default::default(),
		}
	}

	pub(crate) fn shared(&self) -> Arc<Mutex<Self>> {
		Arc::new(Mutex::new(self.clone()))
	}

	pub(crate) fn label(&self) -> &str {
		self.ty
			.as_ref()
			.map(|entity| entity.label())
			.unwrap_or_default()
	}

	pub(crate) fn is_registered(&self) -> bool {
		self.ty
			.as_ref()
			.map(|entity| entity.is_registered())
			.unwrap_or_default()
	}

	fn set_type(&mut self, ty: &'static str) {
		match ty {
			| "client" => {
				if !matches!(self.ty, Some(EntityType::Client(_))) {
					self.ty.replace(EntityType::Client(IrcClient::new(
						self.shared(),
					)));
				}
			}
			| "server" => {
				if !matches!(self.ty, Some(EntityType::Server(_))) {
					self.ty.replace(EntityType::Server(IrcServer::new()));
				}
			}
			| s => panic!("Définition du type {s} impossible"),
		};
	}

	pub(crate) async fn old_prefix(&self) -> String {
		let x = self.server.read().await.config.user.name.clone();
		self.ty
			.as_ref()
			.and_then(|ty| match ty {
				| EntityType::Client(_) => {
					ty.old_prefix().map(|p| format!("{}@{}", p, self.addr))
				}
				| EntityType::Server(_) => ty.old_prefix(),
			})
			.unwrap_or(x)
	}

	pub(crate) async fn prefix(&self) -> String {
		let x = self.server.read().await.config.user.name.clone();
		self.ty
			.as_ref()
			.and_then(|ty| match ty {
				| EntityType::Client(_) => {
					ty.prefix().map(|p| format!("{}@{}", p, self.addr))
				}
				| EntityType::Server(_) => ty.prefix(),
			})
			.unwrap_or(x)
	}

	pub(crate) fn prefix_based_on_reply(
		&self,
		reply: &IrcCommandNumeric,
	) -> String {
		self.ty
			.as_ref()
			.map(|ty| ty.prefix_based_on_reply(reply))
			.unwrap_or_else(|| "*".to_owned())
	}
}

impl Peer {
	/// Gestion de la commande PASS.
	///
	/// Un client n'est censé envoyer qu'un (1) seul argument pour la commande.
	/// Quant à un serveur, il DOIT obligatoirement envoyer quatres (4)
	/// arguments pour la commande.
	pub(crate) fn handle_pass_command(
		&mut self,
		command: &IncomingUnregisteredCommand,
	) -> Result<(), IrcCommandNumeric> {
		assert!(matches!(command, IncomingUnregisteredCommand::PASS { .. }));

		if let IncomingUnregisteredCommand::PASS { parameters } = command {
			if parameters.is_empty() {
				return Err(IrcCommandNumeric::ERR_NEEDMOREPARAMS {
					command: command.to_string(),
				});
			}

			let size = parameters.len();

			// NOTE(phisyx): on pourrait bloquer la connexion lorsqu'il y a plus
			// de 4 arguments, mais on va le considérer comme un client
			if size == 1 || size > 4 {
				self.set_type("client");
			} else if size == 4 {
				self.set_type("server");
			}
		}

		Ok(())
	}

	/// Gestion de la commande NICK.
	///
	/// Un client n'est censé envoyer qu'un (1) seul argument pour la commande.
	/// Quant à un serveur, il DOIT obligatoirement envoyer sept (7) arguments
	/// pour la commande.
	pub(crate) fn handle_nick_command(
		&mut self,
		command: &IncomingUnregisteredCommand,
	) -> Result<(), IrcCommandNumeric> {
		assert!(matches!(command, IncomingUnregisteredCommand::NICK { .. }));

		if let IncomingUnregisteredCommand::NICK { parameters } = command {
			if parameters.is_empty() {
				return Err(IrcCommandNumeric::ERR_NONICKNAMEGIVEN);
			}

			let size = parameters.len();

			if size == 1 || size > 7 {
				self.set_type("client");
			} else if parameters.len() == 7 {
				self.set_type("server");
			}
		}

		Ok(())
	}

	/// Gestion de la commande USER.
	///
	/// Un client est censé envoyer quatres (4) arguments pour la commande.
	/// Quant à un serveur, il ne DOIT en aucun cas envoyer cette commande.
	pub(crate) fn handle_user_command(
		&mut self,
		command: &IncomingUnregisteredCommand,
	) -> Result<(), IrcCommandNumeric> {
		assert!(matches!(command, IncomingUnregisteredCommand::USER { .. }));

		if let IncomingUnregisteredCommand::USER { parameters } = command {
			if parameters.is_empty() {
				return Err(IrcCommandNumeric::ERR_NEEDMOREPARAMS {
					command: command.to_string(),
				});
			}

			// NOTE(phisyx): le type de la connexion a déjà été déduit par les
			// précédente commandes (PASS / NICK).
			if let Some(EntityType::Server(_)) = self.ty {
				return Err(IrcCommandNumeric::ERR_UNKNOWNCOMMAND {
					command: command.to_string(),
				});
			}

			if parameters.len() == 4 {
				self.set_type("client");
			}
		}

		Ok(())
	}

	pub(crate) fn handle_server_command(
		&mut self,
		command: &IncomingUnregisteredCommand,
	) -> Result<(), IrcCommandNumeric> {
		assert!(matches!(
			command,
			IncomingUnregisteredCommand::SERVER { .. }
		));

		if let IncomingUnregisteredCommand::SERVER { parameters } = command {
			if parameters.is_empty() {
				return Err(IrcCommandNumeric::ERR_NEEDMOREPARAMS {
					command: command.to_string(),
				});
			}

			// NOTE(phisyx): le type de la connexion a déjà été déduit par les
			// précédente commandes.
			if let Some(EntityType::Client(_)) = self.ty {
				return Err(IrcCommandNumeric::ERR_UNKNOWNCOMMAND {
					command: command.to_string(),
				});
			}

			if parameters.len() == 4 {
				self.set_type("server");
			}
		}

		Ok(())
	}
}

impl EntityType {
	fn label(&self) -> &str {
		match self {
			| EntityType::Client(client) => client.nick.as_ref(),
			| EntityType::Server(server) => server.label.as_ref(),
		}
	}

	fn is_registered(&self) -> bool {
		match self {
			| EntityType::Client(client) => client.is_registered(),
			| EntityType::Server(server) => server.is_registered(),
		}
	}

	fn old_prefix(&self) -> Option<String> {
		match self {
			| Self::Client(client) => client.old_prefix(),
			| Self::Server(server) => server.prefix(),
		}
	}

	fn prefix(&self) -> Option<String> {
		match self {
			| Self::Client(client) => client.prefix(),
			| Self::Server(server) => server.prefix(),
		}
	}

	fn prefix_based_on_reply(&self, reply: &IrcCommandNumeric) -> String {
		match self {
			| EntityType::Client(client) => client.prefix_based_on_reply(reply),
			| EntityType::Server(server) => server.prefix_based_on_reply(reply),
		}
	}
}
