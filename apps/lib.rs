/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod cli;

use core::fmt;

pub use self::cli::cli_app;
use self::cli::CommandMakePassword;

// ---- //
// Type //
// ---- //

type Result<T> = core::result::Result<T, Error>;

// --------- //
// Structure //
// --------- //

pub struct App {
	args: cli_app,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
pub enum Error {
	BadGenerationPassword,
	SecretKeyNotFound,
}

// -------------- //
// Implémentation // -> Publique
// -------------- //

impl App {
	pub fn new(args: cli_app) -> Self {
		Self { args }
	}

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

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let err_s = match self {
			| Self::BadGenerationPassword => {
				"impossible de générer le mot de passe."
			}
			| Self::SecretKeyNotFound => {
				"la variable d'environnement APP_SECRET_KEY n'existe pas."
			}
		};

		write!(f, "{err_s}")
	}
}
