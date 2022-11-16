/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// NOTE(phisyx): Pas fan de cette partie de code.

use cli::ProcessEnv;
use helpers::lang::WildcardMatching;
use logger::{stdout, tui};
use tokio::sync::mpsc;

use crate::{Result, SetupCliInterface, SetupEnvInterface};

// ---- //
// Sync //
// ---- //

// Le logger par défaut utilisé est `stdout`.
pub fn logger() -> Result<()> {
	let level_filter = if cfg!(debug_assertions) {
		logger::LevelFilter::Debug
	} else {
		logger::LevelFilter::Off
	};

	let debug_filter = "*";

	let logger_builder = stdout::Logger::builder()
		.with_color()
		.with_level(level_filter)
		.with_timestamp()
		.filter(move |metadata| metadata.target().iswm(debug_filter));

	Ok(logger_builder.build_stdout()?)
}

pub fn logger_1<C>(args: &C) -> Result<()>
where
	C: SetupCliInterface,
{
	let cli_args = args;

	let level_filter = match &cli_args.process_env() {
		| ProcessEnv::DEVELOPMENT => logger::LevelFilter::Debug,
		| ProcessEnv::PRODUCTION => logger::LevelFilter::Off,
		| ProcessEnv::TEST => logger::LevelFilter::Trace,
	};

	let debug_filter = "*";

	let logger_builder = stdout::Logger::builder()
		.with_color()
		.with_level(level_filter)
		.with_timestamp()
		.filter(move |metadata| metadata.target().iswm(debug_filter));

	Ok(logger_builder.build_stdout()?)
}

pub fn logger_2<C, E>(args: (&C, &E)) -> Result<()>
where
	C: SetupCliInterface,
	E: SetupEnvInterface,
{
	let (cli_args, env_args) = args;

	let level_filter = match &cli_args.process_env() {
		| ProcessEnv::DEVELOPMENT => logger::LevelFilter::Debug,
		| ProcessEnv::PRODUCTION => logger::LevelFilter::Off,
		| ProcessEnv::TEST => logger::LevelFilter::Trace,
	};

	let debug_filter = env_args.debug_filter();

	let logger_builder = stdout::Logger::builder()
		.with_color()
		.with_level(level_filter)
		.with_timestamp()
		.filter(move |metadata| metadata.target().iswm(&debug_filter));

	Ok(logger_builder.build_stdout()?)
}

// ----- //
// Async //
// ----- //

pub async fn future_logger_tui<Context>(
	ctx: mpsc::UnboundedSender<Context>,
) -> Result<()>
where
	Context: terminal::EventLoop,
{
	let level_filter = if cfg!(debug_assertions) {
		logger::LevelFilter::Debug
	} else {
		logger::LevelFilter::Off
	};

	let debug_filter = "*";

	let logger_builder = tui::Logger::builder()
		.with_color()
		.with_level(level_filter)
		.with_timestamp()
		.filter(move |metadata| metadata.target().iswm(debug_filter));
	Ok(logger_builder.build_tui(ctx).await?)
}

pub async fn future_logger_tui_1<C, Context>(
	args: &C,
	ctx: mpsc::UnboundedSender<Context>,
) -> Result<()>
where
	C: SetupCliInterface,
	Context: terminal::EventLoop,
{
	let cli_args = args;

	let level_filter = match &cli_args.process_env() {
		| ProcessEnv::DEVELOPMENT => logger::LevelFilter::Debug,
		| ProcessEnv::PRODUCTION => logger::LevelFilter::Off,
		| ProcessEnv::TEST => logger::LevelFilter::Trace,
	};

	let debug_filter = "*";

	let logger_builder = tui::Logger::builder()
		.with_color()
		.with_level(level_filter)
		.with_timestamp()
		.filter(move |metadata| metadata.target().iswm(debug_filter));
	Ok(logger_builder.build_tui(ctx).await?)
}

pub async fn future_logger_tui_2<C, E, Context>(
	args: (&C, &E),
	ctx: mpsc::UnboundedSender<Context>,
) -> Result<()>
where
	C: SetupCliInterface,
	E: SetupEnvInterface,
	Context: terminal::EventLoop,
{
	let (cli_args, env_args) = args;

	let level_filter = match &cli_args.process_env() {
		| ProcessEnv::DEVELOPMENT => logger::LevelFilter::Debug,
		| ProcessEnv::PRODUCTION => logger::LevelFilter::Off,
		| ProcessEnv::TEST => logger::LevelFilter::Trace,
	};

	let debug_filter = env_args.debug_filter();

	let logger_builder = tui::Logger::builder()
		.with_color()
		.with_level(level_filter)
		.with_timestamp()
		.filter(move |metadata| metadata.target().iswm(&debug_filter));

	Ok(logger_builder.build_tui(ctx).await?)
}

// -------- //
// Sync Alt //
// -------- //

pub fn logger_stdout() -> Result<()> {
	logger()
}
pub fn logger_stdout_1<C>(args: &C) -> Result<()>
where
	C: SetupCliInterface,
{
	logger_1(args)
}
pub fn logger_stdout_2<C, E>(args: (&C, &E)) -> Result<()>
where
	C: SetupCliInterface,
	E: SetupEnvInterface,
{
	logger_2(args)
}

// --------- //
// Async Alt //
// --------- //

pub async fn future_logger() -> Result<()> {
	logger()
}
pub async fn future_logger_1<C>(args: &C) -> Result<()>
where
	C: SetupCliInterface,
{
	logger_1(args)
}
pub async fn future_logger_2<C, E>(args: (&C, &E)) -> Result<()>
where
	C: SetupCliInterface,
	E: SetupEnvInterface,
{
	logger_2(args)
}
pub async fn future_logger_stdout() -> Result<()> {
	logger()
}
pub async fn future_logger_stdout_1<C>(args: &C) -> Result<()>
where
	C: SetupCliInterface,
{
	logger_1(args)
}
pub async fn future_logger_stdout_2<C, E>(args: (&C, &E)) -> Result<()>
where
	C: SetupCliInterface,
	E: SetupEnvInterface,
{
	logger_2(args)
}
