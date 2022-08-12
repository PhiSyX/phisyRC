/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use env::EnvError;

use super::TypeGuiError;

pub type AppResult<T> = Result<T, AppError>;

// ------------------------ //
// Erreurs de l'application //
// ------------------------ //

#[derive(Debug)]
pub enum AppError {
	IO(std::io::Error),

	Env(EnvError),

	Gui(TypeGuiError),
}

// -------------- //
// Implémentation // -> Gestion des erreurs (`?`)
// -------------- //

impl From<std::io::Error> for AppError {
	fn from(err: std::io::Error) -> Self {
		Self::IO(err)
	}
}

impl From<EnvError> for AppError {
	fn from(err: EnvError) -> Self {
		Self::Env(err)
	}
}

impl From<TypeGuiError> for AppError {
	fn from(err: TypeGuiError) -> Self {
		Self::Gui(err)
	}
}
