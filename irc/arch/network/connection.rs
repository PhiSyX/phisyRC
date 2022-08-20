/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::{net::SocketAddr, sync::Arc};

use tokio::{
	io::{AsyncBufReadExt, BufReader, WriteHalf},
	net::TcpStream,
	sync::{mpsc::UnboundedSender, RwLock},
};

use crate::message::{IrcMessage, IrcMessageError};

// ---- //
// Type //
// ---- //

pub(crate) type AtomicConnection = Arc<RwLock<Connection>>;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
pub(crate) struct Connection {
	pub stream: TcpStream,
	pub addr: SocketAddr,
	pub writer: Option<WriteHalf<TcpStream>>,
}

// -------------- //
// Implémentation //
// -------------- //

impl Connection {
	pub(super) fn new(stream: TcpStream, addr: SocketAddr) -> Self {
		Self {
			stream,
			addr,
			writer: None,
		}
	}

	// TODO(phisyx): améliorer cette partie-ci.
	pub(crate) async fn read_messages(
		&mut self,
		tx: UnboundedSender<Result<IrcMessage, IrcMessageError>>,
	) {
		let (reader, writer) = tokio::io::split(&mut self.stream);

		// self.writer.replace(writer);

		let mut buffer = String::new();
		let mut buf_reader = BufReader::new(reader);

		loop {
			match buf_reader.read_line(&mut buffer).await {
				Ok(0) => {
					logger::trace!("Quit message");
					break;
				}

				Ok(size) => {
					let messages = IrcMessage::lines(&buffer[..size]);

					messages
						.into_iter()
						.for_each(|message| tx.send(message).unwrap());
				}

				Err(err) => {
					logger::error!("Erreur: {:?}", err);
					break;
				}
			}

			// NOTE(phisyx): nous devons vider le tampon de lecture pour
			// éviter de se retrouver avec d'anciens messages.
			buffer.clear();
		}
	}

	pub(crate) fn shared(self) -> AtomicConnection {
		Arc::new(RwLock::new(self))
	}
}
