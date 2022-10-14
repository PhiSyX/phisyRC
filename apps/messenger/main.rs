/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use app::App;

#[phisyrc::setup]
async fn main(args: app::cli_app, env: app::env_app) {
	let app = App::new(args, env);
	if let Err(err) = app.handle_command() {
		panic!("phisyRC: {err}")
	}
	// app.run()
}
