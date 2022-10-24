/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::marker::PhantomData;

use futures::TryStreamExt;
use tokio::{
	net::{TcpStream, ToSocketAddrs},
	sync::mpsc,
};
use tokio_util::codec::{BytesCodec, Framed};

use crate::{
	socket::{OutgoingPacket, OutgoingPacketReader, OutgoingPacketWriter},
	Error, Result, Socket,
};

// --------- //
// Interface //
// --------- //

#[async_trait::async_trait]
pub trait Interface {
	async fn binary(&mut self, bytes: Vec<u8>) -> Result<()>;
}

// --------- //
// Structure //
// --------- //

pub struct Client<I>
where
	I: Interface,
{
	socket: OutgoingPacketWriter,
	_marker: PhantomData<I>,
}

struct Actor<I> {
	client: I,
	socket: Socket,
	reader: OutgoingPacketReader,
}

// -------------- //
// Implémentation // -> API Publique
// -------------- //

impl<I> Client<I>
where
	I: 'static,
	I: Send,
	I: Interface,
{
	pub async fn connect(
		addr: impl ToSocketAddrs,
		ctor: impl FnOnce(Self) -> I,
	) -> Result<Self> {
		let (socket_writer, socket_reader) = mpsc::unbounded_channel();

		let this = Self {
			socket: socket_writer,
			_marker: PhantomData,
		};

		let instance = ctor(this.clone());

		tokio::spawn({
			let socket_stream = TcpStream::connect(addr).await?;
			async move {
				let codec = Framed::new(socket_stream, BytesCodec::new());
				let socket = Socket::new(codec.map_err(Error::IO));

				let mut actor = Actor {
					client: instance,
					socket,
					reader: socket_reader,
				};

				actor.run().await?;

				Result::<()>::Ok(())
			}
		});

		Ok(this)
	}
}

impl<I> Client<I>
where
	I: Interface,
{
	pub async fn text(&self, text: String) {
		let bytes = text.as_bytes();
		_ = self.socket.send(OutgoingPacket::Bin(bytes.to_vec()));
	}

	pub async fn binary(&self, bytes: Vec<u8>) {
		_ = self.socket.send(OutgoingPacket::Bin(bytes));
	}
}

// -------------- //
// Implémentation //
// -------------- //

impl<I> Actor<I>
where
	I: Interface,
{
	async fn run(&mut self) -> Result<()> {
		loop {
			tokio::select! {
				Some(message) = self.reader.recv() => {
					self.socket.send(message.clone());
				}
				result = self.socket.stream.recv() => {
					match result {
						| Some(Ok(message)) => match message.to_owned() {
							| OutgoingPacket::Bin(bytes) => self.client.binary(bytes).await?,
						}
						Some(Err(err)) => {
							logger::error!("Erreur de la connexion: {err}");
						}
						None => {
							// TODO(phisyx): connexion fermée..
							break
						}
					};
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

impl<I> Clone for Client<I>
where
	I: Interface,
{
	fn clone(&self) -> Self {
		Self {
			socket: self.socket.clone(),
			_marker: Default::default(),
		}
	}
}
