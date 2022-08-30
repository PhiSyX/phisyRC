/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::fmt;
use std::{collections::HashMap, net::SocketAddr, sync::Arc, time::SystemTime};

use chrono::{DateTime, Utc};
use futures::{SinkExt, StreamExt};
use lang::stream::prelude::*;
use tokio::{
	io::{self},
	net::TcpStream,
};
use tokio_util::codec::LinesCodecError;

use super::entity::{AtomicEntity, Entity};
use crate::{
	arch::{
		network::entity::EntityType, AtomicNetwork, Listener, ListenerError,
		Socket, SocketStream,
	},
	commands::{
		IncomingUnregisteredCommand, IrcClientCommand, IrcCommandNumeric,
		IrcReplies,
	},
	config::IrcdListen,
	forever,
	message::{IrcCodec, IrcMessage, IrcMessageCommand},
};

// ---- //
// Type //
// ---- //

pub type AtomicServerConfig = Arc<ServerConfig>;
pub type AtomicServer = Arc<Server>;

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
	pub network: AtomicNetwork,

	/// Configuration du serveur.
	pub config: AtomicServerConfig,

	/// Les clients/servers connectés au serveur.
	pub connections: HashMap<String, AtomicEntity>,

	/// Le nom du serveur.
	pub label: String,

	/// La socket du serveur.
	pub socket: Socket,

	pub created_at: DateTime<Utc>,
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
		network: AtomicNetwork,
		listen: &IrcdListen,
	) -> Result<Self, IrcServerError> {
		let config = ServerConfig::new(listen);
		let socket = Socket::new(&config.user.ip, config.user.port)?;
		let label = format!("{}:{}", config.user.ip, config.user.port);
		Ok(Self {
			network,
			config: Arc::new(config),
			connections: Default::default(),
			label,
			socket,
			created_at: DateTime::from(SystemTime::now()),
		})
	}

	/// Vérifie si une connexion vers une adresse est déjà établie.
	pub(crate) async fn ping_host(&self) -> Result<(), IrcServerError> {
		let addr = self.socket.addr;

		logger::trace!(
			"Vérifie que le serveur puisse démarrer à l'adresse {}.",
			addr
		);

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
	) -> Result<Listener, IrcServerError> {
		Ok(self.socket.listen().await?)
	}

	pub(crate) async fn accept(
		&self,
		listener: &Listener,
	) -> Result<SocketStream, IrcServerError> {
		Ok(self.socket.accept(listener).await?)
	}

	pub(crate) async fn can_locate_client(&self, label: &str) -> bool {
		for conn in self.connections.values() {
			let maybe = conn.lock().await.label() == label;
			if maybe {
				return true;
			}
		}
		false
	}

	pub(crate) fn new_entity(&mut self, socket: &SocketStream) -> AtomicEntity {
		let peer_addr = socket.peer_addr();
		let entity = Entity::new(self.shared(), peer_addr);
		let peer_addr = peer_addr.to_string();
		let atomic_entity = entity.shared();
		self.connections.insert(peer_addr, atomic_entity.clone());
		atomic_entity
	}

	/// Intercepte les messages entrants que reçoit le serveur de la
	/// connexion/du client courant(e) et les traitent.
	pub(crate) async fn intercept_messages(
		&self,
		client: AtomicEntity,
		mut irc: IrcCodec<TcpStream>,
	) {
		let server_config = self.config.clone();

		forever! {{
			let shared_client1 = client.clone();

			let maybe_line = irc.next().await;
			if maybe_line.is_none() {
				continue;
			}


			// SAFETY(unwrap): la condition-ci-haut nous permet d'utiliser
			// unwrap avec sûreté.
			let bytes = match maybe_line.unwrap().as_ref() {
				| Err(LinesCodecError::MaxLineLengthExceeded) => {
					let prefix = shared_client1.lock().await.prefix();
					let out = format!("ERROR :Closing Link: [{prefix}] (Max SendQ Exceeded)\r\n");
					irc.send(out).await.unwrap();
					break;
				}

				| Err(err) => {
					logger::error!("codec Error: {err}");
					break;
				}

				| Ok(line) => {
					logger::trace!(
						"Le client « {} » a envoyé le message « {} ».",
						client.lock().await.addr,
						line
					);
					ByteStream::new(line)
				},
			};
			let input = InputStream::new(bytes.chars());

			// Output
			let output = IrcMessage::parse(input)
				.map(move |msg| {
					logger::debug!("Nouveau message entrant analysé:\n\t{:?}", &msg);
					Self::handle_message(shared_client1, msg)
				});

			// Response output
			let shared_client2 = client.clone();
			let replies = match output {
				| Ok(response) => match response.await {
					| Ok(replies) => replies,
					| Err(err) => vec![err.into()],
				},

				// NOTE(phisyx): nous ne voulons pas casser la boucle en cas de
				// caractères invalides.
				| Err(err) => {
					logger::error!("Message: {err}");
					continue
				},
			};

			logger::debug!(
				"Les réponses du message sont les suivantes:\n\t{:?}",
				&replies
			);

			for reply in replies {
				match reply {
					| IrcReplies::Ignore => {}

					| IrcReplies::Error(reason) => {
						let prefix =  shared_client2.lock().await.prefix();
						let out = format!("ERROR :Closing Link: [{prefix}] ({reason})\r\n");
						irc.send(out).await.unwrap();
						return;
					}

					| IrcReplies::Quit(maybe_reason) => {
						let prefix = shared_client2.lock().await.prefix();

						let out = if let Some(reason) = maybe_reason {
							format!(":{prefix} QUIT :{reason}\r\n")
						} else {
							format!(":{prefix} QUIT\r\n")
						};

						irc.send(out).await.unwrap();
						return;
					}

					| IrcReplies::Numeric(reply) => {
						let prefix = shared_client2.lock().await
							.prefix_based_on_reply(&reply);

						let msg = format!(
							":{} {} {} {}\r\n",
							server_config.user.name,
							reply.code(),
							prefix,
							reply,
						);

						irc.send(msg).await.unwrap();
					}
				}
			}
		}}
	}

	/// Gère les message entrants et retourne la réponse appropriée.
	//
	// NOTE(phisyx): à ce moment-ci, nous ne savons pas si les commandes sont
	// envoyées par un client (IRC-CLIENT) ou par un service ou un serveur
	// (IRC-SERVER).
	async fn handle_message(
		shared_entity: AtomicEntity,
		message: IrcMessage,
	) -> Result<Vec<IrcReplies>, IrcCommandNumeric> {
		let mut entity = shared_entity.lock().await;

		if !entity.is_registered() {
			// NOTE(phisyx): vérifie que la commande entrante est valide.
			let command =
				match IncomingUnregisteredCommand::is_valid(&message.command) {
					| Ok(c) => c,
					| Err(err) => return Ok(vec![err]),
				};

			match command {
				| IncomingUnregisteredCommand::PASS { .. } => {
					entity.handle_pass_command(&command)?
				}

				| IncomingUnregisteredCommand::NICK { .. } => {
					entity.handle_nick_command(&command)?
				}

				| IncomingUnregisteredCommand::USER { .. } => {
					entity.handle_user_command(&command)?
				}

				| IncomingUnregisteredCommand::SERVER { .. } => {
					entity.handle_server_command(&command)?
				}

				| IncomingUnregisteredCommand::QUIT { parameters } => {
					return Ok(vec![IrcReplies::Quit(
						parameters.first().cloned(),
					)])
				}
			};

			let server_cfg = entity.server.config.clone();

			return match entity.ty {
				| Some(EntityType::Client(ref mut client)) => {
					let command =
						match IrcClientCommand::is_valid(&message.command) {
							| Ok(c) => c,
							| Err(err) => return Ok(vec![err]),
						};

					match command {
						| IrcClientCommand::PASS { .. } => {
							client
								.handle_upass_command(server_cfg, &command)
								.await?;
							Ok(vec![])
						}

						| IrcClientCommand::NICK { .. } => {
							client.handle_unick_command(&command).await
						}

						| IrcClientCommand::USER { .. } => {
							client.handle_uuser_command(&command).await
						}
					}
				}

				// TODO(phisyx): serveur
				| Some(EntityType::Server(ref mut _server)) => {
					let msg = IrcCommandNumeric::ERR_UNKNOWNCOMMAND {
						command: match message.command {
							| IrcMessageCommand::Numeric { code, .. } => code,
							| IrcMessageCommand::Text { command, .. } => {
								command
							}
						},
					};

					Ok(vec![IrcReplies::Numeric(msg)])
				}

				| None => Ok(vec![]),
			};
		}

		match entity.ty {
			| Some(EntityType::Client(ref mut client)) => {
				let command = match IrcClientCommand::is_valid(&message.command)
				{
					| Ok(c) => c,
					| Err(err) => return Ok(vec![err]),
				};

				match command {
					| IrcClientCommand::PASS { .. }
					| IrcClientCommand::USER { .. } => {
						Err(IrcCommandNumeric::ERR_ALREADYREGISTRED)
					}

					| IrcClientCommand::NICK { .. } => client
						.handle_nick_command(&command)
						.await
						.map(|_| vec![]),
				}
			}

			| Some(EntityType::Server(ref mut _server)) => {
				let msg = IrcCommandNumeric::ERR_UNKNOWNCOMMAND {
					command: match message.command {
						| IrcMessageCommand::Numeric { code, .. } => code,
						| IrcMessageCommand::Text { command, .. } => command,
					},
				};

				Err(msg)
			}

			| None => Ok(vec![]),
		}
	}

	pub(crate) fn shared(&self) -> AtomicServer {
		Arc::new(self.clone())
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
