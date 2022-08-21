/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::sync::{atomic::AtomicBool, Arc};

// --------- //
// Structure //
// --------- //

/// Un client est tout ce qui se connecte à un serveur qui n'est pas un autre
/// serveur. Il existe deux types de clients qui ont chacun un objectif
/// différent.
#[derive(Debug)]
#[derive(Clone)]
pub struct Client {
	pub label: Option<String>,
	pub ty: Option<ClientType>,
	pub registered: Arc<AtomicBool>,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum ClientType {
	/// Les clients utilisateurs sont généralement des programmes fournissant
	/// une interface textuelle qui est utilisée pour communiquer de manière
	/// interactive via IRC. Ce type particulier de clients est souvent appelé
	/// "utilisateurs".
	User,

	/// Contrairement aux utilisateurs, les clients de service ne sont pas
	/// destinés à être utilisés manuellement ni à discuter. Ils ont un accès
	/// plus limité aux fonctions de discussion du protocole, tout en ayant
	/// éventuellement accès à des données plus privées provenant des serveurs.
	///
	/// Les services sont généralement des automates utilisés pour fournir un
	/// certain type de service (pas nécessairement lié à l'IRC lui-même) aux
	/// utilisateurs. Un exemple est un service collectant des statistiques sur
	/// l'origine des utilisateurs connectés sur le réseau IRC.
	Service,
}

// -------------- //
// Implémentation //
// -------------- //

impl Client {
	pub(crate) fn new() -> Self {
		Self {
			label: Default::default(),
			ty: Default::default(),
			registered: Default::default(),
		}
	}
}
