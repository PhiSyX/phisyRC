/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::net::SocketAddr;

use futures::TryStreamExt;
use tokio::{
	net::{TcpListener, ToSocketAddrs},
	sync::{mpsc, oneshot},
};
use tokio_util::codec::{BytesCodec, Framed};

use crate::{session, socket::Socket, Error, Result};

// ---- //
// Type //
// ---- //

pub type IncomingReader<T> = mpsc::UnboundedReceiver<Incoming<T>>;
pub type IncomingWriter<T> = mpsc::UnboundedSender<Incoming<T>>;
pub type OutgoingReader<T> = mpsc::UnboundedReceiver<Outgoing<T>>;
pub type OutgoingWriter<T> = mpsc::UnboundedSender<Outgoing<T>>;

// --------- //
// Interface //
// --------- //

#[async_trait::async_trait]
pub trait Interface: Send {
	type Session: session::Interface;

	async fn accept(
		&mut self,
		socket: Socket,
		addr: SocketAddr,
	) -> Result<session::Session<<Self::Session as session::Interface>::ID>>;
}

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
pub struct Server<I>
where
	I: Interface,
{
	incoming: IncomingWriter<I>,
	outgoing: OutgoingWriter<I>,
}

pub struct Incoming<I>
where
	I: Interface,
{
	socket: Socket,
	addr: SocketAddr,
	respond: oneshot::Sender<<I::Session as session::Interface>::ID>,
}

#[derive(Debug)]
pub struct Outgoing<I> {
	// TODO(phisyx): ajouter un code et une raison lié à ce code
	_marker: I,
}

pub struct Actor<I>
where
	I: Interface,
{
	pub incoming: IncomingReader<I>,
	pub outgoing: OutgoingReader<I>,
	pub this_instance: Server<I>,
	pub user_instance: I,
}

// -------------- //
// Implémentation // -> API Publique
// -------------- //

impl<I> Server<I>
where
	I: 'static,
	I: Clone,
	I: Interface,
{
	pub async fn create(
		addr: impl ToSocketAddrs,
		ctor: impl FnOnce(Self) -> I,
	) -> Result<Self> {
		let (incoming_sender, incoming_receiver) = mpsc::unbounded_channel();
		let (outgoing_sender, outgoing_receiver) = mpsc::unbounded_channel();

		let this = Self {
			incoming: incoming_sender.clone(),
			outgoing: outgoing_sender,
		};

		let instance = ctor(this.clone());

		let actor = Actor {
			incoming: incoming_receiver,
			outgoing: outgoing_receiver,
			this_instance: this.clone(),
			user_instance: instance.clone(),
		};

		tokio::spawn({
			let listener = TcpListener::bind(addr).await?;
			let server = this.clone();

			async move {
				logger::info!(
					"En attente de connexion au serveur « {} »",
					listener.local_addr()?
				);

				loop {
					let (socket_stream, socket_addr) =
						listener.accept().await?;
					let codec = Framed::new(socket_stream, BytesCodec::new());
					let socket = Socket::new(codec.map_err(Error::IO));
					server.accept(socket, socket_addr).await;
				}

				#[allow(unreachable_code)]
				Result::<()>::Ok(())
			}
		});

		tokio::spawn(actor.receiver_task());

		Ok(this)
	}
}

// -------------- //
// Implémentation // -> API Publique
// -------------- //

impl<I> Server<I>
where
	I: Interface,
{
	pub async fn accept(
		&self,
		socket: Socket,
		addr: SocketAddr,
	) -> <I::Session as session::Interface>::ID {
		let (writer, reader) = oneshot::channel();

		_ = self.incoming.send(Incoming {
			socket,
			addr,
			respond: writer,
		});

		reader.await.unwrap()
	}
}

// -------------- //
// Implémentation //
// -------------- //

impl<I> Actor<I>
where
	I: 'static,
	I: Send,
	I: Interface,
{
	// TODO(phisyx): gérer la déconnexion
	async fn receiver_task(mut self) -> Result<()> {
		loop {
			tokio::select! {
			Some(Incoming { socket, addr, respond }) = self.incoming.recv() => {
				logger::info!("Connexion accepté: « {} »", addr);
				let session = self.user_instance.accept(socket, addr).await?;
				_= respond.send(session.id);
			}

			Some(Outgoing { .. }) = self.outgoing.recv() => {
				logger::warn!("Gérer la déconnexion...");
			}
			}
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<I> Clone for Server<I>
where
	I: Interface,
{
	fn clone(&self) -> Self {
		Self {
			incoming: self.incoming.clone(),
			outgoing: self.outgoing.clone(),
		}
	}
}
