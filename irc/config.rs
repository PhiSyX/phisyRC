/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod ircd;

use std::path::Path;

use logger::Stylize;

pub(crate) use self::ircd::*;
use crate::output::IrcResult;

/// Récupère le fichier de configuration de l'IRCd, et dé-sérialise le contenu
/// sous la forme de la structure [IrcdConfig].
pub fn load(filename: impl AsRef<Path>) -> IrcResult<IrcdConfig> {
	let content = std::fs::read(&filename)?;
	let data = toml::from_slice(&content)?;
	let f = filename.as_ref().display();

	logger::info!(
		"Fichier de configuration « {0} » chargé avec succès.",
		f.to_string().magenta()
	);
	logger::debug!(
		"Sortie de dé-sérialisation du fichier de configuration \n {0:#?}",
		&data
	);

	Ok(data)
}
