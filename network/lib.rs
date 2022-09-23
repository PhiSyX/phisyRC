/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![allow(clippy::new_without_default)]

mod export;
mod server;
mod session;
mod socket;

use std::{collections::HashMap, net::SocketAddr};

pub use self::export::*;

// --------- //
// Structure //
// --------- //

pub struct Network<Interface>
where
	Interface: ServerInterface,
{
	servers: HashMap<
		<Interface::Session as SessionInterface>::ID,
		Server<Interface>,
	>,
}

// -------------- //
// Implémentation //
// -------------- //

impl<I> Network<I>
where
	I: 'static,
	I: ServerInterface,
	I: Clone,
{
	pub fn new() -> Self {
		Self {
			servers: Default::default(),
		}
	}

	pub async fn create_server(
		&mut self,
		id: <I::Session as SessionInterface>::ID,
		tcp_addr: SocketAddr,
		maybe_ws_addr: Option<SocketAddr>,
		ctor: impl FnOnce(Server<I>) -> I,
	) -> Result<()> {
		logger::info!("tentative de connexion au serveur '{id}'.");
		let server = Server::new(ctor, tcp_addr, maybe_ws_addr).await?;
		self.servers.insert(id, server);
		Ok(())
	}
}
