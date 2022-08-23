/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::{net::SocketAddr, sync::Arc};

use tokio::sync::Mutex;

use super::AtomicServer;
use crate::commands::{IncomingCommand, IrcCommandNumeric, IrcReplies};

// ---- //
// Type //
// ---- //

pub type AtomicClient = Arc<Mutex<Client>>;

// --------- //
// Structure //
// --------- //

/// Un client est tout ce qui se connecte à un serveur qui n'est pas un autre
/// serveur. Il existe deux types de clients qui ont chacun un objectif
/// différent.
#[derive(Debug)]
#[derive(Clone)]
pub struct Client {
	pub server: AtomicServer,
	pub addr: SocketAddr,

	pub nick: Option<String>,
	pub ty: Option<ClientType>,

	pub registered: bool,
	pub password: Option<String>,
	pub user: Option<String>,
	pub mode: Option<String>,
	pub realname: Option<String>,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum ClientType {
	/// Les clients utilisateurs sont généralement des programmes fournissant
	/// une interface textuelle qui est utilisée pour communiquer de manière
	/// interactive via IRC. Ce type particulier de clients est souvent appelé
	/// "utilisateurs".
	User,

	/// Contrairement aux utilisateurs, les clients de service ne sont pas
	/// destinés à être utilisés manuellement ni à discuter. Ils ont un accès
	/// plus limité aux fonctions de discussion du protocole, tout en ayant
	/// éventuellement accès à des données plus privées provenant des serveurs.
	///
	/// Les services sont généralement des automates utilisés pour fournir un
	/// certain type de service (pas nécessairement lié à l'IRC lui-même) aux
	/// utilisateurs. Un exemple est un service collectant des statistiques sur
	/// l'origine des utilisateurs connectés sur le réseau IRC.
	Service,
}

// -------------- //
// Implémentation //
// -------------- //

impl Client {
	pub(crate) fn new(server: AtomicServer, addr: SocketAddr) -> Self {
		Self {
			server,
			addr,
			ty: Default::default(),
			registered: Default::default(),
			nick: Default::default(),
			password: Default::default(),
			realname: Default::default(),
			mode: Default::default(),
			user: Default::default(),
		}
	}

	pub(crate) fn shared(self) -> Arc<Mutex<Self>> {
		Arc::new(Mutex::new(self))
	}

	pub(crate) fn is_registered(&self) -> bool {
		self.registered
	}
}

impl Client {
	pub(crate) fn on_pass_registration(
		&mut self,
		command: &IncomingCommand,
	) -> Result<&'static str, IrcCommandNumeric> {
		assert!(matches!(command, IncomingCommand::PASS { .. }));

		if let IncomingCommand::PASS { password, .. } = command {
			if password.is_empty() {
				return Err(IrcCommandNumeric::ERR_NEEDMOREPARAMS {
					command: command.to_string(),
				});
			}

			self.password.replace(password.to_owned());
		}

		Ok("Your server access password is taken into consideration.")
	}

	pub(crate) async fn on_nick_registration(
		&mut self,
		command: &IncomingCommand,
	) -> Result<&'static str, IrcCommandNumeric> {
		assert!(matches!(command, IncomingCommand::NICK { .. }));

		if let IncomingCommand::NICK { nickname, .. } = command {
			if nickname.is_empty() {
				return Err(IrcCommandNumeric::ERR_NONICKNAMEGIVEN);
			}

			if self.server.can_locate_nick(nickname).await {
				return Err(IrcCommandNumeric::ERR_NICKNAMEINUSE {
					nick: nickname.to_owned(),
				});
			}

			self.nick.replace(nickname.to_owned());
		}

		Ok("The nickname has been set")
	}

	pub(crate) async fn on_user_registration(
		&mut self,
		command: &IncomingCommand,
	) -> Result<&'static str, IrcCommandNumeric> {
		assert!(matches!(command, IncomingCommand::USER { .. }));

		if let IncomingCommand::USER {
			mode,
			realname,
			user,
			..
		} = command
		{
			self.mode.replace(mode.to_owned());
			self.realname.replace(realname.to_owned());
			self.user.replace(user.to_owned());
		}

		Ok("The user has been set")
	}

	pub(crate) async fn complete_registration(&mut self) -> Vec<IrcReplies> {
		self.registered = true;

		let server_config = self.server.config.clone();

		let mut replies = Vec::with_capacity(3);

		let welcome_001 = IrcCommandNumeric::RPL_WELCOME {
			nick: unsafe { self.nick.to_owned().unwrap_unchecked() },
			user: unsafe { self.user.to_owned().unwrap_unchecked() },
			host: self.addr.to_string(),
		};

		let yourhost_002 = IrcCommandNumeric::RPL_YOURHOST {
			servername: server_config.user.name.to_owned(),
			ver: "v0.1.0".into(),
		};

		let created_003 = IrcCommandNumeric::RPL_CREATED {
			date: self
				.server
				.created_at
				.format("%Y-%m-%d %H:%M:%S.%f")
				.to_string(),
		};

		replies.push(welcome_001);
		replies.push(yourhost_002);
		replies.push(created_003);

		replies.into_iter().map(IrcReplies::Numeric).collect()
	}
}
