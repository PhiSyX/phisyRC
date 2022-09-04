/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::io;

use cli::app::{
	phisyrc_cli, Command, CommandClient, CommandMakePassword, CommandServer,
	PasswordAlgorithm, SubCommandServer,
};
use env::phisyrc_env;
use irc::{Irc, IrcDaemon};

use super::{AppResult, TypeGui, GUI, TUI, UI};

// --------- //
// Structure //
// --------- //

pub struct App {
	pub cli: phisyrc_cli,
	pub env: phisyrc_env,
}

// -------------- //
// Implementation //
// -------------- //

impl App {
	pub fn new(cli_args: phisyrc_cli, env_args: phisyrc_env) -> Self {
		Self {
			cli: cli_args,
			env: env_args,
		}
	}

	/// Gère les commandes de la CLI.
	pub(crate) async fn handle_cli_command(&self) -> AppResult<()> {
		match self.cli.command.as_ref() {
			| Some(Command::Client(client)) => {
				self.handle_client_command(Some(client)).await
			}

			| Some(Command::Server(server)) => {
				self.handle_server_command(server).await
			}

			| Some(Command::MakePassword(password_cli)) => {
				self.handle_make_password_command(password_cli)
			}

			// Par défaut.
			| None => self.handle_client_command(Default::default()).await,
		}
	}

	/// Commande `client`.
	async fn handle_client_command(
		&self,
		maybe_client_cli: Option<&CommandClient>,
	) -> AppResult<()> {
		let type_gui = self.env.gui_to_use.parse::<TypeGui>()?;

		if maybe_client_cli.is_none() {
			return Ok(Self::launch(UI::Graphical(type_gui)).await?);
		}

		let client_cli = maybe_client_cli.as_ref().unwrap();

		let ui = if client_cli.options.tui {
			UI::Textual
		} else {
			UI::Graphical(type_gui)
		};

		Ok(App::launch(ui).await?)
	}

	/// Commande `server`.
	async fn handle_server_command(
		&self,
		server_cli: &CommandServer,
	) -> AppResult<()> {
		match server_cli.command.as_ref() {
			| Some(cmd) => match cmd {
				| SubCommandServer::Restart { .. } => {}
				| SubCommandServer::Rehash { .. } => {}
			},

			| None => {
				if server_cli.options.daemon {
					IrcDaemon::spawn(&server_cli.options.config).await?;
				} else {
					Irc::run(&server_cli.options.config).await?;
				}
			}
		};

		Ok(())
	}

	/// Commande `make:password`
	fn handle_make_password_command(
		&self,
		password_cli: &CommandMakePassword,
	) -> AppResult<()> {
		let raw_password = &password_cli.flags.password;
		let algo = &password_cli.options.algo;
		let generated_password = match algo {
			| PasswordAlgorithm::Argon2 => {
				let config = argon2::Config {
					variant: argon2::Variant::Argon2id,
					thread_mode: argon2::ThreadMode::Parallel,
					..argon2::Config::default()
				};

				unsafe {
					argon2::hash_encoded(
						raw_password.as_bytes(),
						self.env.app_secret_key.as_bytes(),
						&config,
					)
					.unwrap_unchecked()
				}
			}
		};

		println!(
			"Le mot de passe '{}' généré par {}: {}",
			raw_password, algo, generated_password,
		);

		Ok(())
	}
}

impl App {
	/// Lance l'application en mode [graphique](Ui::Graphical) ou
	/// [textuel](Ui::Textual).
	pub async fn launch(ui: UI) -> io::Result<()> {
		match ui {
			| UI::Graphical(gui) => GUI::launch(gui),
			| UI::Textual => TUI::launch().await,
		}
	}
}
