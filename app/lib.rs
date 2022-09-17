/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod config;
mod export;
mod output;
mod ui;

use cli::app::{
	Command, CommandClient, CommandMakePassword, CommandServer,
	PasswordAlgorithm, SubCommandServer,
};
use gui::GUI;
use irc::{Irc, IrcDaemon};
use tui::TUI;
use web::WEB;

pub use self::export::*;
use crate::{config::*, ui::*};

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
	) -> Result<Self> {
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
	pub async fn handle_cli_command(&self) -> Result<()> {
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
	) -> Result<()> {
		if maybe_client_cli.is_none() {
			return self.launch(UI::Graphical).await;
		}

		let client_cli = maybe_client_cli.unwrap();

		let ui = if client_cli.options.tui {
			UI::Textual
		} else if client_cli.options.web {
			UI::Web
		} else {
			UI::Graphical
		};

		self.launch(ui).await
	}

	/// Commande `server`.
	async fn handle_server_command(
		&self,
		server_cli: &CommandServer,
	) -> Result<()> {
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
	) -> Result<()> {
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
	/// Lance l'application en mode graphique ou textuel.
	pub async fn launch(&self, ui: UI) -> Result<()> {
		let client_cfg =
			fs::TOMLFileLoader::<ClientConfig>::load_with_next_key(
				&self.global_config.config_client,
			)?;

		match ui {
			| UI::Graphical => Ok(GUI::launch()?),
			| UI::Textual => Ok(TUI::launch().await?),
			| UI::Web => Ok(WEB::launch(client_cfg.config_web).await?),
		}
	}
}
