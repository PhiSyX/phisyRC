/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use lang::lexer::ParseState;

// ----------- //
// Énumération //
// ----------- //

#[derive(Default)]
#[derive(PartialEq, Eq)]
pub(super) enum ParseCommandState {
	/// L'état initiale de l'analyse de commande.
	#[default]
	Initial,

	/// Analyse d'une commande numérique.
	///
	/// Les clients ne PEUVENT PAS envoyer ce genre de commande.
	Numeric {
		/// Le compteur ne DOIT pas dépasser le chiffre 3. Autrement il s'agit
		/// d'une command incorrect.
		counter: u8,
	},

	/// Analyse d'une commande normale ou textuelle.
	Text,
}

/// Les états sont organisés par ordre par lesquelles l'analyseur doit passer.
#[derive(Default)]
pub(super) enum ParseCommandParametersStepState {
	/// État initiale de l'analyse de paramètres.
	#[default]
	Initial,

	/// Analyse des paramètres. Ces paramètres séparés par des espaces.
	FirstStep,

	PrepareSecondStep,
	/// Analyse des restes des paramètres. Ces paramètres peuvent contenir
	/// des espaces.
	SecondStep,
	AfterColon,

	Finish,
}

// -------------- //
// Implémentation //
// -------------- //

impl ParseCommandState {
	pub(super) fn increment_counter(&mut self) {
		if let ParseCommandState::Numeric { counter } = self {
			*counter += 1;
		}
	}
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl ParseState for ParseCommandState {
	fn switch(&mut self, new_state: Self) {
		*self = new_state;
	}
}

impl ParseState for ParseCommandParametersStepState {
	fn switch(&mut self, new_state: Self) {
		*self = new_state;
	}
}
