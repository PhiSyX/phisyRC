/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::fmt;

use tokio::sync::mpsc;

use crate::{
	socket::{OutgoingPacket, OutgoingPacketReader, OutgoingPacketWriter},
	Result, Socket,
};

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
pub struct Session<I>
where
	I: Clone,
{
	pub id: I,
	socket: OutgoingPacketWriter,
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
// Implémentation //
// -------------- //

impl<I> Session<I>
where
	I: Send,
	I: Clone,
{
	pub fn create<U>(
		socket: Socket,
		id: I,
		ctor: impl FnOnce(Self) -> U,
	) -> Self
	where
		U: 'static,
		U: Interface<ID = I>,
	{
		let (socket_sender, socket_receiver) = mpsc::unbounded_channel();

		let this = Self {
			id: id.clone(),
			socket: socket_sender,
		};

		let instance = ctor(this.clone());

		let actor = Actor::new(instance, id, socket_receiver, socket);
		tokio::spawn(actor.run());
		this
	}
}

impl<I> Actor<I>
where
	I: Interface,
{
	pub(crate) fn new(
		session: I,
		id: I::ID,
		packet_reader: OutgoingPacketReader,
		socket: Socket,
	) -> Self {
		Self {
			id,
			session,
			packet_reader,
			socket,
		}
	}

	async fn run(mut self) -> Result<()> {
		loop {
			tokio::select! {
				Some(message) = self.packet_reader.recv() => {
					self.socket.send(message.clone());
				}
				maybe_message = self.socket.recv() => {
					match maybe_message {
						| Some(Ok(message)) => match message {
							| OutgoingPacket::Bin(bytes) => self.session.binary(bytes).await?,
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
		Ok(())
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
			socket: self.socket.clone(),
		}
	}
}
