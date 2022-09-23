/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod message;
mod sink;
mod stream;
mod types;
pub mod ws;

use futures::{SinkExt, StreamExt, TryStreamExt};
use shared::err;
use tokio_tungstenite::tungstenite;

pub use self::{message::*, types::*};
use self::{sink::SocketSink, stream::SocketStream};

// --------- //
// Structure //
// --------- //

pub struct Socket {
	pub sink: SocketSink,
	pub stream: SocketStream,
	pub ty: SocketType,
}

// ----------- //
// Énumération //
// ----------- //

err! {
	| IO(std::io::Error) => "{}"
	| Codec(tokio_util::codec::LinesCodecError) => "line codec -- {}"
	| WebSocket(tungstenite::Error) => "WebSocket -- {}"
	| Ignore(String) => "{}"
}

pub type SocketError = Error;

// -------------- //
// Implémentation //
// -------------- //

impl Socket {
	pub fn new<GenericMessage, GenericError: std::error::Error, GenericSocket>(
		socket_type: SocketType,
		socket: GenericSocket,
	) -> Self
	where
		GenericMessage: 'static,
		GenericMessage: Send,
		GenericMessage: core::fmt::Debug,
		GenericMessage: Into<SocketMessageRaw> + From<SocketMessageRaw>,

		GenericError: Into<SocketError>,

		GenericSocket: 'static,
		GenericSocket: Send + Unpin,
		GenericSocket: SinkExt<GenericMessage, Error = GenericError>
			+ StreamExt<Item = Result<GenericMessage>>,
	{
		let (sink, stream) = socket.sink_err_into().err_into().split();
		let ((sink, sink_task), (stream, stream_task)) =
			(SocketSink::new(sink), SocketStream::new(stream));

		// todo: ping / pong event

		tokio::spawn(async move {
			let result = stream_task
				.await
				.map_err(|_| SocketError::Ignore("stream task".to_owned()))?;
			sink_task.abort();
			result
		});

		Self {
			sink,
			stream,
			ty: socket_type,
		}
	}

	pub fn send(&self, message: SocketMessage) {
		self.sink.send(message.into()).expect("l'envoie du message");
	}

	pub async fn recv(&mut self) -> Option<Result<SocketMessage>> {
		self.stream.recv().await
	}
}
