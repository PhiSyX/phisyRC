/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use shared::serde::Port;

// ---- //
// Type //
// ---- //

pub type ClientWebServerPortConfig = Port;

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(serde::Deserialize)]
pub struct ClientWebConfig {
	pub public_dir: String,
	pub public_url: String,
	pub server: ClientWebServerConfig,
}

#[derive(Debug)]
#[derive(serde::Deserialize)]
pub struct ClientWebServerConfig {
	pub ip: String,
	#[serde(deserialize_with = "ClientWebServerPortConfig::validate")]
	pub port: ClientWebServerPortConfig,
	pub proxy: Option<String>,
}
