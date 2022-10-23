/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

use phisyrc_config::{load_or_prompt, ServerConfig};

fn main() {
	let maybe_cfg = load_or_prompt::<ServerConfig>(
		constants::CONFIG_SERVER,
		"Voulez-vous créer la configuration serveur?",
	);

	if let Ok(cfg) = maybe_cfg {
		println!("La configuration serveur est: {cfg:#?}");
	}
}
