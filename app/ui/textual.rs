/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod home;

use std::io::{self, Write};

use crossterm::{
	event::{
		self, Event, EventStream, KeyCode, KeyEvent, KeyModifiers, MouseEvent,
	},
	execute, terminal,
};
use futures::{FutureExt, StreamExt};
use tokio::time;
use tui::{
	backend::{Backend, CrosstermBackend},
	layout::Rect,
	Frame, Terminal,
};

use self::home::HomeView;
use super::UInterface;

// --------- //
// Interface //
// --------- //

#[async_trait::async_trait]
pub trait ViewInterface {
	fn render(&mut self, frame: &mut Frame<impl Backend>, chunk: Rect);
	async fn update_keyboard_event(&mut self, event: KeyEvent);
	async fn update_mouse_event(&mut self, event: MouseEvent);
}

// --------- //
// Structure //
// --------- //

#[allow(clippy::upper_case_acronyms)]
pub struct TUI<W: Write> {
	terminal: Terminal<CrosstermBackend<W>>,
}

#[derive(Default)]
pub struct View {
	home_view: HomeView,
}

// -------------- //
// Implementation //
// -------------- //

impl<W: Write> TUI<W> {
	fn new(mut output: W) -> io::Result<Self> {
		terminal::enable_raw_mode()?;

		execute!(
			output,
			terminal::EnterAlternateScreen,
			event::EnableMouseCapture
		)?;

		Ok(Self {
			terminal: Terminal::new(CrosstermBackend::new(output))?,
		})
	}

	fn render(&mut self, view: &mut View) -> io::Result<()> {
		self.terminal
			.draw(|frame| view.render(frame, frame.size()))
			.map(|_| ())
	}

	async fn run(&mut self, mut view: View) -> io::Result<()> {
		let mut event_stream = EventStream::new();

		let timeout = time::Duration::from_millis(64);

		loop {
			self.render(&mut view)?;

			let maybe_event =
				time::timeout(timeout, event_stream.next().fuse()).await;
			if maybe_event.is_err() {
				continue;
			}

			let maybe_event = maybe_event.unwrap();
			if maybe_event.is_none() {
				continue;
			}

			let maybe_event = maybe_event.unwrap();
			if maybe_event.is_err() {
				continue;
			}

			let event = maybe_event.unwrap();

			if let Event::Key(event) = event {
				// NOTE(phisyx): quitter l'application de Chat.
				let quit_key_bindings = ['c', 'C', 'q', 'Q'].map(KeyCode::Char);
				if event.modifiers == KeyModifiers::CONTROL
					&& quit_key_bindings.contains(&event.code)
				{
					break;
				}

				view.update_keyboard_event(event).await;
			}
		}

		Ok(())
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

#[async_trait::async_trait]
impl UInterface for TUI<io::Stdout> {
	async fn launch() -> io::Result<()> {
		let view = View::default();
		let stdout = io::stdout();
		let mut tui = Self::new(stdout)?;
		tui.run(view).await
	}
}

#[async_trait::async_trait]
impl ViewInterface for View {
	fn render(&mut self, frame: &mut Frame<impl Backend>, chunk: Rect) {
		self.home_view.render(frame, chunk)
	}

	async fn update_keyboard_event(&mut self, event: KeyEvent) {
		self.home_view.update_keyboard_event(event).await;
	}

	async fn update_mouse_event(&mut self, event: MouseEvent) {
		let _ = self.home_view.update_mouse_event(event);
	}
}

impl<W: Write> Drop for TUI<W> {
	fn drop(&mut self) {
		execute!(
			self.terminal.backend_mut(),
			terminal::LeaveAlternateScreen,
			event::DisableMouseCapture
		)
		.and_then(|_| terminal::disable_raw_mode())
		.expect("TUI#drop");

		println!("Arrêt de l'application de Chat textuel.");
	}
}
