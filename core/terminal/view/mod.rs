/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crossterm::event::{KeyEvent, MouseEvent};
use tui::{backend::Backend, layout::Rect, Frame};

// --------- //
// Interface //
// --------- //

#[async_trait::async_trait]
pub trait Interface {
	fn render(&mut self, frame: &mut Frame<impl Backend>, chunk: Rect);
	async fn update_keyboard_event(&mut self, event: KeyEvent);
	async fn update_mouse_event(&mut self, event: MouseEvent);
}
