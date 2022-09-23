/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::ops;

use futures::StreamExt;
use tokio::{sync::mpsc, task};

use super::message::SocketMessageRaw;
use crate::{SocketError, SocketMessage};

// --------- //
// Structure //
// --------- //

pub struct SocketStream {
	reader: mpsc::UnboundedReceiver<Result<SocketMessage, SocketError>>,
}

#[derive(Debug)]
struct SocketStreamActor<Message, Stream>
where
	Message: Into<SocketMessageRaw>,
	Stream: Unpin,
	Stream: StreamExt<Item = Result<Message, SocketError>>,
{
	stream: Stream,
	writer: mpsc::UnboundedSender<Result<SocketMessage, SocketError>>,
}

// -------------- //
// Implémentation //
// -------------- //

impl SocketStream {
	pub fn new<M, S>(
		stream: S,
	) -> (Self, task::JoinHandle<Result<(), SocketError>>)
	where
		M: 'static,
		M: Send,
		M: core::fmt::Debug,
		M: Into<SocketMessageRaw>,

		S: 'static,
		S: Unpin + Send,
		S: StreamExt<Item = Result<M, SocketError>>,
	{
		let (sender, receiver) = mpsc::unbounded_channel();

		let mut actor = SocketStreamActor {
			writer: sender,
			stream,
		};

		let task = tokio::spawn(async move { actor.run().await });

		(Self { reader: receiver }, task)
	}
}

impl<M, S> SocketStreamActor<M, S>
where
	M: Into<SocketMessageRaw>,
	S: Unpin,
	S: StreamExt<Item = Result<M, SocketError>>,
{
	async fn run(&mut self) -> Result<(), SocketError> {
		while let Some(result) = self.stream.next().await {
			let result = result.map(M::into);
			logger::trace!("message reçu: {:?}", result);
			let message = Ok(match result {
				| Ok(msg) => match msg {
					| SocketMessageRaw::Text(t) => SocketMessage::Text(t),
					| SocketMessageRaw::Binary(bytes) => {
						SocketMessage::Binary(bytes)
					}
					| SocketMessageRaw::Ping(_) => continue,
					| SocketMessageRaw::Pong(_) => todo!("pong"),
					| SocketMessageRaw::Close(_) => break,
				},
				| Err(err) => return Err(err),
			});
			let _ = self.writer.send(message);
		}
		Ok(())
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl ops::Deref for SocketStream {
	type Target = mpsc::UnboundedReceiver<Result<SocketMessage, SocketError>>;

	fn deref(&self) -> &Self::Target {
		&self.reader
	}
}

impl ops::DerefMut for SocketStream {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.reader
	}
}
