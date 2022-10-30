/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::fmt;
use std::{
	borrow::Cow,
	net::{IpAddr, SocketAddr},
	ops::{self, RangeFrom},
};

use helpers::algorithms::SHA1;
use irc_replies::{
	Command as IrcCommand, CommandUnregistered as IrcCommandUnregistered,
	Error as IrcError, IncomingCommand as IrcIncomingCommand,
	Numeric as IrcNumeric,
};
use lang::stream::ByteStream;

use crate::{
	server::Server as AppServer,
	AppContext,
	// AppContext,
	NetworkSession,
};

// ---- //
// Type //
// ---- //

pub type Result<T, E = IrcError> = core::result::Result<T, E>;

pub type SessionID = uuid::Uuid;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
pub struct Session {
	pub inner: NetworkSession<SessionID>,
	pub id: SessionID,
	pub server: AppServer,
	pub host: Host,
	pub pass: Option<String>,
	pub old_nick: Option<String>,
	pub nick: Option<String>,
	pub user: Option<String>,
	pub mode: Option<String>,
	pub realname: Option<String>,
	pub is_registered: bool,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Host {
	/// Adresse IP du client.
	pub ip_addr: IpAddr,
	/// Nom d'hôte masqué de l'adresse IP.
	pub cloaked: String,
	/// Nom d'hôte de l'adresse IP.
	pub raw: String,
	/// Nom d'hôte virtuel (perso) de l'utilisateur.
	pub virtual_host: Option<String>,
}

// ----------- //
// Énumération //
// ----------- //

// -------------- //
// Implémentation //
// -------------- //

impl Session {
	pub fn new(
		server_instance: AppServer,
		session_instance: NetworkSession<uuid::Uuid>,
		id: SessionID,
		addr: SocketAddr,
	) -> Self {
		Self {
			inner: session_instance,
			server: server_instance,
			id,
			host: Host::new(addr.ip()),
			pass: Default::default(),
			old_nick: Default::default(),
			nick: Default::default(),
			user: Default::default(),
			mode: Default::default(),
			realname: Default::default(),
			is_registered: Default::default(),
		}
	}

	fn prefix_based_on_reply<'a>(
		&'a self,
		numeric: &'a irc_replies::Numeric,
	) -> Cow<str> {
		use irc_replies::Numeric;
		if let Numeric::RPL_WELCOME { .. }
		| Numeric::RPL_YOURHOST { .. }
		| Numeric::RPL_CREATED { .. } = &numeric
		{
			Cow::from(unsafe { self.nick.as_ref().unwrap_unchecked() })
		} else if let Numeric::ERR_NICKNAMEINUSE { nick } = numeric {
			Cow::from(nick)
		} else {
			Cow::from(self.to_string())
		}
	}
}

impl Session {
	/// Gestion de la commande PASS (UNREGISTERED).
	///
	/// Un client n'est censé envoyer qu'un (1) seul argument pour la commande.
	fn handle_upass_command(&mut self, command: &IrcCommand) -> Result<()> {
		assert!(matches!(command, IrcCommand::PASS { .. }));

		// NOTE(phisyx): ignorer le fait que le client envoie un mot de passe
		// si le serveur n'a pas de mot de passe dans sa configuration.
		if self.server.config.password.is_none() {
			return Ok(());
		}

		if let IrcCommand::PASS { password, .. } = command {
			self.pass.replace(password.to_owned());
		}

		Ok(())
	}

	/// Gestion de la commande NICK (UNREGISTERED).
	///
	/// Un client n'est censé envoyer qu'un (1) seul argument pour la commande.
	fn handle_unick_command(&mut self, command: &IrcCommand) -> Result<()> {
		assert!(matches!(command, IrcCommand::NICK { .. }));

		if let IrcCommand::NICK { nickname, .. } = command {
			if self.server.can_locate_client(nickname) {
				return Err(IrcError::Numeric(IrcNumeric::ERR_NICKNAMEINUSE {
					nick: nickname.to_owned(),
				}));
			}

			self.old_nick.replace(nickname.clone());
			self.nick.replace(nickname.to_owned());
		}

		self.complete_registration();
		Ok(())
	}

	/// Gestion de la commande USER.
	///
	/// Un client est censé envoyer quatres (4) arguments pour la commande.
	fn handle_uuser_command(&mut self, command: &IrcCommand) -> Result<()> {
		assert!(matches!(command, IrcCommand::USER { .. }));

		if let IrcCommand::USER {
			user,
			mode,
			_unused,
			realname,
			..
		} = command
		{
			self.user.replace(user.to_owned());
			self.mode.replace(mode.to_owned());
			self.realname.replace(realname.to_owned());
		}

		self.complete_registration();
		Ok(())
	}
}

impl Session {
	/// Gestion de la commande PASS (REGISTERED).
	///
	/// On ne peut plus envoyer cette commande une fois enregistrée.
	fn handle_pass_command(&mut self, command: &IrcCommand) -> Result<()> {
		assert!(matches!(command, IrcCommand::PASS { .. }));
		Err(IrcError::Numeric(IrcNumeric::ERR_ALREADYREGISTRED))
	}

	/// Gestion de la commande NICK (REGISTERED).
	fn handle_nick_command(&mut self, command: &IrcCommand) -> Result<()> {
		assert!(matches!(command, IrcCommand::NICK { .. }));

		if let IrcCommand::NICK { nickname, .. } = command {
			if self.server.can_locate_client(nickname) {
				return Err(IrcError::Numeric(IrcNumeric::ERR_NICKNAMEINUSE {
					nick: nickname.to_owned(),
				}));
			}

			// TODO(phisyx): valider le pseudonyme.
			self.old_nick.replace(nickname.to_owned());
			self.nick.replace(nickname.to_owned());
		}

		Ok(())
	}

	/// Gestion de la commande USER (REGISTERED).
	///
	/// On ne peut plus envoyer cette commande une fois enregistrée.
	fn handle_user_command(&mut self, _: &IrcCommand) -> Result<()> {
		Err(IrcError::Numeric(IrcNumeric::ERR_ALREADYREGISTRED))
	}

	fn complete_registration(&mut self) {
		match self.nick.as_ref().zip(self.user.as_ref()) {
			| Some((n, u)) => {
				if n.is_empty() || u.is_empty() {
					return;
				}
			}
			| _ => return,
		}

		if let Some((from_cfg, from_cmd)) =
			self.server.config.password.as_ref().zip(self.pass.as_ref())
		{
			logger::warn!(
				"{from_cfg} == {from_cmd} ? {}",
				from_cfg == from_cmd
			);
		}

		self.is_registered = true;

		let welcome_001 = IrcNumeric::RPL_WELCOME {
			nick: self.nick.to_owned().unwrap(),
			user: self.user.to_owned().unwrap(),
			host: self.host.to_string(),
		};
		let yourhost_002 = IrcNumeric::RPL_YOURHOST {
			servername: self.server.config.name.to_owned(),
			ver: "v1.0.0".into(),
		};
		let created_003 = IrcNumeric::RPL_CREATED {
			date: self
				.server
				.created_at
				.format("%Y-%m-%d %H:%M:%S.%f")
				.to_string(),
		};

		let replies = [welcome_001, yourhost_002, created_003];

		for reply in replies {
			self.server.notify(AppContext::Reply {
				id: Some(self.id),
				prefix: self.prefix_based_on_reply(&reply).to_string(),
				numeric: reply,
			});
		}
	}
}

impl Host {
	pub fn new(ip_addr: IpAddr) -> Self {
		let resolve_addr = dns_lookup::lookup_addr(&ip_addr)
			.unwrap_or_else(|_| String::from("localhost"));

		let cloaked = Self::get_cloaked_ip(&resolve_addr, 1..);
		let raw = Self::get_cloaked_ip(&resolve_addr, 0..);

		Self {
			ip_addr,
			cloaked,
			raw,
			virtual_host: Default::default(),
		}
	}
}

impl Host {
	fn get_cloaked_ip(hostname: &str, rng: RangeFrom<usize>) -> String {
		const SEPARATOR: char = '.';

		hostname
			.split(SEPARATOR)
			.enumerate()
			.map(|(idx, part)| {
				if rng.contains(&idx) {
					let parsed: Result<u8, _> = part.parse();
					if parsed.is_err() {
						Cow::Owned(part.sha1_sliced(10..14))
					} else {
						Cow::Borrowed(part)
					}
				} else {
					Cow::Owned(part.sha1_sliced(2..10))
				}
			})
			.collect::<Vec<Cow<str>>>()
			.join(&SEPARATOR.to_string())
	}
} // -------------- //
  // Implémentation // -> Interface
  // -------------- //

#[network::async_trait]
impl network::session::Interface for Session {
	type ID = SessionID;

	async fn binary(&mut self, bytes: Vec<u8>) -> network::Result<()> {
		let bytes_stream = ByteStream::from(bytes);

		// Vérifie que le message est de type IRC
		if let Ok(message) = irc_msg::Message::parse_from(bytes_stream) {
			// Non enregistré...
			if !self.is_registered {
				let r = message
					.command
					.is_valid()
					.and_then(|_: IrcCommandUnregistered| {
						let real_command: irc_replies::Result<IrcCommand> =
							message.command.is_valid();
						real_command
					})
					.and_then(|command| match command {
						| IrcCommand::PASS { .. } => {
							self.handle_upass_command(&command)
						}
						| IrcCommand::NICK { .. } => {
							self.handle_unick_command(&command)
						}
						| IrcCommand::USER { .. } => {
							self.handle_uuser_command(&command)
						}
					});

				if let Err(irc_replies::Error::Numeric(numeric)) = r {
					let reply_id = Some(self.id);
					let reply_prefix =
						self.prefix_based_on_reply(&numeric).to_string();
					let reply_numeric = numeric;

					self.server.notify(AppContext::Reply {
						id: reply_id,
						prefix: reply_prefix,
						numeric: reply_numeric,
					});
				}

				return Ok(());
			}

			// Enregistré ...

			let r =
				message
					.command
					.is_valid()
					.and_then(|command| match command {
						| IrcCommand::PASS { .. } => {
							self.handle_pass_command(&command)
						}
						| IrcCommand::NICK { .. } => {
							self.handle_nick_command(&command)
						}
						| IrcCommand::USER { .. } => {
							self.handle_user_command(&command)
						}
					});

			if let Err(irc_replies::Error::Numeric(numeric)) = r {
				let reply_id = Some(self.id);
				let reply_prefix =
					self.prefix_based_on_reply(&numeric).to_string();
				let reply_numeric = numeric;

				self.server.notify(AppContext::Reply {
					id: reply_id,
					prefix: reply_prefix,
					numeric: reply_numeric,
				});
			}
		}

		Ok(())
	}
}

impl ops::Deref for Session {
	type Target = NetworkSession<uuid::Uuid>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl fmt::Display for Host {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if let Some(virtual_host) = self.virtual_host.as_ref() {
			return write!(f, "{virtual_host}");
		}
		write!(f, "{}", self.cloaked)
	}
}

impl fmt::Display for Session {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let h = &self.host;
		if let Some((n, u)) = self.nick.as_ref().zip(self.user.as_ref()) {
			return write!(f, "{n}!{u}@{h}");
		}
		write!(f, "{h}")
	}
}
