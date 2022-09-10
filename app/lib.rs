/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod config;
mod output;
mod ui;

use std::io;

pub use cli::app::phisyrc_cli;
use cli::app::{
	Command, CommandClient, CommandMakePassword, CommandServer,
	PasswordAlgorithm, SubCommandServer,
};
pub use env::phisyrc_env;
use gui::{TypeGui, GUI};
use irc::{Irc, IrcDaemon};
use tui::TUI;
use web::WEB;

pub use self::{config::*, output::*, ui::*};

// --------- //
// Structure //
// --------- //

pub struct App {
	pub cli: phisyrc_cli,
	pub env: phisyrc_env,
	pub global_config: GlobalConfig,
}

// -------------- //
// Implementation //
// -------------- //

impl App {
	pub fn new(
		cli_args: phisyrc_cli,
		env_args: phisyrc_env,
	) -> AppResult<Self> {
		let global_config =
			fs::TOMLFileLoader::<GlobalConfig>::load_with_next_key(
				&cli_args.options.config,
			)?;

		Ok(Self {
			cli: cli_args,
			env: env_args,
			global_config,
		})
	}

	/// Gère les commandes de la CLI.
	pub async fn handle_cli_command(&self) -> AppResult<()> {
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
			return Ok(self.launch(UI::Graphical(type_gui)).await?);
		}

		let client_cli = maybe_client_cli.unwrap();

		let ui = if client_cli.options.tui {
			UI::Textual
		} else if client_cli.options.web {
			UI::Web
		} else {
			UI::Graphical(type_gui)
		};

		Ok(self.launch(ui).await?)
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
				let server_cfg: ServerConfig =
					fs::TOMLFileLoader::load_with_next_key(
						server_cli
							.options
							.config
							.as_ref()
							.unwrap_or(&self.global_config.config_server),
					)?;

				if server_cli.options.daemon {
					IrcDaemon::spawn(server_cfg.config_irc).await?;
				} else {
					Irc::run(server_cfg.config_irc).await?;
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
	pub async fn launch(&self, ui: UI) -> io::Result<()> {
		let client_cfg =
			fs::TOMLFileLoader::<ClientConfig>::load_with_next_key(
				&self.global_config.config_client,
			)?;

		match ui {
			| UI::Graphical(gui) => GUI::launch(gui),
			| UI::Textual => TUI::launch().await,
			| UI::Web => WEB::launch(client_cfg.config_web).await,
		}
	}
}
