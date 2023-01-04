/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

// Utilise le contexte d'execution tokio par défaut.

use core::fmt;

use tokio::time;

#[phisyrc_macro::setup]
async fn main() {
	println!("Hello World");

	sleep(1).await;
	sleep(3).await;
	sleep(2).await;
}

async fn sleep(n: impl fmt::Display) {
	time::sleep(time::Duration::from_secs(1)).await;
	println!("Coucou {n}");
}
