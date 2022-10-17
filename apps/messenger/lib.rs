/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod cli;
mod env;

use core::fmt;

use config::ServerConfig;

use self::cli::CommandMakePassword;
pub use self::{cli::cli_app, env::env_app};

// ---- //
// Type //
// ---- //

pub type Result<T> = core::result::Result<T, Error>;

// --------- //
// Structure //
// --------- //

pub struct App {
	args: cli_app,
	env: env_app,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
pub enum Error {
	IO(std::io::Error),
	BadGenerationPassword,
	SecretKeyNotFound,
}

// -------------- //
// Implémentation // -> Publique
// -------------- //

impl App {
	/// Initialise la structure de l'application.
	pub fn new(args: cli_app, env: env_app) -> Self {
		Self { args, env }
	}

	/// Gère les commandes de la CLI.
	pub fn handle_command(&self) -> Result<()> {
		match self.args.command.as_ref() {
			| Some(cmd) => match cmd {
				| cli::Command::MakePassword(make_password) => {
					self.handle_make_password_command(make_password)
				}
			},
			| None => Ok(()),
		}
	}

	/// Lance le serveur de Chat.
	pub async fn launch(&self) -> Result<()> {
		let cfg = config::prompt_or_load::<ServerConfig>("server.toml")?;

		loop {
			tokio::time::sleep(tokio::time::Duration::from_secs(1024)).await;
		}
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
			| Self::IO(e) => {
				format!("IO: {e}")
			}
		};

		write!(f, "{err_s}")
	}
}
