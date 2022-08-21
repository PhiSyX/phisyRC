/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;
use std::{net::SocketAddr, sync::Arc};

use futures::{SinkExt, StreamExt};
use lang::stream::prelude::*;
use tokio::{
	io::{self},
	sync::RwLock,
};

// use super::Client;
use crate::{
	arch::{AtomicIrcNetwork, ListenerError, Socket, SocketStream},
	commands::IrcCommandNumeric,
	config::IrcdListen,
	message::{IrcCodec, IrcMessage, IrcMessageCommand, IrcMessageError},
};

// ---- //
// Type //
// ---- //

pub type AtomicServerConfig = Arc<RwLock<ServerConfig>>;

/// Type de drapeaux utilisateurs.
type ServerUserFlags = [char; 1];

// --------- //
// Structure //
// --------- //

/// Le serveur constitue l'épine dorsale d'IRC, car il est le seul composant du
/// protocole capable de relier tous les autres composants entre eux : il
/// fournit un point auquel les clients peuvent se connecter pour parler entre
/// eux [IRC-CLIENT], et un point auquel les autres serveurs peuvent se
/// connecter [IRC-SERVER]. Le serveur est également chargé de fournir les
/// services de base définis par le protocole IRC.
#[derive(Debug)]
#[derive(Clone)]
pub struct Server {
	pub network: AtomicIrcNetwork,

	/// Configuration du serveur.
	pub config: AtomicServerConfig,

	/// Les clients connectés au serveur.
	pub clients: Vec<SocketAddr>,

	/// Le nom du serveur.
	pub label: String,

	/// La socket du serveur.
	pub socket: Socket,
}

/// Cette structure contient les valeurs par défaut d'une configuration serveur
/// IRC.
#[derive(Debug)]
pub struct ServerConfig {
	/// Les drapeaux utilisateurs IRC par défaut.
	pub uflags: ServerUserFlags,

	/// La taille maximale du nom d'un serveur IRC.
	pub name_max_size: u8,

	/// Port `PLAINTEXT` IRC par défaut.
	///
	/// Dans le cas où le port n'est pas spécifié dans la configuration, cette
	/// valeur sera utilisée.
	pub port_plaintext: u16,

	/// Port `ENCRYPT` IRC par défaut.
	///
	/// Dans le cas où le port n'est pas spécifié dans la configuration TLS,
	/// cette valeur sera utilisée.
	pub port_encrypt: u16,

	pub user: IrcdListen,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
pub enum IrcServerError {
	AddrIsAlreadyEstablished(SocketAddr),
	IO(io::Error),
	Listener(ListenerError),
	ParseAddr(std::net::AddrParseError),
}

// -------------- //
// Implémentation //
// -------------- //

impl Server {
	/// La taille maximale du nom d'un serveur IRC.
	///
	/// Les noms de serveurs ont une longueur de soixante-trois (63) caractères.
	pub const NAME_MAX_SIZE: u8 = 63;
	/// Le port par défaut d'un port IRC 'ENCRYPT' est `6697`.
	///
	/// Un consensus similaire a été atteint au sein de la communauté IRC
	/// concernant l'écoute sur le port TCP 6697 pour les connexions entrantes
	/// cryptées via SSL/TLS (RFC 5246).
	pub const PORT_ENCRYPT: u16 = 6697;
	/// Le port par défaut d'un port IRC 'PLAINTEXT' est `6667`.
	///
	/// Les réseaux IRC écoutent par défaut sur le port 6667 pour les
	/// connexions de type `PLAINTEXT`. Ceci est couvert par l'assignation
	/// des ports TCP/UDP IRCU 6667-6669.
	pub const PORT_PLAINTEXT: u16 = 6667;
	/// Les drapeaux utilisateurs par défaut, qui sont appliqués à la connexion,
	/// sont les suivants : 'i'.
	pub const USER_FLAGS: ServerUserFlags = ['i'];

	/// Crée un serveur IRC.
	pub(crate) fn new(
		network: AtomicIrcNetwork,
		listen: &IrcdListen,
	) -> Result<Self, IrcServerError> {
		let config = ServerConfig::new(listen);
		let socket = Socket::new(&config.user.ip, config.user.port)?;
		let label = format!("{}:{}", config.user.ip, config.user.port);
		Ok(Self {
			network,
			config: Arc::new(RwLock::new(config)),
			clients: Default::default(),
			label,
			socket,
		})
	}

	/// Vérifie si une connexion vers une adresse est déjà établie.
	pub(crate) async fn ping_host(&self) -> Result<(), IrcServerError> {
		let addr = self.socket.addr;
		let stream = std::net::TcpStream::connect(addr);
		if stream.is_ok() {
			Err(IrcServerError::AddrIsAlreadyEstablished(addr))
		} else {
			Ok(())
		}
	}

	/// Écoute sur la socket du serveur.
	pub(crate) async fn try_establish_connection(
		&self,
	) -> Result<SocketStream, IrcServerError> {
		Ok(self.socket.listen().await?)
	}

	/// Intercepte les messages entrants que reçoit le serveur de la
	/// connexion/du client courant(e) et les traitent.
	pub(crate) async fn intercept_messages(&self, socket: SocketStream) {
		let mut stream = IrcCodec::new(socket.0);

		tokio::spawn(async move {
			while let Some(Ok(line)) = stream.next().await {
				let bytestream = ByteStream::new(line);
				let inputstream = InputStream::new(bytestream.chars());

				let output =
					IrcMessage::parse(inputstream).map(Self::handle_message);

				if let Ok(response) = output.expect("une réponse").await {
					logger::debug!("Output: {:?}", response,);

					let msg = format!(
						":{} {} {} :{}\r\n",
						stream.get_ref().peer_addr().unwrap(),
						response.code(),
						"yournick",
						response,
					);

					stream.send(msg).await.expect("l'envoie de la réponse");
				}
			}
		});
	}

	/// Gère les message entrants et retourne la réponse appropriée.
	//
	// NOTE(phisyx): à ce moment-ci, nous ne savons pas si les commandes sont
	// envoyées par un client (IRC-CLIENT) ou par un service ou un serveur.
	async fn handle_message(
		message: IrcMessage,
	) -> Result<IrcCommandNumeric, IrcMessageError> {
		logger::debug!("Input: {:#?}", &message);

		let msg = IrcCommandNumeric::ERR_UNKNOWNCOMMAND {
			command: match message.command {
				IrcMessageCommand::Numeric { code, .. } => code,
				IrcMessageCommand::Text { text, .. } => text,
			},
		};

		Ok(msg)
	}
}

impl ServerConfig {
	/// Nouvelle configuration serveur IRC.
	fn new(listen: &IrcdListen) -> Self {
		Self {
			uflags: Server::USER_FLAGS,
			name_max_size: Server::NAME_MAX_SIZE,
			port_plaintext: Server::PORT_PLAINTEXT,
			port_encrypt: Server::PORT_ENCRYPT,
			user: listen.to_owned(),
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl From<std::net::AddrParseError> for IrcServerError {
	fn from(err: std::net::AddrParseError) -> Self {
		Self::ParseAddr(err)
	}
}

impl From<io::Error> for IrcServerError {
	fn from(err: io::Error) -> Self {
		Self::IO(err)
	}
}

impl From<ListenerError> for IrcServerError {
	fn from(err: ListenerError) -> Self {
		Self::Listener(err)
	}
}

impl fmt::Display for IrcServerError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"{}",
			match self {
				| Self::AddrIsAlreadyEstablished(addr) => format!(
					"Une connexion vers « {0} » est déjà établie.",
					addr
				),
				| Self::IO(err) => err.to_string(),
				| Self::Listener(err) => err.to_string(),
				| Self::ParseAddr(err) => err.to_string(),
			}
		)
	}
}
