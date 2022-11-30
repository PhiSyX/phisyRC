/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::{path::Path, process::ExitCode};

use include_dir::{include_dir, Dir, File};

const LICENSE_MPL: &str = r#"/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */"#;

const LICENSE_MPL_PUB: &str = r#"/*
 * Any copyright is dedicated to the Public Domain.
 * https://creativecommons.org/publicdomain/zero/1.0/
 */"#;

const APPS_DIR: Dir = include_dir!("apps");
const CONFIG_DIR: Dir = include_dir!("config");
const CONSTANTS_DIR: Dir = include_dir!("constants");
const CRATES_DIR: Dir = include_dir!("crates");
const MACRO_DIR: Dir = include_dir!("macro");
const NPM_DIR: Dir = include_dir!("npm");
const PROTO_DIR: Dir = include_dir!("proto");
const TOOLS_DIR: Dir = include_dir!("tools");

fn main() -> ExitCode {
	let mut files = vec![];

	check_license(&APPS_DIR, &mut files);
	check_license(&CONFIG_DIR, &mut files);
	check_license(&CONSTANTS_DIR, &mut files);
	check_license(&CRATES_DIR, &mut files);
	check_license(&NPM_DIR, &mut files);
	check_license(&MACRO_DIR, &mut files);
	check_license(&PROTO_DIR, &mut files);
	check_license(&TOOLS_DIR, &mut files);

	if files.is_empty() {
		println!("OK.");
		return ExitCode::SUCCESS;
	}

	println!();
	for file in files {
		eprintln!(
			"Le fichier source « {} » ne contient pas l'en-tête de la licence.",
			file.display()
		);
	}

	ExitCode::FAILURE
}

fn check_license(directory: &'static Dir, append_files: &mut Vec<&Path>) {
	for dir in directory.dirs() {
		let mut entries = vec![];
		let rs_entries: Vec<_> = dir.find("**/*.rs").expect("?").collect();
		let js_entries = dir.find("**/*.ts").expect("?");
		let css_entries = dir.find("**/*.scss").expect("?");
		entries.extend(rs_entries);
		entries.extend(js_entries);
		entries.extend(css_entries);
		let entries = entries;
		for entry in entries {
			if let Some(file) = entry.as_file() {
				let path = file.path().to_string_lossy();
				if path.contains("node_modules") || path.contains("wasm.") {
					continue;
				}
				check_file(file, append_files);
			}
		}
	}

	for file in directory.files() {
		if file
			.path()
			.extension()
			.filter(|ext| *ext == "rs" || *ext == "ts" || *ext == "scss")
			.is_none()
		{
			continue;
		}
		check_file(file, append_files);
	}
}

fn check_file(file: &'static File, append_files: &mut Vec<&Path>) {
	if file
		.contents_utf8()
		.filter(|content| {
			content.starts_with(LICENSE_MPL)
				|| content.starts_with(LICENSE_MPL_PUB)
		})
		.is_none()
	{
		append_files.push(file.path());
	}
}
