/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::{marker::PhantomData, ops};

use futures::SinkExt;
use tokio::{sync::mpsc, task};

use super::message::SocketMessageRaw;
use crate::SocketError;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
pub struct SocketSink {
	writer: mpsc::UnboundedSender<SocketMessageRaw>,
}

#[derive(Debug)]
struct SocketSinkActor<Message, Sink>
where
	Message: From<SocketMessageRaw>,
	Sink: Unpin,
	Sink: SinkExt<Message, Error = SocketError>,
{
	sink: Sink,
	reader: mpsc::UnboundedReceiver<SocketMessageRaw>,
	_marker: PhantomData<Message>,
}

// -------------- //
// Implémentation //
// -------------- //

impl SocketSink {
	pub fn new<M, S>(
		sink: S,
	) -> (Self, task::JoinHandle<Result<(), SocketError>>)
	where
		M: 'static,
		M: Send,
		M: From<SocketMessageRaw>,

		S: 'static,
		S: Send + Unpin,
		S: SinkExt<M, Error = SocketError>,
	{
		let (sender, receiver) = mpsc::unbounded_channel();

		let mut actor = SocketSinkActor {
			reader: receiver,
			sink,
			_marker: Default::default(),
		};

		let task = tokio::spawn(async move { actor.run().await });

		(Self { writer: sender }, task)
	}
}

impl<M, S> SocketSinkActor<M, S>
where
	M: From<SocketMessageRaw>,
	S: Unpin,
	S: SinkExt<M, Error = SocketError>,
{
	async fn run(&mut self) -> Result<(), SocketError> {
		while let Some(message) = self.reader.recv().await {
			logger::trace!("envoie du message: {:?}", message);
			self.sink.send(M::from(message)).await?;
		}
		Ok(())
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl ops::Deref for SocketSink {
	type Target = mpsc::UnboundedSender<SocketMessageRaw>;

	fn deref(&self) -> &Self::Target {
		&self.writer
	}
}

impl ops::DerefMut for SocketSink {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.writer
	}
}
