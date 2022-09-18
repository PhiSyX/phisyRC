/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use core::result::Result as ResultResult;

use shared::err;

// ---- //
// Type //
// ---- //

pub type Result<T> = ResultResult<T, Error>;

// ------------------------ //
// Erreurs de l'application //
// ------------------------ //

err! {
	| IO(std::io::Error) => "IO: {}"
	| Database(database::Error) => "de la base de donnée: {}"
	| Env(env::Error) => "des variables d'environnement: {}"
	| GUI(gui::Error) => "de l'application graphique: {}"
	| IRC(irc::Error) => "IRC: {}"
}
