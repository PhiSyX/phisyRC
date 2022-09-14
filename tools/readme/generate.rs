use std::{
	fs::File,
	io::{Read, Write},
};

fn main() {
	let file_content = include_str!("./TEMPLATE_README.md");

	let content: String = file_content
		.lines()
		.map(|line| {
			if line.starts_with("#include <") && line.ends_with('>') {
				let filename = line.replace("#include <", "").replace('>', "");
				let mut file = File::open(&filename)
					.unwrap_or_else(|err| panic!("{err} -- > {filename} < "));
				let mut buffer_content = String::new();
				file.read_to_string(&mut buffer_content)
					.unwrap_or_else(|err| panic!("Devrait pouvoir remplir le tampon `buffer_content` du contenu du fichier '{filename}': {err}."));
				buffer_content
			} else {
				line.to_owned()
			}
		})
		.collect::<Vec<String>>()
		.join("\n");

	let mut new_file = File::create("./README.md")
		.expect("Devait créer un nouveau fichier README.md");

	new_file.write_all(content.as_bytes()).expect(
		"Devrait pouvoir écrire le contenu dans le nouveau fichier README.md",
	);

	println!("Le fichier README.md a été généré.");
}
