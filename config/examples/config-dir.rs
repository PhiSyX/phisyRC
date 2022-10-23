/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

use phisyrc_config::config_dir;
use terminal::crossterm::style::Stylize;

fn main() {
	println!(
		"Le chemin de la configuration du projet se trouve ici: « {} »",
		config_dir().display().to_string().magenta()
	);
	println!(
		"{}: en mode `--release` le chemin varie, évidemment.",
		"NOTE".reverse()
	);
}
