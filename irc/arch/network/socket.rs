/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;
use std::{net::SocketAddr, ops};

use tokio::{
	net::{TcpListener, TcpStream},
	time,
};

use crate::message::IrcCodec;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
pub struct Socket {
	pub addr: SocketAddr,
}

#[derive(Debug)]
pub struct SocketStream(TcpStream, SocketAddr);

pub struct Listener {
	pub listener: TcpListener,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
pub enum ListenerError {
	IO(std::io::Error),
}

// -------------- //
// Implémentation //
// -------------- //

// NOTE(phisyx): Le terme chaussette est évidement utiliser pour le fun ;-)
impl Socket {
	/// Crée une nouvelle adresse chaussette à partir d'une IP et d'un port.
	pub fn new(
		ip: impl Into<String>,
		port: impl Into<u16>,
	) -> Result<Self, std::net::AddrParseError> {
		let host_port = format!("{}:{}", ip.into(), port.into());
		let addr = host_port.parse()?;
		Ok(Self { addr })
	}

	/// Crée un nouvelle connexion.
	pub(crate) async fn listen(&self) -> Result<SocketStream, ListenerError> {
		let listener = Listener::from(&self.addr).await?;

		logger::info!("Connexion au serveur « {} » ouverte.", self.addr);

		let mut backoff: u8 = 1;

		loop {
			match listener.accept().await {
				| Err(err) if backoff > 64 => {
					logger::error!(
						"Erreur d'acceptation de connexion sur {}: {}",
						self.addr,
						err
					);

					return Err(ListenerError::IO(err));
				}

				| Err(_) => {
					backoff *= 2;

					logger::warn!(
						"Tentative d'acceptation de connexion sur « {} » échouée, \
						nouvelle tentative dans {} secondes...",
						self.addr,
						backoff
					);

					time::sleep(time::Duration::from_secs(backoff as u64))
						.await;
				}

				| Ok((socket_stream, socket_addr)) => {
					logger::info!(
						"Client connecté « {} » sur le serveur « {} ».",
						socket_stream.peer_addr()?,
						self.addr
					);
					return Ok(SocketStream(socket_stream, socket_addr));
				}
			}
		}
	}
}

impl SocketStream {
	/// IRC Codec
	pub fn codec(self) -> IrcCodec<TcpStream> {
		IrcCodec::new(self.stream())
	}

	/// Flux de la chaussette.
	pub fn stream(self) -> TcpStream {
		self.0
	}

	/// Adresse de la chaussette.
	pub fn addr(&self) -> SocketAddr {
		self.1
	}

	/// Adresse pair.
	pub fn peer_addr(&self) -> SocketAddr {
		self.0.peer_addr().expect("peer address")
	}
}

impl Listener {
	/// Créer une connexion TCP à partir d'une adresse chaussette.
	pub async fn from(addr: &SocketAddr) -> Result<Self, ListenerError> {
		let listener = TcpListener::bind(addr).await?;
		Ok(Self { listener })
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl ops::Deref for Listener {
	type Target = TcpListener;

	fn deref(&self) -> &Self::Target {
		&self.listener
	}
}

impl From<std::io::Error> for ListenerError {
	fn from(err: std::io::Error) -> Self {
		Self::IO(err)
	}
}

impl fmt::Display for ListenerError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::IO(err) => err,
			}
		)
	}
}
