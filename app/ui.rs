/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use gui::TypeGui;

// ----------- //
// Énumération //
// ----------- //

/// Interface utilisateur
pub enum UI {
	/// Interface utilisateur graphique
	Graphical(TypeGui),

	/// Interface utilisateur textuel
	Textual,

	/// Interface utilisateur pour le web
	Web,
}
