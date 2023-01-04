/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

#![allow(dead_code)]

// Cet exemple montre comment utiliser le premier argument de la fonction main.

// ---------------- //
// Premier argument // -> CLI
// ---------------- //

#[derive(Debug)]
struct Cli {
	command: &'static str,
}

impl Cli {
	// NOTE(phisyx): cette fonction est obligatoire, car la macro l'appelle.
	//
	// On s'imagine ici récupérer les arguments de CLI.
	fn arguments() -> Self {
		Self { command: "test" }
	}
}

#[phisyrc_macro::setup]
fn main(cli_args: Cli) {
	println!("{cli_args:?}");
}
