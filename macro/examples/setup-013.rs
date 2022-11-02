/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

#![allow(dead_code)]

// Cet exemple montre comment configurer le logger avec une interface (tui).

// Il y a deux types de logger:
//     - Normal (stdout)
//     - Avec une interface (tui)

use tokio::{sync::mpsc, time};

enum AppContext {
	Input(String),
	Quit,
}

impl AppContext {
	fn mpsc() -> (mpsc::UnboundedSender<Self>, mpsc::UnboundedReceiver<Self>) {
		mpsc::unbounded_channel()
	}
}

impl terminal::EventLoop for AppContext {
	fn input(input: String) -> Self {
		Self::Input(input)
	}

	fn quit() -> Self {
		Self::Quit
	}
}

#[phisyrc_macro::setup(logger = "tui")]
async fn main<Context>()
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
