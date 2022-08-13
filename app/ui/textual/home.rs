/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::borrow::BorrowMut;

use cli::{
	event::{KeyCode, KeyEvent, MouseEvent},
	PROJECT_NAME,
};
use tui::{
	backend::Backend,
	layout::{Alignment, Constraint, Direction, Layout, Rect},
	style::{Color, Style},
	text::{Span, Spans, Text},
	widgets::{
		Block, BorderType, Borders, List, ListItem, ListState, Paragraph,
	},
	Frame,
};

use super::ViewInterface;

// --------- //
// Structure //
// --------- //

pub struct HomeView {
	state: State,
	cursor_home: ListState,

	cursor_login: ListState,
	nickname: String,
	channels: String,
}

// ----------- //
// Énumération //
// ----------- //

#[derive(Default)]
enum State {
	#[default]
	Initial,

	PopupLogin,
}

enum Cursor {
	Home,
	Login,
}

// -------------- //
// Implémentation //
// -------------- //

impl HomeView {
	const HOME_NAVIGATION_LIST: [&'static str; 2] =
		["Connexion au Chat", "Mes préférences"];

	/// Dessine la vue initiale de la page d'accueil.
	fn draw_initial(&mut self, frame: &mut Frame<impl Backend>, chunk: Rect) {
		let layout = Layout::default()
			.direction(Direction::Vertical)
			.constraints(
				[Constraint::Percentage(20), Constraint::Percentage(80)]
					.as_ref(),
			)
			.split(chunk);
		self.display_project_name(frame, layout[0]);
		self.display_navigation_list(frame, layout[1]);
	}

	/// Gestion des touches du clavier sur la page d'accueil.
	fn update_keyboard_initial(&mut self, event: KeyEvent) {
		match event.code {
			| KeyCode::Home => self.navigate_to_first_element(Cursor::Home),
			| KeyCode::End => self.navigate_to_last_element(Cursor::Home),
			| KeyCode::Up => self.navigate_to_previous_element(Cursor::Home),
			| KeyCode::Down => self.navigate_to_next_element(Cursor::Home),
			| KeyCode::Enter => {
				if let Some(index) = self.cursor_home.selected() {
					self.state = match index {
						| 0 => State::PopupLogin,
						| _ => {
							todo!("Mes préférences");
						}
					};
				}
			}
			| _ => {}
		}
	}

	/// Dessine le menu de navigation de la page d'accueil.
	fn display_navigation_list(
		&mut self,
		frame: &mut Frame<impl Backend>,
		layout: Rect,
	) {
		let layouts = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([
				Constraint::Percentage(40),
				Constraint::Percentage(25),
				Constraint::Percentage(40),
			])
			.split(layout);

		let navigation_list = Self::HOME_NAVIGATION_LIST
			.map(|item| ListItem::new(Spans::from(item)));

		let widget_list = List::new(navigation_list)
			.highlight_style(Style::default().bg(Color::Rgb(55, 100, 141)))
			.highlight_symbol("|> ");

		frame.render_stateful_widget(
			widget_list,
			layouts[1],
			&mut self.cursor_home,
		);
	}

	/// Affiche le nom du projet en haut au centre de la page d'accueil.
	fn display_project_name(
		&self,
		frame: &mut Frame<impl Backend>,
		layout: Rect,
	) {
		let project_name: Vec<Spans> = PROJECT_NAME
			.lines()
			.map(Span::from)
			.map(Spans::from)
			.collect();
		let art = Text::from(project_name);
		let widget_paragraph = Paragraph::new(art).alignment(Alignment::Center);
		frame.render_widget(widget_paragraph, layout);
	}

	/// Déplace le curseur vers l'élément suivant de la liste de navigation.
	fn navigate_to_next_element(&mut self, cursor: Cursor) {
		let cursor_mut = match cursor {
			| Cursor::Home => self.cursor_home.borrow_mut(),
			| Cursor::Login => self.cursor_login.borrow_mut(),
		};
		let cursor_list = match cursor {
			| Cursor::Home => Self::HOME_NAVIGATION_LIST,
			| Cursor::Login => Self::LOGIN_LABEL,
		};

		let index = cursor_mut.selected().map(|index| {
			if index >= cursor_list.len() - 1 {
				0
			} else {
				index + 1
			}
		});
		cursor_mut.select(index.or(Some(0)));
	}

	/// Déplace le curseur vers l'élément précédent de la liste de navigation.
	fn navigate_to_previous_element(&mut self, cursor: Cursor) {
		let cursor_mut = match cursor {
			| Cursor::Home => self.cursor_home.borrow_mut(),
			| Cursor::Login => self.cursor_login.borrow_mut(),
		};
		let cursor_list = match cursor {
			| Cursor::Home => Self::HOME_NAVIGATION_LIST,
			| Cursor::Login => Self::LOGIN_LABEL,
		};

		let index = cursor_mut.selected().map(|index| {
			if index == 0 {
				cursor_list.len() - 1
			} else {
				index - 1
			}
		});
		cursor_mut.select(index.or(Some(0)));
	}

	/// Déplace le curseur vers le premier élément de la liste de navigation.
	fn navigate_to_first_element(&mut self, cursor: Cursor) {
		let cursor_mut = match cursor {
			| Cursor::Home => self.cursor_home.borrow_mut(),
			| Cursor::Login => self.cursor_login.borrow_mut(),
		};

		cursor_mut.select(0.into())
	}

	/// Déplace le curseur vers le dernier élément de la liste de navigation.
	fn navigate_to_last_element(&mut self, cursor: Cursor) {
		let cursor_mut = match cursor {
			| Cursor::Home => self.cursor_home.borrow_mut(),
			| Cursor::Login => self.cursor_login.borrow_mut(),
		};
		let cursor_list = match cursor {
			| Cursor::Home => Self::HOME_NAVIGATION_LIST,
			| Cursor::Login => Self::LOGIN_LABEL,
		};

		cursor_mut.select((cursor_list.len() - 1).into())
	}
}

impl HomeView {
	const LOGIN_LABEL: [&'static str; 2] = ["Mon pseudonyme", "Mes salons"];

	fn draw_popup_login(
		&mut self,
		frame: &mut Frame<impl Backend>,
		chunk: Rect,
	) {
		let block = Block::default()
			.title(format!(" {} ", Self::HOME_NAVIGATION_LIST[0]))
			.title_alignment(Alignment::Center)
			.borders(Borders::all())
			.border_type(BorderType::Rounded);

		let area = Self::centered_rect(60, 40, chunk);

		let layouts = Layout::default()
			.direction(Direction::Horizontal)
			.constraints([
				Constraint::Percentage(40),
				Constraint::Percentage(60),
			])
			.split(area);

		let label_list =
			Self::LOGIN_LABEL.map(|item| ListItem::new(Spans::from(item)));

		let widget_list = List::new(label_list)
			.block(Block::default().borders(Borders::LEFT.union(Borders::TOP)))
			.highlight_style(Style::default().fg(Color::Yellow));

		frame.render_stateful_widget(
			widget_list,
			layouts[0],
			&mut self.cursor_login,
		);

		let input_list = [self.nickname.clone(), self.channels.clone()]
			.map(|item| ListItem::new(Spans::from(item)));

		let widget_list = List::new(input_list)
			.block(Block::default().borders(Borders::RIGHT.union(Borders::TOP)))
			.highlight_style(Style::default().fg(Color::Yellow));

		frame.render_stateful_widget(
			widget_list,
			layouts[1],
			&mut self.cursor_login,
		);

		frame.render_widget(block, area);
	}

	/// Gestion des touches du clavier sur la page d'accueil.
	fn update_keyboard_popup_login(&mut self, event: KeyEvent) {
		match event.code {
			| KeyCode::Esc => {
				self.state = State::Initial;
			}
			| KeyCode::BackTab | KeyCode::Up => {
				self.navigate_to_previous_element(Cursor::Login)
			}
			| KeyCode::Tab | KeyCode::Down => {
				self.navigate_to_next_element(Cursor::Login)
			}
			| KeyCode::Char(ch) => {
				if let Some(0) = self.cursor_login.selected() {
					self.nickname.push(ch)
				}
				if let Some(1) = self.cursor_login.selected() {
					self.channels.push(ch)
				}
			}
			| KeyCode::Backspace => {
				if let Some(0) = self.cursor_login.selected() {
					self.nickname.pop();
				}
				if let Some(1) = self.cursor_login.selected() {
					self.channels.pop();
				}
			}
			| _ => {}
		}
	}

	fn centered_rect(px: u16, py: u16, r: Rect) -> Rect {
		let pl = Layout::default()
			.direction(Direction::Vertical)
			.constraints([
				Constraint::Percentage((100 - py) / 2),
				Constraint::Percentage(py),
				Constraint::Percentage((100 - py) / 2),
			])
			.split(r);

		Layout::default()
			.direction(Direction::Horizontal)
			.constraints([
				Constraint::Percentage((100 - px) / 2),
				Constraint::Percentage(px),
				Constraint::Percentage((100 - px) / 2),
			])
			.split(pl[1])[1]
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl Default for HomeView {
	fn default() -> Self {
		let mut cursor = ListState::default();
		cursor.select(0.into());
		Self {
			state: Default::default(),
			cursor_home: cursor.clone(),
			cursor_login: cursor,
			nickname: Default::default(),
			channels: Default::default(),
		}
	}
}

#[async_trait::async_trait]
impl ViewInterface for HomeView {
	fn render(&mut self, frame: &mut Frame<impl Backend>, chunk: Rect) {
		self.draw_initial(frame, chunk);
		match self.state {
			| State::Initial => {}
			| State::PopupLogin => {
				self.draw_popup_login(frame, chunk);
			}
		}
	}

	async fn update_keyboard_event(&mut self, event: KeyEvent) {
		match self.state {
			| State::Initial => {
				self.update_keyboard_initial(event);
			}
			| State::PopupLogin => {
				self.update_keyboard_popup_login(event);
			}
		}
	}

	async fn update_mouse_event(&mut self, _: MouseEvent) {}
}
