/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::io::{self, Write};

use futures::{FutureExt, StreamExt};
use log::{
	max_level, set_boxed_logger, set_logger, set_max_level, LevelFilter, Log,
	Metadata, Record, SetLoggerError,
};
use terminal::{
	crossterm::{
		event::{
			DisableMouseCapture, EnableMouseCapture, Event, EventStream,
			KeyCode, KeyEvent, KeyModifiers, MouseEvent,
		},
		execute,
		terminal::{
			disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
			LeaveAlternateScreen,
		},
	},
	tui::{
		backend::{Backend, CrosstermBackend},
		layout::{Constraint, Direction, Layout, Rect},
		style::{Color, Modifier, Style},
		text::{Span, Spans},
		widgets::{Block, Borders, List, ListItem, Paragraph},
		Frame, Terminal,
	},
	view::Interface as ViewInterface,
	EventLoop,
};
use tokio::time;

use crate::{
	builder::Builder,
	echo::{Entry, LoggerWriter},
	FilterFn, LoggerReader, NO,
};

// --------- //
// Structure //
// --------- //

pub struct Logger {
	pub colorized: bool,
	pub timestamp: bool,
	pub level: Option<LevelFilter>,
	pub filters_fn: Vec<FilterFn>,
	pub writer: LoggerWriter,
}

#[allow(clippy::upper_case_acronyms)]
pub struct Tui<W, C>
where
	W: Write,
	C: EventLoop,
{
	ctx: tokio::sync::mpsc::Sender<C>,
	terminal: Terminal<CrosstermBackend<W>>,
}

pub struct View {
	reader: LoggerReader,
	logs: Vec<Entry>,
	scroll_position: usize,
	input_line: String,
}

// -------------- //
// Implémentation // -> API Publique
// -------------- //

impl Logger {
	pub fn builder() -> Builder {
		Builder::default()
	}

	pub(super) fn apply(self) -> Result<(), SetLoggerError> {
		let level = self.level.unwrap_or(LevelFilter::Off);
		set_max_level(level);
		if LevelFilter::Off == max_level() {
			set_logger(&NO)
		} else {
			set_boxed_logger(Box::new(self))
		}
	}
}

impl<C> Tui<io::Stdout, C>
where
	C: EventLoop,
{
	pub async fn launch(
		ctx: tokio::sync::mpsc::Sender<C>,
		reader: LoggerReader,
	) -> io::Result<()> {
		let view = View::new(reader);
		let stdout = io::stdout();
		let mut tui = Self::new(ctx, stdout)?;
		tui.run(view).await
	}
}

// -------------- //
// Implémentation //
// -------------- //

impl<W, C> Tui<W, C>
where
	W: Write,
	C: EventLoop,
{
	fn new(
		ctx: tokio::sync::mpsc::Sender<C>,
		mut output: W,
	) -> io::Result<Self> {
		enable_raw_mode().and_then(|_| {
			execute!(output, EnterAlternateScreen, EnableMouseCapture)
		})?;

		Ok(Self {
			ctx,
			terminal: Terminal::new(CrosstermBackend::new(output))?,
		})
	}

	async fn run(&mut self, mut view: View) -> io::Result<()> {
		let mut event_stream = EventStream::new();

		let timeout = time::Duration::from_millis(64);

		loop {
			let maybe_event =
				time::timeout(timeout, event_stream.next().fuse());

			tokio::select! {
				incoming_log = view.reader.recv() => match incoming_log {
					| Some(entry) => view.logs.push(entry),
					| None => continue,
				},

				Ok(Some(Ok(event))) = maybe_event => {
					if let Event::Key(event) = event {
						// NOTE(phisyx): quitter l'application.
						let quit_key_bindings = ['c', 'C', 'q', 'Q'].map(KeyCode::Char);
						if event.modifiers == KeyModifiers::CONTROL
							&& quit_key_bindings.contains(&event.code)
						{
							_ = self.ctx.send(C::quit());
							break;
						}

						view.update_keyboard_event(event).await;
					}
				}
			};

			self.render(&mut view)?;
		}

		Ok(())
	}

	fn render(&mut self, view: &mut View) -> io::Result<()> {
		self.terminal
			.draw(|frame| view.render(frame, frame.size()))
			.map(|_| ())
	}
}

impl View {
	fn new(reader: LoggerReader) -> Self {
		Self {
			reader,
			logs: Default::default(),
			scroll_position: Default::default(),
			input_line: Default::default(),
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

#[async_trait::async_trait]
impl ViewInterface for View {
	fn render(&mut self, frame: &mut Frame<impl Backend>, _: Rect) {
		let split = Layout::default()
			.direction(Direction::Vertical)
			.constraints([
				Constraint::Length(frame.size().height - 3),
				Constraint::Min(3),
			])
			.split(frame.size());

		let items: Vec<ListItem> = self
			.logs
			.iter()
			.skip(self.scroll_position)
			.map(|entry| {
				let style = entry.style();
				let default_style = Style::default().fg(Color::Gray);
				let spans = [
					Span::styled(
						entry.level.to_string(),
						style.add_modifier(Modifier::BOLD),
					),
					Span::styled(" [", default_style),
					Span::styled(&entry.target, default_style),
					Span::styled("] ", default_style),
					Span::styled(&entry.args, style),
				];
				ListItem::new(Spans::from(spans.to_vec()))
			})
			.collect();

		let items = List::new(items).block(
			Block::default()
				.borders(Borders::ALL.difference(Borders::BOTTOM))
				.title(" Logger "),
		);

		frame.render_widget(items, split[0]);

		let input = Paragraph::new(self.input_line.as_str()).block(
			Block::default()
				.borders(Borders::ALL)
				.title(" Boite de saisie "),
		);

		frame.set_cursor(
			split[1].x + (self.input_line.len()) as u16 + 1,
			split[1].y + 1,
		);

		frame.render_widget(input, split[1]);
	}

	async fn update_keyboard_event(&mut self, event: KeyEvent) {
		match event.code {
			| KeyCode::Char(ch) => {
				self.input_line.push(ch);
			}

			| KeyCode::Backspace => {
				self.input_line.pop();
			}

			| KeyCode::Enter => {
				log::warn!(
					"TODO: envoyer la ligne au serveur: {}",
					&self.input_line
				);
			}
			| KeyCode::Left => {
				log::warn!("TODO: déplacer le curseur vers la gauche.")
			}
			| KeyCode::Right => {
				log::warn!("TODO: déplacer le curseur vers la droite.");
			}

			| KeyCode::Up => {
				self.scroll_position = self.scroll_position.saturating_sub(1);
			}
			| KeyCode::Down => {
				self.scroll_position = self.scroll_position.saturating_add(1);
			}
			| KeyCode::Home => {
				self.scroll_position = 0;
			}
			| KeyCode::End => {
				self.scroll_position = self.logs.len().saturating_sub(1);
			}
			| KeyCode::PageUp => {
				self.scroll_position = self.scroll_position.saturating_sub(10);
			}
			| KeyCode::PageDown => {
				self.scroll_position = self
					.scroll_position
					.saturating_add(10)
					.min(self.logs.len() - 1);
			}
			| _ => {}
		}
	}

	async fn update_mouse_event(&mut self, _: MouseEvent) {}
}

impl Log for Logger {
	/// On ne veut pas afficher les logs si le niveau est à
	/// [LevelFilter::Off].
	/// Des conditions utilisateurs peuvent être utilisées pour
	/// filtrer les logs.
	fn enabled(&self, metadata: &Metadata) -> bool {
		metadata.level() != LevelFilter::Off
			&& self.filters_fn.iter().all(|boxed_fn| boxed_fn(metadata))
	}

	/// Affiche le log.
	fn log(&self, record: &Record) {
		if !self.enabled(record.metadata()) {
			return;
		}

		let message = record.args();
		if message.to_string().trim().is_empty() {
			return;
		}

		if let Err(err) = self.writer.send(record.into()) {
			eprintln!("Erreur lors de l'envoie du log: {err}");
		}
	}

	fn flush(&self) {}
}

impl<W, C> Drop for Tui<W, C>
where
	W: Write,
	C: EventLoop,
{
	fn drop(&mut self) {
		execute!(
			self.terminal.backend_mut(),
			LeaveAlternateScreen,
			DisableMouseCapture
		)
		.and_then(|_| disable_raw_mode())
		.expect("Tui#drop");

		println!("Arrêt du logger.");
	}
}
