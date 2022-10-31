/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

// Cet exemple montre comment configurer le logger avec une interface (tui) en
// utilisant les arguments de la CLI.

// Il y a deux types de logger:
//     - Normal (stdout)
//     - Avec une interface (tui)

use tokio::time;

enum AppContext {
	Input(String),
	Quit,
}

impl terminal::EventLoop for AppContext {
	fn input(input: String) -> Self {
		Self::Input(input)
	}

	fn quit() -> Self {
		Self::Quit
	}
}

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

#[phisyrc_macro::setup(logger = "tui")]
async fn main<Context>(cli_args: Cli)
where
	Context: AppContext,
	[crx]: mpsc::UnboundedReceiver<AppContext>,
{
	let receiver_context_task = tokio::spawn(async move {
		loop {
			tokio::select! {
				Some(app_ctx) = crx.recv() => match app_ctx {
					| AppContext::Quit => break,

					| AppContext::Input(msg) => {
						logger::warn!("Reception du message: {msg}");
					}
				}
			}
		}
	});

	logger::info!("Hello World");

	let mut i = 0;
	loop {
		if receiver_context_task.is_finished() {
			break;
		}

		if i % 10 == 0 {
			logger::info!("Compteur : {i}");
		}

		i += 1;
		if i == 1000 {
			break;
		}

		logger::warn!(""); // HACK(phisyx): permet de ne pas bloquer l'interface.
		time::sleep(time::Duration::from_millis(64)).await;
	}
}
