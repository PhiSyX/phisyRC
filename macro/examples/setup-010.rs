/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */

#![allow(dead_code)]

// Cet exemple montre comment configurer le logger avec stdout.

// Il y a deux types de logger:
//     - Normal (stdout)
//     - Avec une interface (tui)

#[phisyrc_macro::setup(logger = "stdout")]
fn main() {
	logger::info!("Hello World");
}
