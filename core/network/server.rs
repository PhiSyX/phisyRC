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
pub type NotifierReader<T> = mpsc::UnboundedReceiver<T>;
pub type NotifierWriter<T> = mpsc::UnboundedSender<T>;

// --------- //
// Interface //
// --------- //

#[async_trait::async_trait]
pub trait Interface: Send {
	type Argument: Send;
	type Session: session::Interface;

	async fn accept(
		&mut self,
		socket: Socket,
		addr: SocketAddr,
	) -> Result<session::Session<<Self::Session as session::Interface>::ID>>;

	async fn close(
		&mut self,
		id: <Self::Session as session::Interface>::ID,
	) -> Result<()>;

	async fn notify(&mut self, argument: Self::Argument) -> Result<()>;
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
	notifier: NotifierWriter<I::Argument>,
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
pub struct Outgoing<I>
where
	I: Interface,
{
	id: <I::Session as session::Interface>::ID,
	reason: Result<Option<Reason>>,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Reason {
	pub code: u16,
	pub reason: String,
}

pub struct Actor<I>
where
	I: Interface,
{
	pub incoming: IncomingReader<I>,
	pub outgoing: OutgoingReader<I>,
	pub notifier: NotifierReader<I::Argument>,
	pub server_instance: Server<I>,
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
		tcp_addr: impl ToSocketAddrs,
		ws_addr: impl ToSocketAddrs,
		ctor: impl FnOnce(Self) -> I,
	) -> Result<Self> {
		let (incoming_sender, incoming_receiver) = mpsc::unbounded_channel();
		let (outgoing_sender, outgoing_receiver) = mpsc::unbounded_channel();
		let (notifier_sender, notifier_receiver) = mpsc::unbounded_channel();

		let this = Self {
			incoming: incoming_sender.clone(),
			outgoing: outgoing_sender,
			notifier: notifier_sender,
		};

		let instance = ctor(this.clone());

		let actor = Actor {
			incoming: incoming_receiver,
			outgoing: outgoing_receiver,
			notifier: notifier_receiver,
			server_instance: this.clone(),
			user_instance: instance.clone(),
		};

		tokio::spawn({
			let listener = TcpListener::bind(tcp_addr).await?;
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

		tokio::spawn({
			let listener = TcpListener::bind(ws_addr).await?;
			let server = this.clone();

			async move {
				logger::info!(
					"En attente de connexion au serveur WebSocket « {} »",
					listener.local_addr()?
				);

				loop {
					let (socket_stream, socket_addr) =
						listener.accept().await?;
					let websocket =
						tokio_tungstenite::accept_async(socket_stream).await?;
					let socket =
						Socket::new(websocket.map_err(Error::WebSocket));
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

	pub fn close(
		&self,
		session_id: <I::Session as session::Interface>::ID,
		reason: Result<Option<Reason>, Error>,
	) {
		_ = self.outgoing.send(Outgoing {
			id: session_id,
			reason,
		})
	}

	pub fn notify(&self, argument: I::Argument) {
		_ = self.notifier.send(argument);
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
	async fn receiver_task(mut self) -> Result<()> {
		loop {
			tokio::select! {
			Some(Incoming { socket, addr, respond }) = self.incoming.recv() => {
				logger::info!("Connexion accepté: « {} »", addr);
				let session = self.user_instance.accept(socket, addr).await?;
				_ = respond.send(session.id.clone());
				tokio::spawn({
					let server = self.server_instance.clone();
					async move {
						let reason = session.close().await;
						server.close(session.id, reason);
					}
				});
			}

			Some(Outgoing { id, reason }) = self.outgoing.recv() => {
				self.user_instance.close(id.clone()).await?;
				match reason {
					| Ok(Some(Reason { code, reason })) => {
						if reason.is_empty() {
							logger::info!("[{id}]: connexion fermée ({code})");
						} else {
							logger::info!("[{id}]: connexion fermée: {reason} ({code})");
						}
					},
					| Ok(_) => {
						logger::info!("[{id}]: connexion fermée");
					}
					| Err(err) => {
						logger::error!("[{id}]: connexion fermée: {err}");
					},
				}
			}
			Some(argument) = self.notifier.recv() => {
				self.user_instance.notify(argument).await?
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
			notifier: self.notifier.clone(),
		}
	}
}
