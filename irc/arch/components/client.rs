/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::{
	arch::{AtomicEntity, AtomicServerConfig},
	commands::{IrcClientCommand, IrcCommandNumeric, IrcReplies},
	config::IrcdPasswordAlgorithm,
};

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
#[derive(Clone)]
pub struct IrcClient {
	entity: AtomicEntity,
	registered: bool,
	password: Option<String>,
	nick: String,
	ident: String,
	mode: String,
	realname: String,
}

// -------------- //
// Implémentation //
// -------------- //

impl IrcClient {
	pub fn new(entity: AtomicEntity) -> Self {
		Self {
			entity,
			registered: false,
			password: Default::default(),
			nick: Default::default(),
			ident: Default::default(),
			mode: Default::default(),
			realname: Default::default(),
		}
	}
}

impl IrcClient {
	pub fn is_registered(&self) -> bool {
		self.registered
	}

	pub fn prefix(&self) -> Option<String> {
		if self.nick.is_empty() || self.ident.is_empty() {
			return None;
		}

		format!("{}!{}", self.nick, self.ident).into()
	}

	pub fn prefix_based_on_reply(&self, reply: &IrcCommandNumeric) -> String {
		match reply {
			| IrcCommandNumeric::RPL_WELCOME { .. }
			| IrcCommandNumeric::RPL_YOURHOST { .. }
			| IrcCommandNumeric::RPL_CREATED { .. } => self.nick.clone(),
			| _ => unsafe { self.prefix().unwrap_unchecked() },
		}
	}
}

impl IrcClient {
	pub async fn handle_upass_command(
		&mut self,
		server_config: AtomicServerConfig,
		command: &IrcClientCommand,
	) -> Result<(), IrcCommandNumeric> {
		assert!(matches!(command, IrcClientCommand::PASS { .. }));

		// NOTE(phisyx): ignorer le fait que le client envoie un mot de passe
		// si le serveur n'a pas de mot de passe dans sa configuration.
		if server_config.user.password.is_none() {
			return Ok(());
		}

		if let IrcClientCommand::PASS { password, .. } = command {
			let password_cfg =
				&self.entity.lock().await.server.config.clone().user.password;

			let password = match password_cfg {
				| Some(cfg) => match cfg.algo {
					| IrcdPasswordAlgorithm::Plain => password.to_owned(),
					| IrcdPasswordAlgorithm::Argon2 => {
						let app_secret_key = std::env::var("APP_SECRET_KEY")
							.expect(
								"La variable d'environnement `APP_SECRET_KEY`
								  est obligatoirement présente car nous
								  demandons que celle-ci soit obligatoire.",
							);

						let config = argon2::Config {
							variant: argon2::Variant::Argon2id,
							thread_mode: argon2::ThreadMode::Parallel,
							..argon2::Config::default()
						};

						argon2::hash_encoded(
							password.as_bytes(),
							app_secret_key.as_bytes(),
							&config,
						)
						.expect("Argon2")
					}
				},
				| None => password.to_owned(),
			};

			self.password.replace(password);
		}

		Ok(())
	}

	pub async fn handle_unick_command(
		&mut self,
		command: &IrcClientCommand,
	) -> Result<Vec<IrcReplies>, IrcCommandNumeric> {
		assert!(matches!(command, IrcClientCommand::NICK { .. }));

		if let IrcClientCommand::NICK { nickname, .. } = command {
			self.nick = nickname.to_owned();
		}

		Ok(self.complete_registration().await)
	}

	pub async fn handle_uuser_command(
		&mut self,
		command: &IrcClientCommand,
	) -> Result<Vec<IrcReplies>, IrcCommandNumeric> {
		assert!(matches!(command, IrcClientCommand::USER { .. }));

		if let IrcClientCommand::USER {
			user,
			mode,
			realname,
			..
		} = command
		{
			self.ident = user.to_owned();
			self.mode = mode.to_owned();
			self.realname = realname.to_owned();
		}

		Ok(self.complete_registration().await)
	}

	async fn complete_registration(&mut self) -> Vec<IrcReplies> {
		if self.nick.is_empty()
			|| self.ident.is_empty()
			|| self.realname.is_empty()
			|| self.mode.is_empty()
		{
			return vec![];
		}

		let entity = self.entity.lock().await;

		if let (Some(password_cfg), Some(password)) =
			(&entity.server.config.clone().user.password, &self.password)
		{
			if password_cfg.secret.ne(password) {
				return vec![IrcReplies::Error(
					"You are not authorized to connect to this server"
						.to_owned(),
				)];
			}
		}

		self.registered = true;

		let server_cfg = entity.server.config.clone();

		let mut replies = Vec::with_capacity(3);

		let welcome_001 = IrcCommandNumeric::RPL_WELCOME {
			nick: self.nick.to_owned(),
			user: self.ident.to_owned(),
			host: entity.addr.to_string(),
		};

		let yourhost_002 = IrcCommandNumeric::RPL_YOURHOST {
			servername: server_cfg.user.name.to_owned(),
			ver: "v0.1.0".into(),
		};

		let created_003 = IrcCommandNumeric::RPL_CREATED {
			date: entity
				.server
				.created_at
				.format("%Y-%m-%d %H:%M:%S.%f")
				.to_string(),
		};

		replies.push(welcome_001);
		replies.push(yourhost_002);
		replies.push(created_003);

		replies.into_iter().map(IrcReplies::Numeric).collect()
	}
}
