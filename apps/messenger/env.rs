/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// --------- //
// Structure //
// --------- //

/// Les variables d'environnement de l'application.
#[derive(Debug)]
#[derive(phisyrc::Env)]
#[allow(non_camel_case_types)]
pub struct env_app {
	#[var(key = "DEBUG", default = "*")]
	pub debug_filter: String,

	#[var(key = "APP_SECRET_KEY")]
	pub app_secret_key: String,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl setup::SetupEnvInterface for env_app {
	fn debug_filter(&self) -> String {
		self.debug_filter.clone()
	}
}
