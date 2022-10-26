/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;

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
}

// -------------- //
// Implémentation //
// -------------- //

impl Server {
	/// Crée un nouveau serveur.
	pub fn new(ctx: AppContextWriter, instance: NetworkServer<Self>) -> Self {
		Self {
			inner: instance,
			ctx,
			sessions: Default::default(),
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

#[network::async_trait]
impl network::server::Interface for Server {
	type Argument = AppContext;
	type Session = session::Actor;

	async fn accept(
		&mut self,
		socket: network::Socket,
		_addr: std::net::SocketAddr,
	) -> network::Result<
		NetworkSession<<Self::Session as NetworkSessionInterface>::ID>,
	> {
		let id = uuid::Uuid::new_v4();
		let session = NetworkSession::create(socket, id, |_| {
			session::Actor::new(self.inner.clone(), id)
		});
		self.sessions.insert(id, session.clone());
		Ok(session)
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
		// TODO(phisyx): gérer les autres cas...

		if let irc @ AppContext::IRC(_) = argument {
			_ = self.ctx.send(irc);
		}

		Ok(())
	}
}
