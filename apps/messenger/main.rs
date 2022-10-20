/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use app::App;

#[phisyrc::setup(logger = "tui", database = "postgres")]
async fn main<Async>(args: app::cli_app, env: app::env_app) -> app::Result<()>
where
	Async: tokio,
{
	let app = App::new(args, env, maybe_database?);

	if let Err(err) = app.handle_command() {
		match err {
			| app::Error::EXIT_SUCCESS => std::process::exit(0),
			| _ => panic!("phisyRC: {err}"),
		}
	}

	app.launch(crx).await
}
