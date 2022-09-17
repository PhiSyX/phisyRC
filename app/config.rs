/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::path::PathBuf;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(serde::Deserialize)]
pub struct GlobalConfig {
	pub config_client: PathBuf,
	pub config_server: PathBuf,
}

#[derive(Debug)]
#[derive(serde::Deserialize)]
pub struct ClientConfig {
	pub config_ui: PathBuf,
	pub config_web: PathBuf,
}

#[derive(Debug)]
#[derive(serde::Deserialize)]
pub struct ServerConfig {
	pub config_irc: PathBuf,
}
