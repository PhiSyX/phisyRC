/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use app::App;

#[phisyrc::setup(logger)]
async fn main(args: app::cli_app, env: app::env_app) -> app::Result<()> {
	let app = App::new(args, env);
	if let Err(err) = app.handle_command() {
		panic!("phisyRC: {err}")
	}
	app.launch().await
}
