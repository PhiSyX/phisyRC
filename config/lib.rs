/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

mod common;
mod database;
mod export;
mod server;

use core::fmt;
use std::{
	fs, io,
	path::{Path, PathBuf},
};

use constants::DS;
use terminal::io::{confirm, Prompt};

pub use self::export::*;

/// Répertoire de configuration.
///
/// NOTE(phisyx): en mode debug, le répertoire se situe à la racine du projet:
/// `./.phisyrc`.
/// NOTE(phisyx): en mode release, le répertoire va dépendre de l'OS. La valeur
/// que PEUT prendre ce répertoire est celle de la fonction de retour
/// [dirs::config_dir].
pub fn config_dir() -> PathBuf {
	if cfg!(debug_assertions) {
		let mut tmp = PathBuf::new();
		tmp.push(format!(".{DS}.phisyrc{DS}"));
		tmp
	} else {
		dirs::config_dir()
			.map(|mut cfg| {
				cfg.push(format!("phisyrc{DS}"));
				cfg
			})
			.expect("devrait retourner le chemin du fichier de configuration")
	}
}

/// Charge le fichier de configuration passé par argument et le dé-sérialise en
/// son type [<T>] (passé par générique).
pub fn load<T>(path: impl AsRef<Path>) -> Result<T, io::Error>
where
	T: serde::de::DeserializeOwned,
	T: serde::ser::Serialize,
	T: Default + core::fmt::Debug,
{
	let cfg_dir = config_dir();

	if !cfg_dir.exists() {
		fs::create_dir(&cfg_dir)?;
		logger::info!(
			"Création du répertoire de configuration: '{}'",
			cfg_dir.display()
		);
	}

	let mut cfg_path = cfg_dir;
	cfg_path.push(path);

	if !cfg_path.exists() {
		let s = toml::to_string(&T::default())
			.expect("devrait pouvoir sérialiser la structure");
		fs::write(&cfg_path, &s)?;
		logger::info!(
			"Création du fichier de configuration '{}' avec les valeurs par \
			défaut de la structure '{}'",
			cfg_path.display(),
			core::any::type_name::<T>()
		);
	}

	let content = fs::read_to_string(&cfg_path)?;
	let obj = toml::from_str(&content)?;
	logger::trace!(
		"Fichier de configuration '{}' dé-sérialisé en type '{}' avec succès.",
		cfg_path.display(),
		core::any::type_name::<T>()
	);
	logger::debug!("{:?}", &obj);
	Ok(obj)
}

/// Charge le fichier de configuration passé par argument et le dé-sérialise en
/// son type [<T>] (passé par générique) ou demande à l'utilisateur de générer
/// une nouveau fichier de configuration de manière interactive ou utiliser
/// la configuration par défaut.
pub fn load_or_prompt<T>(
	path: impl AsRef<Path>,
	title: impl fmt::Display,
) -> Result<T, io::Error>
where
	T: serde::de::DeserializeOwned,
	T: serde::ser::Serialize,
	T: Default + Prompt + core::fmt::Debug,
{
	let cfg_dir = config_dir();

	if !cfg_dir.exists() {
		fs::create_dir(&cfg_dir)?;
		logger::info!(
			"Création du répertoire de configuration: '{}'",
			cfg_dir.display()
		);
	}

	let mut cfg_path = cfg_dir;
	cfg_path.push(path);

	if !cfg_path.exists() {
		println!(
			"Configuration '{}' manquante... {title}",
			cfg_path.display()
		);
		println!();

		let s = if confirm("Voulez-vous utiliser la configuration par défaut?")
		{
			toml::to_string(&T::default())
		} else if confirm(
			"Voulez-vous créer la configuration de manière interactive?",
		) {
			println!();
			println!("Configuration interactive...");
			println!();
			toml::to_string(&T::prompt())
		} else {
			return Err(io::Error::new(
				io::ErrorKind::Interrupted,
				"la configuration est manquante...",
			));
		}
		.expect("devrait pouvoir sérialiser la structure");

		fs::write(&cfg_path, &s)?;
		logger::info!(
			"Création du fichier de configuration '{}' avec les valeurs par \
			défaut de la structure '{}'",
			cfg_path.display(),
			core::any::type_name::<T>()
		);
	}

	let content = fs::read_to_string(&cfg_path)?;
	let obj = toml::from_str(&content)?;
	logger::trace!(
		"Fichier de configuration '{}' dé-sérialisé en type '{}' avec succès.",
		cfg_path.display(),
		core::any::type_name::<T>()
	);
	logger::debug!("{:?}", &obj);
	Ok(obj)
}

pub fn delete(path: impl AsRef<Path>) -> io::Result<()> {
	let mut cfg = config_dir();
	cfg.push(path);

	if cfg.exists() {
		logger::info!(
			"Suppression du fichier de configuration '{}'.",
			cfg.display(),
		);
		return fs::remove_file(cfg);
	}

	Ok(())
}
