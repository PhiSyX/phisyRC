/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

// Cette exemple envoie simplement des données au serveur.

use std::io::{self, BufRead};

struct AppClient {
	#[allow(dead_code)]
	inner: network::Client<Self>,
}

#[network::async_trait]
impl network::client::Interface for AppClient {
	async fn binary(&mut self, bytes: Vec<u8>) -> network::Result<()> {
		println!(">> réception des données binaires : {bytes:?}");
		Ok(())
	}
}

#[phisyrc::setup]
async fn main() {
	let cfg = config::load::<config::ServerConfig>(constants::CONFIG_SERVER)
		.expect("La configuration serveur.");

	let client =
		network::Client::connect((cfg.ip, cfg.tcp_port.into()), |client| {
			AppClient { inner: client }
		})
		.await
		.expect("La connexion au serveur.");

	client.binary(b"hello world".to_vec()).await;
	while let Some(Ok(line)) = io::stdin().lock().lines().next() {
		client.text(line).await; // <- le serveur reçoit sous forme de données binaires.
	}
}
