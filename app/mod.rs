/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::io;

use super::{GUI, TUI, UI};

// --------- //
// Structure //
// --------- //

pub struct App;

// -------------- //
// Implementation //
// -------------- //

impl App {
	/// Lance l'application en mode [graphique](Ui::Graphical) ou
	/// [textuel](Ui::Textual).
	pub async fn launch(ui: UI) -> io::Result<()> {
		match ui {
			| UI::Graphical(gui) => GUI::launch(gui).await,
			| UI::Textual => TUI::launch().await,
		}
	}
}
