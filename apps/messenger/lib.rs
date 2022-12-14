/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod cli;
mod commands;
mod env;
mod server;
mod session;

use core::fmt;

use commands::ServerCommand;
use config::ServerConfig;
pub(crate) use network::{
	session::Interface as NetworkSessionInterface, Server as NetworkServer,
	Session as NetworkSession,
};
use session::{SessionID, User};
use tokio::sync::mpsc;

pub use self::{cli::cli_app, env::env_app};
use self::{cli::CommandMakePassword, server::Server as AppServer};

// ---- //
// Type //
// ---- //

pub type AppContextWriter = mpsc::UnboundedSender<AppContext>;
pub type AppContextReader = mpsc::UnboundedReceiver<AppContext>;

pub type Result<T> = core::result::Result<T, Error>;

// --------- //
// Structure //
// --------- //

#[allow(dead_code)]
pub struct App {
	/// Argument de la CLI.
	cli: cli_app,
	/// Les variables d'environnement.
	env: env_app,
	/// Client de la base de données.
	database: database::Client,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
pub enum AppContext {
	/// Utilisé pour quitter l'application.
	Quit,
	/// Entrée utilisateur envoyé depuis le terminal (stdout / TUI)
	InputFromTUI(String),
	/// Message IRC.
	// IRC(irc_msg::Message),

	/// Server communique avec client : commande numérique
	ReplyNumeric {
		id: SessionID,
		prefix: String,
		numeric: Box<irc_replies::Numeric>,
	},

	/// Répondre à toutes les session : command textuelle
	BroadcastCommand {
		command: irc_replies::Command,
	},

	RegisterClient {
		id: SessionID,
		user: User,
	},

	SessionsList,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Error {
	/// Erreur I/O.
	IO(std::io::Error),
	/// Erreur liée à la base de données.
	Database(database::Error),
	/// Erreur liée au réseau (serveur / client).
	Network(network::Error),
	/// Erreur liée au message IRC
	IrcMessage(irc_msg::Error),
	/// Erreur liée au réponse IRC
	IrcReplies(irc_replies::Error),
	/// Génération du hachage de mot de passe invalide.
	BadGenerationPassword,
	/// La variable d'environnement APP_SECRET_KEY n'est pas définie.
	SecretKeyNotFound,

	/// [std::process::ExitCode::SUCCESS]
	EXIT_SUCCESS,
}

// -------------- //
// Implémentation // -> Publique
// -------------- //

impl App {
	/// Initialise la structure de l'application.
	pub fn new(
		args: cli_app,
		env: env_app,
		database: database::Client,
	) -> Self {
		Self {
			cli: args,
			env,
			database,
		}
	}

	/// Gère les commandes de la CLI.
	pub fn handle_command(&self) -> Result<()> {
		if let Some(command) = self.cli.command.as_ref() {
			match command {
				| cli::Command::Config(cfg_cli) if cfg_cli.options.delete => {
					config::delete(constants::CONFIG_SERVER)?;

					return Err(Error::EXIT_SUCCESS);
				}

				| cli::Command::Config(_) => {
					let cfg =
						config::load::<ServerConfig>(constants::CONFIG_SERVER)?;
					println!("{cfg:#?}");
					return Err(Error::EXIT_SUCCESS);
				}

				| cli::Command::MakePassword(make_password) => {
					return self.handle_make_password_command(make_password);
				}
			}
		}

		Ok(())
	}

	/// Lance le serveur de Chat.
	pub async fn launch(
		self,
		(ctx, mut crx): (AppContextWriter, AppContextReader),
	) -> Result<()> {
		let cfg = config::load::<ServerConfig>(constants::CONFIG_SERVER)?;

		let server = NetworkServer::create(
			(cfg.ip.to_owned(), cfg.tcp_port.into()),
			(cfg.ip.to_owned(), cfg.websocket_port.into()),
			|instance: NetworkServer<AppServer>| {
				AppServer::new(ctx, instance, cfg)
			},
		)
		.await?;

		let receiver_context_task = tokio::spawn(async move {
			loop {
				tokio::select! {
					Some(app_ctx) = crx.recv() => match app_ctx {
						| AppContext::Quit => break,

						| AppContext::InputFromTUI(input) => {
							match ServerCommand::parse(&input) {
								| Ok(command) => {
									command.handle(&server);
								}
								| Err(err) => {
									logger::error!("Commande serveur invalide: {err}");
								}
							}
						}

						| _ => continue,
					}
				}
			}
		});

		loop {
			if receiver_context_task.is_finished() {
				break;
			}

			logger::info!(""); // <- HACK(phisyx): permet de ne pas bloquer tui
			tokio::time::sleep(tokio::time::Duration::from_millis(64)).await;
		}

		Ok(())
	}
}

impl AppContext {
	pub fn mpsc() -> (AppContextWriter, AppContextReader) {
		mpsc::unbounded_channel()
	}
}

// -------------- //
// Implémentation // -> Privée
// -------------- //

impl App {
	/// `CLI`: Génère un mot de passe haché avec l'algorithme de hachage choisi
	/// par le choix utilisateur.
	fn handle_make_password_command(
		&self,
		make_password: &CommandMakePassword,
	) -> Result<()> {
		let raw_password = &make_password.flags.password;
		let algo = &make_password.options.algo;

		let password = match algo {
			| cli::PasswordAlgorithm::Argon2 => {
				let config = argon2::Config {
					variant: argon2::Variant::Argon2id,
					thread_mode: argon2::ThreadMode::Parallel,
					..Default::default()
				};

				if make_password.options.secret_key.is_none() {
					return Err(Error::SecretKeyNotFound);
				}

				let app_secret_key =
					make_password.options.secret_key.as_ref().unwrap();

				argon2::hash_encoded(
					raw_password.as_bytes(),
					app_secret_key.as_bytes(),
					&config,
				)
			}
		}?;

		println!(
			"Le mot de passe « {} » généré par « {} »: {}",
			raw_password, algo, password
		);

		Ok(())
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl terminal::EventLoop for AppContext {
	fn input(msg: String) -> Self {
		Self::InputFromTUI(msg)
	}

	fn quit() -> Self {
		Self::Quit
	}
}

// -------------- //
// Implémentation // -> From<T>
// -------------- //

impl From<argon2::Error> for Error {
	fn from(_: argon2::Error) -> Self {
		Self::BadGenerationPassword
	}
}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Self::IO(err)
	}
}

impl From<database::Error> for Error {
	fn from(err: database::Error) -> Self {
		Self::Database(err)
	}
}

impl From<network::Error> for Error {
	fn from(err: network::Error) -> Self {
		Self::Network(err)
	}
}

impl From<irc_msg::Error> for Error {
	fn from(err: irc_msg::Error) -> Self {
		Self::IrcMessage(err)
	}
}

impl From<irc_replies::Error> for Error {
	fn from(err: irc_replies::Error) -> Self {
		Self::IrcReplies(err)
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let err_s = match self {
			| Self::BadGenerationPassword => {
				"impossible de générer le mot de passe.".to_owned()
			}
			| Self::SecretKeyNotFound => {
				"la variable d'environnement APP_SECRET_KEY n'existe pas."
					.to_owned()
			}
			| Self::IO(err) => format!("IO: {err}"),
			| Self::Database(err) => format!("Base de données: {err}"),
			| Self::Network(err) => format!("Réseau: {err}"),
			| Self::IrcMessage(err) => format!("message irc: {err}"),
			| Self::IrcReplies(err) => format!("réponse irc: {err}"),
			| Self::EXIT_SUCCESS => "exit success".to_owned(),
		};

		write!(f, "{err_s}")
	}
}
