/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// --------- //
// Constante //
// --------- //

/// Séparateur de répertoire.
///
/// DS = DIRECTORY_SEPARATOR
/// Suivant l'OS, le caractère peut être différent.
pub const DS: &str = if cfg!(windows) { "\\" } else { "/" };

/// Caractère de fin de ligne.
pub const CRLF: &str = if cfg!(windows) { "\r\n" } else { "\n" };

// ------- //
// Serveur //
// ------- //

/// Nom du fichier de configuration du serveur.
pub const CONFIG_FILENAME: &str = "server.toml";

pub const DEFAULT_SERVER_NAME: &str = "localhost";
pub const DEFAULT_SERVER_IP: &str = "127.0.0.1";
pub const DEFAULT_SERVER_PORT: u16 = 6667;
pub const DEFAULT_SERVER_PASSWORD: Option<String> = None;

// --------------- //
// Base de données //
// --------------- //

/// Nom du fichier de configuration de la base de données.
pub const CONFIG_DATABASE: &str = "database.toml";

pub const DEFAULT_DATABASE_IP: &str = "127.0.0.1";
pub const DEFAULT_DATABASE_PORT: u16 = 5432;
pub const DEFAULT_DATABASE_PASSWORD: &str = "root";
pub const DEFAULT_DATABASE_USERNAME: &str = "root";
pub const DEFAULT_DATABASE_NAME: &str = "phisyrc";
