/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod cli;
mod env;
mod server;
mod session;

use core::fmt;

use config::ServerConfig;
pub(crate) use network::{
	session::Interface as NetworkSessionInterface, Server as NetworkServer,
	Session as NetworkSession,
};
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

pub struct App {
	args: cli_app,
	env: env_app,
	database: database::Client,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
pub enum AppContext {
	Quit,
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Error {
	IO(std::io::Error),
	Boxed(Box<dyn std::error::Error>),
	Database(database::Error),
	Network(network::Error),
	BadGenerationPassword,
	SecretKeyNotFound,
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
			args,
			env,
			database,
		}
	}

	/// Gère les commandes de la CLI.
	pub fn handle_command(&self) -> Result<()> {
		match self.args.command.as_ref() {
			| Some(cmd) => match cmd {
				| cli::Command::MakePassword(make_password) => {
					self.handle_make_password_command(make_password)
				}
				| cli::Command::Config(config_cli) => {
					if config_cli.options.delete {
						config::delete(constants::CONFIG_SERVER)?;
					} else if config_cli.options.show {
						let cfg = config::load::<ServerConfig>(
							constants::CONFIG_SERVER,
						)?;
						println!("{cfg:#?}");
					}

					Err(Error::EXIT_SUCCESS)
				}
			},
			| None => Ok(()),
		}
	}

	/// Lance le serveur de Chat.
	pub async fn launch(
		self,
		(ctx, mut crx): (AppContextWriter, AppContextReader),
	) -> Result<()> {
		let receiver_context_task = tokio::spawn(async move {
			loop {
				tokio::select! {
					Some(app_ctx) = crx.recv() => match app_ctx {
						| AppContext::Quit => break,
					}
				}
			}
		});

		let cfg = config::load_or_prompt::<ServerConfig>(
			constants::CONFIG_SERVER,
			"Voulez-vous créer la configuration serveur?",
		)?;

		let server_addr = (cfg.ip, cfg.port.into());

		NetworkServer::create(
			server_addr,
			|instance: NetworkServer<AppServer>| AppServer::new(ctx, instance),
		)
		.await?;

		loop {
			if receiver_context_task.is_finished() {
				break;
			}

			logger::info!(""); // <- HACK(phisyx): permet de ne pas bloquer tui
		}

		Ok(())
	}
}

// -------------- //
// Implémentation // -> Privée
// -------------- //

impl App {
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
			"Le mot de passe '{}' généré par {}: {}",
			raw_password, algo, password
		);

		Ok(())
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl terminal::EventLoop for AppContext {
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
impl From<Box<dyn std::error::Error>> for Error {
	fn from(err: Box<dyn std::error::Error>) -> Self {
		Self::Boxed(err)
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
			| Self::IO(err) => {
				format!("IO: {err}")
			}
			| Self::Boxed(err) => err.to_string(),
			| Self::Database(err) => {
				format!("Base de données: {err}")
			}
			| Self::Network(err) => {
				format!("Réseau: {err}")
			}
			| Self::EXIT_SUCCESS => "exit success".to_owned(),
		};

		write!(f, "{err_s}")
	}
}
