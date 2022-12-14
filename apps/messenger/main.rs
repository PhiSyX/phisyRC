/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use app::App;

#[phisyrc::setup(config = "interactive", logger = "tui", database = "postgres")]
async fn main<Async, Context>(
	args: app::cli_app,
	env: app::env_app,
) -> app::Result<()>
where
	Async: tokio,

	Context: app::AppContext,
	[ctx]: app::AppContextWriter,
	[crx]: app::AppContextReader,
{
	let app = App::new(args, env, maybe_database?);

	if let Err(err) = app.handle_command() {
		match err {
			| app::Error::EXIT_SUCCESS => std::process::exit(0),
			| _ => panic!("phisyRC: {err}"),
		}
	}

	app.launch((ctx, crx)).await
}
