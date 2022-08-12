use std::{
	fs::File,
	io::{self, Read},
	str::FromStr,
};

// --------- //
// Structure //
// --------- //

#[derive(Debug)]
pub(crate) struct EnvParser(String, String);

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
pub(crate) enum EnvParseError {
	InvalidKeyValue,
}

// -------------- //
// Implémentation //
// -------------- //

impl EnvParser {
	/// Analyse un fichier d'environnement.
	pub fn file(
		filename: impl AsRef<std::path::Path>,
	) -> Result<(), io::Error> {
		let mut file = File::open(filename)?;

		let mut content = String::new();
		file.read_to_string(&mut content)?;

		let parsed_file = Self::parse(&content);
		parsed_file.for_each(|env| env.set());

		Ok(())
	}

	/// Analyse une chaîne de caractères.
	fn parse(input: &str) -> impl Iterator<Item = EnvParser> + '_ {
		input.lines().filter_map(|line| line.parse().ok())
	}
}

impl EnvParser {
	/// Définie une variable d'environnement.
	fn set(&self) {
		let EnvParser(key, value) = self;
		std::env::set_var(key, value);
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl FromStr for EnvParser {
	type Err = EnvParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		s.split_once('=')
			.map(|(name, value)| Self(name.to_owned(), value.to_owned()))
			.ok_or(Self::Err::InvalidKeyValue)
	}
}
