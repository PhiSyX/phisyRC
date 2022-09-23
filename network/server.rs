/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::net::SocketAddr;

use async_trait::async_trait;
use futures::TryStreamExt;
use shared::{err, AnyError};
use tokio::{
	net::TcpListener,
	sync::{mpsc, oneshot},
};
use tokio_tungstenite::{accept_async, tungstenite};
use tokio_util::codec::{Framed, LinesCodec};

use crate::{Session, SessionInterface, Socket, SocketError, SocketType};

// --------- //
// Interface //
// --------- //

#[async_trait]
pub trait ServerInterface: Send {
	type Session: SessionInterface;
	type Parameters: Send + core::fmt::Debug;

	async fn accept(
		&mut self,
		socket: Socket,
		addr: SocketAddr,
	) -> Result<
		Session<
			<Self::Session as SessionInterface>::ID,
			<Self::Session as SessionInterface>::Parameters,
		>,
	>;

	async fn notice(&mut self, parameters: Self::Parameters) -> Result<()>;
}

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
pub struct Server<Interface>
where
	Interface: ServerInterface,
{
	pub incoming: mpsc::UnboundedSender<Incoming<Interface>>,
	pub outgoing: mpsc::UnboundedSender<Outgoing<Interface>>,
	pub notifier: mpsc::UnboundedSender<Interface::Parameters>,
}

pub struct Actor<Interface>
where
	Interface: ServerInterface,
{
	pub incoming: mpsc::UnboundedReceiver<Incoming<Interface>>,
	pub outgoing: mpsc::UnboundedReceiver<Outgoing<Interface>>,
	pub notifier: mpsc::UnboundedReceiver<Interface::Parameters>,
	pub instance: Server<Interface>,
	pub user_instance: Interface,
}

pub struct Incoming<Interface>
where
	Interface: ServerInterface,
{
	pub socket: Socket,
	pub socket_type: SocketType,
	pub addr: SocketAddr,
	pub notifier: oneshot::Sender<<Interface::Session as SessionInterface>::ID>,
}

#[derive(Debug)]
pub struct Outgoing<Interface>
where
	Interface: ServerInterface,
{
	pub id: <Interface::Session as SessionInterface>::ID,
	pub reason: core::result::Result<Option<OutgoingReason>, AnyError>,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct OutgoingReason {
	pub code: u16,
	pub text: String,
}

// ----------- //
// Énumération //
// ----------- //

err! {
	| IO(std::io::Error) => "{}"
	| WebSocket(tungstenite::Error) => "erreur websocket -- {}"
}

// -------------- //
// Implémentation //
// -------------- //

impl<I> Server<I>
where
	I: 'static,
	I: Clone,
	I: ServerInterface,
{
	pub async fn new(
		ctor: impl FnOnce(Self) -> I,
		tcp_addr: SocketAddr,
		ws_addr: Option<SocketAddr>,
	) -> Result<Self> {
		let (incoming_sender, incoming_receiver) = mpsc::unbounded_channel();
		let (outgoing_sender, outgoing_receiver) = mpsc::unbounded_channel();
		let (notifier_sender, notifier_receiver) = mpsc::unbounded_channel();

		let this = Self {
			incoming: incoming_sender,
			outgoing: outgoing_sender,
			notifier: notifier_sender,
		};
		let instance = ctor(this.clone());

		let mut actor = Actor {
			incoming: incoming_receiver,
			outgoing: outgoing_receiver,
			instance: this.clone(),
			user_instance: instance.clone(),
			notifier: notifier_receiver,
		};

		#[allow(unreachable_code)]
		tokio::spawn({
			let listener = TcpListener::bind(&tcp_addr).await?;
			let mut server = instance.clone();
			async move {
				logger::info!(
					"En attente de connexion au serveur '{}'.",
					listener.local_addr()?
				);

				loop {
					let (socket_stream, socket_addr) =
						listener.accept().await?;
					let codec = Framed::new(socket_stream, LinesCodec::new());
					let socket = Socket::new(
						SocketType::Tcp,
						codec.map_err(SocketError::Codec),
					);
					server.accept(socket, socket_addr).await?;
				}
				Result::<()>::Ok(())
			}
		});

		// FIXME(phisyx): utiliser un proxy tcp <-> websocket
		if let Some(ws_addr) = ws_addr {
			#[allow(unreachable_code)]
			tokio::spawn({
				let listener = TcpListener::bind(ws_addr).await?;
				let mut server = instance.clone();
				async move {
					logger::info!(
						"En attente de connexion au serveur websocket '{}'.",
						listener.local_addr()?
					);

					loop {
						let (socket_stream, socket_addr) =
							listener.accept().await?;
						let websocket = accept_async(socket_stream).await?;
						let socket = Socket::new(
							SocketType::Ws,
							websocket.map_err(SocketError::WebSocket),
						);
						server.accept(socket, socket_addr).await?;
					}
					Result::<()>::Ok(())
				}
			});
		}

		tokio::spawn(async move { actor.receiver_task().await });

		Ok(this)
	}
}

impl<I> Server<I>
where
	I: ServerInterface,
{
	async fn quit(
		&self,
		id: <I::Session as SessionInterface>::ID,
		reason: core::result::Result<Option<OutgoingReason>, AnyError>,
	) {
		let _ = self.outgoing.send(Outgoing { id, reason });
	}

	pub async fn notice(&self, parameters: I::Parameters) {
		self.notifier.send(parameters).unwrap();
	}
}

impl<I> Actor<I>
where
	I: 'static,
	I: Send,
	I: ServerInterface,

	<I::Session as SessionInterface>::ID: Send,
{
	async fn receiver_task(&mut self) -> Result<()> {
		loop {
			tokio::select! {
			Some(Incoming { socket, socket_type: _, addr, notifier }) = self.incoming.recv() => {
				let session = self.user_instance.accept(socket, addr).await?;
				notifier.send(session.id.clone()).expect(
					"notifier le serveur de la présence d'un nouvel arrivant"
				);

				tokio::spawn({
					let server = self.instance.clone();
					async move {
						let reason = session.quit().await;
						server.quit(session.id, reason).await
					}
				});
			}

			Some(Outgoing { id, reason }) = self.outgoing.recv() => {
				dbg!("OUTGOING");

				// self.user_instance.quit(id.clone()).await?;
				match reason {
					| Ok(Some(OutgoingReason { code, text })) => {
						logger::info!("Connexion '{id}' fermée : {text} (code: {code})");
					}
					| Ok(None) => {
						logger::info!("Connexion '{id}' fermée");
					}
					| Err(err) => {
						logger::warn!("Connexion '{id}' fermée : {err}");
					},
				}
			}

			Some(parameters) = self.notifier.recv() => {
				self.user_instance.notice(parameters).await?
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
	I: 'static,
	I: Send,
	I: ServerInterface,
	<I::Session as SessionInterface>::ID: Send,
{
	fn clone(&self) -> Self {
		Self {
			incoming: self.incoming.clone(),
			outgoing: self.outgoing.clone(),
			notifier: self.notifier.clone(),
		}
	}
}
