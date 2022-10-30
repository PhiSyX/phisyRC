/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::fmt;
use std::sync::Arc;

use futures::channel::oneshot;
use tokio::sync::{mpsc, Mutex};

use crate::{
	server::Reason,
	socket::{OutgoingPacket, OutgoingPacketReader, OutgoingPacketWriter},
	Result, Socket,
};

// ---- //
// Type //
// ---- //

type DisconnectedReader = oneshot::Receiver<Result<Option<Reason>>>;

// --------- //
// Interface //
// --------- //

#[async_trait::async_trait]
pub trait Interface: Send + Sync {
	type ID: Send + Sync + Clone + fmt::Debug + fmt::Display;

	async fn binary(&mut self, bytes: Vec<u8>) -> Result<()>;
}

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
pub struct Session<I> {
	pub id: I,
	packet_writer: OutgoingPacketWriter,
	outgoing: Arc<Mutex<Option<DisconnectedReader>>>,
}

struct Actor<I>
where
	I: Interface,
{
	pub session: I,
	id: I::ID,
	socket: Socket,
	packet_reader: OutgoingPacketReader,
}

// -------------- //
// Implémentation // -> API Publique
// -------------- //

impl<I> Session<I>
where
	I: Send,
	I: Clone,
{
	pub fn create<U>(socket: Socket, id: I, ctor: impl FnOnce(Self) -> U) -> U
	where
		U: 'static,
		U: Clone,
		U: Interface<ID = I>,
	{
		let (packet_sender, packet_receiver) = mpsc::unbounded_channel();
		let (outgoing_sender, outgoing_receiver) = oneshot::channel();

		let this = Self {
			id: id.clone(),
			packet_writer: packet_sender,
			outgoing: Arc::new(Mutex::new(Some(outgoing_receiver))),
		};

		let instance = ctor(this);

		let actor = Actor::new(instance.clone(), id, socket, packet_receiver);
		tokio::spawn(async move {
			let reason = actor.run().await;
			outgoing_sender.send(reason).unwrap()
		});
		instance
	}

	pub(crate) async fn close(&self) -> Result<Option<Reason>> {
		let mut outgoing = self.outgoing.lock().await;
		let reason = outgoing.take().unwrap();
		reason.await.unwrap()
	}

	pub fn binary(&self, bytes: Vec<u8>) {
		_ = self.packet_writer.send(OutgoingPacket::Bin(bytes));
	}

	pub fn text(&self, i: impl Into<OutgoingPacket>) {
		let o: OutgoingPacket = i.into();
		_ = self.packet_writer.send(o);
	}
}

impl<I> Actor<I>
where
	I: Interface,
{
	pub(crate) fn new(
		session: I,
		id: I::ID,
		socket: Socket,
		packet_reader: OutgoingPacketReader,
	) -> Self {
		Self {
			id,
			session,
			socket,
			packet_reader,
		}
	}

	async fn run(mut self) -> Result<Option<Reason>> {
		loop {
			tokio::select! {
				Some(message) = self.packet_reader.recv() => {
					self.socket.send(message.clone());
					if let OutgoingPacket::Quit(r) = message {
						return Ok(r)
					}
				}
				maybe_message = self.socket.recv() => {
					match maybe_message {
						| Some(Ok(message)) => match message {
							| OutgoingPacket::Bin(bytes) => self.session.binary(bytes).await?,
							| OutgoingPacket::Quit(reason) => {
								return Ok(reason.map(Reason::from))
							},
						}
						| Some(Err(err)) => {
							logger::error!("Erreur de connexion: {err} ({})", self.id.clone());
						}
						| None => break,
					}
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

impl<I> Clone for Session<I>
where
	I: Clone,
{
	fn clone(&self) -> Self {
		Self {
			id: self.id.clone(),
			packet_writer: self.packet_writer.clone(),
			outgoing: self.outgoing.clone(),
		}
	}
}
