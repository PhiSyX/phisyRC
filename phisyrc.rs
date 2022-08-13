/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod app {
	mod r#mod;
	mod output;
	mod ui;

	pub use self::{output::*, r#mod::*, ui::*};
}

use app::{AppResult, TypeGui};
use cli::app::{
	phisyrc_cli, Command, CommandClient, CommandServer, SubCommandServer,
};
use env::phisyrc_env;
use irc::{IrcDaemon, IRC};

use self::app::{App, UI};

#[phisyrc::setup(logger)]
async fn main(cli_args: phisyrc_cli, env_args: phisyrc_env) -> AppResult<()> {
	match cli_args.command {
		| Some(Command::Client(client)) => {
			let type_gui = env_args.gui_to_use.parse()?;
			handle_client_command(client.into(), type_gui).await
		}

		| Some(Command::Server(server)) => handle_server_command(server).await,

		| None => {
			let type_gui = env_args.gui_to_use.parse()?;
			handle_client_command(Default::default(), type_gui).await
		}
	}
	.expect("CLI");

	Ok(())
}

async fn handle_client_command(
	client: Option<CommandClient>,
	type_gui: TypeGui,
) -> AppResult<()> {
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
				IrcDaemon::spawn();
			} else {
				IRC::run();
			}
		}
	};

	Ok(())
}
