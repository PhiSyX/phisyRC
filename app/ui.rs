/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod graphical;
mod textual;

pub use self::{graphical::GUI, textual::TUI};

// --------- //
// Interface //
// --------- //

#[async_trait::async_trait]
pub trait UInterface {
	async fn launch() -> std::io::Result<()>;
}

// ----------- //
// Énumération //
// ----------- //

/// Interface utilisateur
pub enum UI {
	/// Interface utilisateur graphique
	Graphical,

	/// Interface utilisateur textuel
	Textual,
}
