/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![doc = include_str!("../README.md")]

use app::{phisyrc_cli, phisyrc_env, App};

#[phisyrc::setup(logger)]
	let app = App::new(cli_args, env_args)?;
async fn main(cli_args: phisyrc_cli, env_args: phisyrc_env) -> app::Result<()> {

	if let Err(err) = app.handle_cli_command().await {
		panic!("{err}");
	}

	Ok(())
}
