/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;

use crate::{
	session::{self, Session as AppSession, SessionID as AppSessionID},
	AppContextWriter, NetworkServer, NetworkSession, NetworkSessionInterface,
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
	type Session = session::Actor;

	async fn accept(
		&mut self,
		socket: network::Socket,
		addr: std::net::SocketAddr,
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
}
