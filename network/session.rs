/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;
use std::{hash, sync::Arc};

use async_trait::async_trait;
use shared::AnyError;
use tokio::sync::{mpsc, oneshot, Mutex};

use crate::{server::OutgoingReason, Socket, SocketMessage};

// --------- //
// Interface //
// --------- //

#[async_trait]
pub trait SessionInterface: Send {
	type ID: Clone
		+ Sync
		+ Send
		+ PartialEq
		+ Eq
		+ hash::Hash
		+ fmt::Display
		+ fmt::Debug;
	type Parameters: fmt::Debug + Send;

	async fn text(&mut self, text: SocketMessage) -> Result<(), AnyError>;

	async fn notice(
		&mut self,
		parameters: Self::Parameters,
	) -> Result<(), AnyError>;
}

// --------- //
// Structure //
// --------- //

type MaybeQuitEvent = Result<Option<OutgoingReason>, AnyError>;
type QuitEventReader = Option<oneshot::Receiver<MaybeQuitEvent>>;
type SharedQuitEventReader = Arc<Mutex<QuitEventReader>>;

#[derive(Debug)]
pub struct Session<Interface, Parameters>
where
	Interface: Clone,
	Interface: fmt::Display,
	Parameters: fmt::Debug,
{
	pub id: Interface,
	parameters_event: mpsc::UnboundedSender<Parameters>,
	message_event: mpsc::UnboundedSender<SocketMessage>,
	quit_event: SharedQuitEventReader,
}

struct SessionActor<Interface>
where
	Interface: SessionInterface,
{
	pub session: Interface,
	id: Interface::ID,
	message_event: mpsc::UnboundedReceiver<SocketMessage>,
	parameters_event: mpsc::UnboundedReceiver<Interface::Parameters>,
	socket: Socket,
}

// -------------- //
// Implémentation //
// -------------- //

impl<I, P> Session<I, P>
where
	I: Send + Clone,
	I: fmt::Display,
	P: Send,
	P: fmt::Debug,
{
	pub fn new<T>(ctor: impl FnOnce(Self) -> T, sid: I, socket: Socket) -> Self
	where
		T: 'static,
		T: SessionInterface<ID = I, Parameters = P>,
	{
		let (socket_sender, socket_receiver) = mpsc::unbounded_channel();
		let (parameters_sender, parameters_receiver) =
			mpsc::unbounded_channel();
		let (quit_sender, quit_receiver) = oneshot::channel();

		let handle = Self {
			id: sid.clone(),
			message_event: socket_sender,
			parameters_event: parameters_sender,
			quit_event: Arc::new(Mutex::new(Some(quit_receiver))),
		};

		let session = ctor(handle.clone());
		let mut actor = SessionActor {
			session,
			id: sid,
			message_event: socket_receiver,
			parameters_event: parameters_receiver,
			socket,
		};

		tokio::spawn(async move {
			let result = actor.run().await;
			quit_sender.send(result).unwrap();
		});

		handle
	}

	pub async fn quit(&self) -> Result<Option<OutgoingReason>, AnyError> {
		self.quit_event
			.lock()
			.await
			.take()
			.expect("l'événement a déjà été appelé.")
			.await
			.unwrap()
	}

	pub async fn text(&self, text: SocketMessage) {
		self.message_event.send(text).unwrap();
	}
}

impl<E: SessionInterface> SessionActor<E> {
	async fn run(&mut self) -> Result<Option<OutgoingReason>, AnyError> {
		loop {
			tokio::select! {
				Some(message) = self.message_event.recv() => {
					self.socket.send(message);
				}
				Some(parameters) = self.parameters_event.recv() => {
					self.session.notice(parameters).await?;
				}
				message = self.socket.recv() => {
				match message {
					Some(Ok(message)) => {
						self.session.text(message).await?;
					}
					Some(Err(err)) => {
						logger::error!("erreur de connexion: {err} ({})", self.id);
					}
					None => break
				};
				}
				else => break,
			}
		}
		Ok(None)
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl<I, P> Clone for Session<I, P>
where
	I: Send + Clone,
	I: fmt::Display,
	P: Send,
	P: fmt::Debug,
{
	fn clone(&self) -> Self {
		Self {
			id: self.id.clone(),
			parameters_event: self.parameters_event.clone(),
			message_event: self.message_event.clone(),
			quit_event: self.quit_event.clone(),
		}
	}
}
