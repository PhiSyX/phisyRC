/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

#![allow(dead_code)]

// Cet exemple montre comment utiliser le second argument de la fonction main.

// ---------------- //
// Premier argument // -> CLI
// ---------------- //

#[derive(Debug)]
struct Cli {
	command: &'static str,
}

impl Cli {
	fn arguments() -> Self {
		Self { command: "test" }
	}
}

// --------------- //
// Second argument // -> Env
// --------------- //

#[derive(Debug)]
struct Env {
	debug_filter: &'static str,
}

#[derive(Debug)]
enum EnvError {}

impl Env {
	// NOTE(phisyx): cette fonction est obligatoire, car la macro l'appelle.
	//
	// On s'imagine ici récupérer les variables d'environnement à partir d'un
	// nom de fichier (passé en argument).
	//
	// filename:
	//     - en debug mode, la valeur est à ".env.local"
	//     - en release mode, la valeur est à ".env"
	//
	// Le retour de la fonction DOIT être un `Result<T, E>` où T et E PEUVENT
	// être de n'importe quel type.
	fn setup(filename: &str) -> Result<Self, EnvError> {
		println!("le nom du fichier à charger...: {filename:?}");
		Ok(Self { debug_filter: "*" })
	}
}

#[phisyrc_macro::setup]
fn main(cli_args: Cli, env_args: Env) {
	println!("{cli_args:?} et {env_args:?}");
}
