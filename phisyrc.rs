/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![doc = include_str!("README.md")]

mod app {
	mod r#mod;
	mod output;
	mod ui;

	pub use self::{output::*, r#mod::*, ui::*};
}

use app::{AppResult, TypeGui};
use cli::app::{
	phisyrc_cli, Command, CommandClient, CommandMakePassword, CommandServer,
	SubCommandServer,
};
use env::phisyrc_env;
use irc::{IrcDaemon, IRC};

use self::app::{App, UI};

#[phisyrc::setup(logger)]
async fn main(cli_args: phisyrc_cli, env_args: phisyrc_env) -> AppResult<()> {
	let output = match cli_args.command {
		| Some(Command::Client(client)) => {
			handle_client_command(client.into(), env_args.gui_to_use).await
		}

		| Some(Command::Server(server)) => handle_server_command(server).await,

		| Some(Command::MakePassword(password)) => {
			handle_make_password_command(password, env_args.app_secret_key)
		}

		| None => {
			handle_client_command(Default::default(), env_args.gui_to_use).await
		}
	};

	let exit_code = match output {
		| Ok(()) => 0,
		| Err(err) => {
			eprintln!("{}", err);
			1
		}
	};

	std::process::exit(exit_code);
}

async fn handle_client_command(
	client: Option<CommandClient>,
	type_gui: String,
) -> AppResult<()> {
	let type_gui = type_gui.parse::<TypeGui>()?;

	if client.is_none() {
		return Ok(App::launch(UI::Graphical(type_gui)).await?);
	}

	let client = client.unwrap();

	let ui = if client.options.tui {
		UI::Textual
	} else {
		UI::Graphical(type_gui)
	};

	Ok(App::launch(ui).await?)
}

async fn handle_server_command(server: CommandServer) -> AppResult<()> {
	match server.command {
		| Some(cmd) => match cmd {
			| SubCommandServer::Restart { id } => {
				todo!("restart: {id:?}");
			}
			| SubCommandServer::Rehash { id } => {
				todo!("rehash {id:?}");
			}
		},
		| None => {
			if server.options.daemon {
				IrcDaemon::spawn(server.options.config).await?;
			} else {
				IRC::run(server.options.config).await?;
			}
		}
	};

	Ok(())
}

fn handle_make_password_command(
	password: CommandMakePassword,
	app_secret_key: String,
) -> AppResult<()> {
	if let Ok(passwd) = argon2::hash_encoded(
		password.flags.password.as_bytes(),
		app_secret_key.as_bytes(),
		&argon2::Config::default(),
	) {
		println!("Le mot de passe Argon2 généré: {passwd}");
	}

	Ok(())
}
