/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use core::{fmt, marker::PhantomData, ops};

use futures::{Future, SinkExt, StreamExt, TryStreamExt};
use tokio::{sync::mpsc, task};

use crate::server::Result;

// ---- //
// Type //
// ---- //

pub type IncomingPacketReader = mpsc::UnboundedReceiver<IncomingPacket>;
pub type IncomingPacketWriter = mpsc::UnboundedSender<IncomingPacket>;

pub type OutgoingPacketReader = mpsc::UnboundedReceiver<OutgoingPacket>;
pub type OutgoingPacketWriter = mpsc::UnboundedSender<OutgoingPacket>;

pub type MaybeOutgoingPacketReader =
	mpsc::UnboundedReceiver<Result<OutgoingPacket>>;
pub type MaybeOutgoingPacketWriter =
	mpsc::UnboundedSender<Result<OutgoingPacket>>;

// --------- //
// Structure //
// --------- //

pub struct Socket {
	sink: Sink,
	stream: Stream,
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Sink {
	writer: IncomingPacketWriter,
}

#[derive(Debug)]
struct SinkActor<P, S>
where
	P: From<IncomingPacket>,
	S: Unpin,
	S: SinkExt<P, Error = crate::Error>,
{
	sink: S,
	reader: IncomingPacketReader,
	_marker: PhantomData<P>,
}

pub struct Stream {
	reader: MaybeOutgoingPacketReader,
}

#[derive(Debug)]
struct StreamActor<P, S>
where
	P: Into<IncomingPacket>,
	S: Unpin,
	S: StreamExt<Item = Result<P>>,
{
	stream: S,
	writer: MaybeOutgoingPacketWriter,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Clone)]
pub enum IncomingPacket {
	Raw(String),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum OutgoingPacket {
	Raw(String),
}

// -------------- //
// Implémentation // -> API Publique
// -------------- //

impl Socket {
	pub fn new<P, E, S>(socket: S) -> Self
	where
		P: 'static,
		P: Send,
		P: fmt::Debug,
		P: From<IncomingPacket> + Into<IncomingPacket>,

		E: std::error::Error,
		E: Into<crate::Error>,

		S: 'static,
		S: Send + Unpin,
		S: SinkExt<P, Error = E> + StreamExt<Item = Result<P>>,
	{
		let (write, read) = socket.sink_err_into().err_into().split();
		let (sink, sink_task) = Sink::new(write);
		let (stream, stream_task) = Stream::new(read);

		tokio::spawn(async move {
			let result = stream_task.await;
			sink_task.abort();
			result
		});

		Self { sink, stream }
	}

	pub fn recv(
		&mut self,
	) -> impl Future<Output = Option<Result<OutgoingPacket>>> + '_ {
		self.stream.recv()
	}

	pub fn send(&self, message: OutgoingPacket) {
		self.sink.send(message.into()).expect("packet");
	}
}

// -------------- //
// Implémentation //
// -------------- //

impl Sink {
	fn new<P, S>(sink: S) -> (Self, task::JoinHandle<Result<()>>)
	where
		P: 'static,
		P: Send,
		P: From<IncomingPacket>,

		S: 'static,
		S: Send + Unpin,
		S: SinkExt<P, Error = crate::Error>,
	{
		let (sender, receiver) = mpsc::unbounded_channel();

		let actor = SinkActor {
			reader: receiver,
			sink,
			_marker: Default::default(),
		};

		let task = tokio::spawn(actor.run());

		(Self { writer: sender }, task)
	}
}

impl<P, S> SinkActor<P, S>
where
	P: From<IncomingPacket>,
	S: Unpin,
	S: SinkExt<P, Error = crate::Error>,
{
	async fn run(mut self) -> Result<()> {
		while let Some(message) = self.reader.recv().await {
			logger::trace!("envoie du message: {:?}", message);
			self.sink.send(P::from(message)).await?;
		}
		Ok(())
	}
}

impl Stream {
	fn new<P, S>(stream: S) -> (Self, task::JoinHandle<Result<()>>)
	where
		P: 'static,
		P: Send,
		P: fmt::Debug,
		P: Into<IncomingPacket>,

		S: 'static,
		S: Unpin + Send,
		S: StreamExt<Item = Result<P>>,
	{
		let (sender, receiver) = mpsc::unbounded_channel();

		let actor = StreamActor {
			writer: sender,
			stream,
		};

		let task = tokio::spawn(actor.run());

		(Self { reader: receiver }, task)
	}
}

impl<P, S> StreamActor<P, S>
where
	P: Into<IncomingPacket>,
	S: Unpin,
	S: StreamExt<Item = Result<P>>,
{
	async fn run(mut self) -> Result<()> {
		while let Some(result) = self.stream.next().await {
			let result = result.map(P::into);
			logger::trace!("réception du message: {:?}", result);

			let message = Ok(match result {
				| Ok(msg) => match msg {
					| IncomingPacket::Raw(t) => OutgoingPacket::Raw(t),
				},
				| Err(err) => return Err(err),
			});

			_ = self.writer.send(message);
		}

		Ok(())
	}
}

// -------------- //
// Implémentation // -> From<T>
// -------------- //

impl From<String> for IncomingPacket {
	fn from(mut message: String) -> Self {
		message.push_str("\r\n");
		Self::Raw(message)
	}
}

impl From<IncomingPacket> for String {
	fn from(s: IncomingPacket) -> Self {
		if let IncomingPacket::Raw(s) = s {
			format!("{s}\r\n")
		} else {
			String::default()
		}
	}
}

impl From<OutgoingPacket> for IncomingPacket {
	fn from(packet: OutgoingPacket) -> Self {
		match packet {
			| OutgoingPacket::Raw(s) => Self::Raw(s),
		}
	}
}

impl ops::Deref for Sink {
	type Target = IncomingPacketWriter;

	fn deref(&self) -> &Self::Target {
		&self.writer
	}
}

impl ops::DerefMut for Sink {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.writer
	}
}

impl ops::Deref for Stream {
	type Target = MaybeOutgoingPacketReader;

	fn deref(&self) -> &Self::Target {
		&self.reader
	}
}

impl ops::DerefMut for Stream {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.reader
	}
}
