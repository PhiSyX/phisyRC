/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use phisyrc_config::{load_or_prompt, DatabaseConfig};

fn main() {
	let maybe_cfg = load_or_prompt::<DatabaseConfig>(
		constants::CONFIG_DATABASE,
		"Voulez-vous créer la configuration de la base de données?",
	);

	if let Ok(cfg) = maybe_cfg {
		println!("La configuration de la base de données est: {cfg:#?}");
	}
}
