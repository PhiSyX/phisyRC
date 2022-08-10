/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod app {
	mod r#mod;
	mod ui;

	pub use self::{r#mod::*, ui::*};
}

use cli::{Command, CommandClient, CommandServer, SubCommandServer};
use daemon::IrcDaemon;
use irc::IRC;

use self::app::{App, UI};

#[phisyrc::setup]
async fn main(args: cli::phisyrc) {
	match args.command {
		| Some(Command::Client(client)) => handle_client_command(client).await,
		| Some(Command::Server(server)) => handle_server_command(server).await,
		| None => App::launch(UI::Graphical)
			.await
			.expect("l'interface graphique"),
	}
}

async fn handle_client_command(client: CommandClient) {
	let ui = if client.options.tui {
		UI::Textual
	} else {
		UI::Graphical
	};

	App::launch(ui).await.expect("l'interface graphique");
}

async fn handle_server_command(server: CommandServer) {
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
	}
}
