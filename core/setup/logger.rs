/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::str::FromStr;

use cli::ProcessEnv;
use helpers::lang::WildcardMatching;
use logger::stdout;
use tokio::sync::mpsc;

use crate::{Result, SetupCliInterface, SetupEnvInterface};

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Default)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
pub enum LoggerType {
	#[default]
	Stdout,
	Tui,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl FromStr for LoggerType {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			| "tui" => Self::Tui,
			| _ => Self::Stdout,
		})
	}
}

impl<'a> From<&'a str> for LoggerType {
	fn from(s: &'a str) -> Self {
		s.parse().unwrap_or_default()
	}
}

// FIXME(phisyx): dépendant de tokio (mpsc).
//
// FIXME(phisyx): le stdout étant changé lorsqu'on utilise TUI il y a un léger
// souci avec la configuration interactive si l'attribut setup::config n'est
// jamais appelé.
pub async fn logger<C, E, Ctx>(
	args: (&C, &E),
	ty: impl Into<LoggerType>,
	ctx: mpsc::UnboundedSender<Ctx>,
) -> Result<()>
where
	C: SetupCliInterface,
	E: SetupEnvInterface,
	Ctx: terminal::EventLoop,
{
	let (cli_args, env_args) = args;

	let level_filter = match &cli_args.process_env() {
		| ProcessEnv::DEVELOPMENT => logger::LevelFilter::Debug,
		| ProcessEnv::PRODUCTION => logger::LevelFilter::Off,
		| ProcessEnv::TEST => logger::LevelFilter::Trace,
	};

	let logger_type = ty.into();

	let debug_filter = env_args.debug_filter();
	let logger_builder = stdout::Logger::builder()
		.with_color()
		.with_level(level_filter)
		.with_timestamp()
		.filter(move |metadata| metadata.target().iswm(&debug_filter));

	if LoggerType::Tui == logger_type {
		return Ok(logger_builder.build_tui(ctx).await?);
	}

	Ok(logger_builder.build_stdout()?)
}
