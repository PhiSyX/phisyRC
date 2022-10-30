/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{collections::HashMap, ops, time::SystemTime};

use chrono::{DateTime, Utc};
use config::ServerConfig;

use crate::{
	session::{self, Session as AppSession, SessionID as AppSessionID},
	AppContext, AppContextWriter, NetworkServer, NetworkSession,
	NetworkSessionInterface,
};

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
pub struct Server {
	inner: NetworkServer<Self>,
	ctx: AppContextWriter,
	sessions: HashMap<AppSessionID, AppSession>,
	pub config: ServerConfig,
	pub created_at: DateTime<Utc>,
}

// -------------- //
// Implémentation //
// -------------- //

impl Server {
	/// Crée un nouveau serveur.
	pub fn new(
		ctx: AppContextWriter,
		instance: NetworkServer<Self>,
		config: ServerConfig,
	) -> Self {
		Self {
			inner: instance,
			ctx,
			sessions: Default::default(),
			config,
			created_at: DateTime::from(SystemTime::now()),
		}
	}
}

impl Server {
	pub fn can_locate_client(&self, nickname: &str) -> bool {
		self.sessions.iter().any(|(_, session)| {
			if let Some(nick) = &session.nick {
				return nick.to_lowercase() == nickname.to_lowercase();
			}
			false
		})
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

#[network::async_trait]
impl network::server::Interface for Server {
	type Argument = AppContext;
	type Session = session::Session;

	async fn accept(
		&mut self,
		socket: network::Socket,
		addr: std::net::SocketAddr,
	) -> network::Result<
		NetworkSession<<Self::Session as NetworkSessionInterface>::ID>,
	> {
		let id = uuid::Uuid::new_v4();
		let session = NetworkSession::create(socket, id, |instance| {
			session::Session::new(self.clone(), instance, id, addr)
		});
		self.sessions.insert(id, session.clone());
		Ok(session.inner)
	}

	async fn close(
		&mut self,
		id: <Self::Session as network::session::Interface>::ID,
	) -> network::Result<()> {
		self.sessions.remove(&id);
		Ok(())
	}

	async fn notify(
		&mut self,
		argument: Self::Argument,
	) -> network::Result<()> {
		match argument {
			| AppContext::Reply {
				id,
				prefix,
				numeric,
			} => {
				let msg = format!(
					":{} {} {prefix} {numeric}",
					self.config.name,
					numeric.code()
				);
				if let Some(id) = id {
					let session = self
						.sessions
						.iter()
						.find_map(
							|(sid, ses)| {
								if id.eq(sid) {
									Some(ses)
								} else {
									None
								}
							},
						)
						.unwrap();
					let msg = format!("@sid={id} {msg}");
					session.text(msg);
				} else {
					for (id, session) in self.sessions.iter() {
						let msg = format!("@sid={id} {msg}");
						session.text(msg);
					}
				}
			}

			| AppContext::Quit => {}
			| AppContext::InputFromTUI(_) => {}
		}

		Ok(())
	}
}

impl ops::Deref for Server {
	type Target = NetworkServer<Self>;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}
