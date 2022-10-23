/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{
	fs::File,
	io::{Read, Write},
};

use helpers::string::Slugify;

const ROOT_README_FILE: &str = "./README.md";
const ROOT_DOCS_DIR: &str = "docs/";

const INPUT: &str = include_str!("./TEMPLATE_README.md");

const SHOULD_CREATE_NEW_FILE: &str =
	"Devait créer un nouveau fichier README.md";

const SHOULD_BE_ABLE_TO_WRITE: &str =
	"Devrait pouvoir écrire le contenu dans le nouveau fichier README.md";

const SHOULD_BE_ABLE_TO_FILL_BUFFER: &str =
	"Devrait pouvoir remplir le tampon temporaire avec le contenu du fichier.";

fn main() {
	let output: String = read_input(INPUT);

	let output = generate_toc(&output);

	let mut new_file =
		File::create(ROOT_README_FILE).expect(SHOULD_CREATE_NEW_FILE);

	new_file
		.write_all(output.as_bytes())
		.expect(SHOULD_BE_ABLE_TO_WRITE);

	println!("Le fichier {ROOT_README_FILE} a été généré.");
}

fn read_input(input: &str) -> String {
	let mut output = input
		.lines()
		.map(include_file)
		.map(relative_file)
		.collect::<Vec<String>>()
		.join(constants::CRLF);
	output.push_str(constants::CRLF);
	output
}

fn include_file(line: &str) -> String {
	if !(line.starts_with("#include <") && line.ends_with('>')) {
		return line.to_owned();
	}

	// e.g: "#include<docs/application.md>" vers "docs/application.md"
	let relative_filename = line.replace("#include <", "").replace('>', "");

	let maybe_file = File::open(&relative_filename);
	if maybe_file.is_err() {
		eprintln!("Le fichier « {} » ne peut pas être lu.", relative_filename);
		return line.to_owned();
	}

	let mut file = maybe_file.unwrap();

	let mut temporary_buffer = String::new();

	file.read_to_string(&mut temporary_buffer)
		.expect(SHOULD_BE_ABLE_TO_FILL_BUFFER);

	read_input(&temporary_buffer)
}

fn relative_file(line: String) -> String {
	line.replace("(./", &format!("({ROOT_DOCS_DIR}"))
}

fn generate_toc(input: &str) -> String {
	let toc = input
		.lines()
		.skip(1)
		.filter(|line| line.starts_with('#'))
		.map(|line| {
			let title = line.trim_start_matches('#').trim();
			let slug = title.slugify();
			let link = format!("[{title}](#{slug})");
			let count = line.match_indices('#').count();
			match count {
				| 1 => line.replace('#', "-"),
				| n => line.replacen('#', "\t", n - 1).replace('#', "-"),
			}
			.replacen(title, &link, 1)
		})
		.collect::<Vec<_>>()
		.join(constants::CRLF);

	if !toc.is_empty() {
		let mut title_toc = String::from("# Table des matières");
		title_toc.push_str(constants::CRLF);
		input.replace("[TOC]", &format!("{title_toc}{toc}"))
	} else {
		input.replace("[TOC]", "")
	}
}
