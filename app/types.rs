/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use shared::err;

// ------------------------ //
// Erreurs de l'application //
// ------------------------ //

err! {
	/// STD IO
	| IO(std::io::Error) => "erreur IO: {}"
	| Database(database::Error) => "erreur de la base de donnée: {}"
	| Env(env::Error) => "erreur liée aux variables d'environnement: {}"
	| GUI(gui::Error) => "erreur de l'application graphique: {}"
	| IRC(irc::Error) => "erreur IRC: {}"
}
