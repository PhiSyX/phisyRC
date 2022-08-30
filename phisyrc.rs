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

use app::AppResult;
use cli::app::phisyrc_cli;
use env::phisyrc_env;

use self::app::App;

#[phisyrc::setup(logger)]
async fn main(cli_args: phisyrc_cli, env_args: phisyrc_env) -> AppResult<()> {
	let app = App::new(cli_args, env_args);

	if let Err(err) = app.handle_command().await {
		panic!("{err}");
	}

	Ok(())
}
