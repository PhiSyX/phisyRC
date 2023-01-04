/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

#![allow(dead_code)]

// Cet exemple montre comment configurer le logger avec stdout en utilisant le
// mode de processus de la CLI.

// Il y a deux types de logger:
//     - Normal (stdout)
//     - Avec une interface (tui)

#[derive(Debug)]
struct Cli {
	mode: cli::ProcessEnv,
}

impl Cli {
	fn arguments() -> Self {
		Self {
			mode: cli::ProcessEnv::DEVELOPMENT,
		}
	}
}

impl setup::SetupCliInterface for Cli {
	fn process_env(&self) -> cli::ProcessEnv {
		self.mode
	}
}

#[phisyrc_macro::setup(logger = "stdout")]
fn main(cli_args: Cli) {
	logger::info!("Hello World");
}
