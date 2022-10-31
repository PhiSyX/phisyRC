/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

#![allow(dead_code)]

// Cet exemple montre comment configurer le logger avec stdout en utilisant le
// mode de processus de la CLI et le filtre de debug des variables
// d'environnement.

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

#[derive(Debug)]
struct Env {
	debug_filter: &'static str,
}

#[derive(Debug)]
enum EnvError {}

impl Env {
	fn setup(_: &str) -> Result<Self, EnvError> {
		Ok(Self { debug_filter: "*" })
	}
}

impl setup::SetupEnvInterface for Env {
	fn debug_filter(&self) -> String {
		self.debug_filter.to_owned()
	}
}

#[phisyrc_macro::setup(logger = "stdout")]
async fn main(cli_args: Cli, env_args: Env) {
	logger::info!("Hello World");
}
