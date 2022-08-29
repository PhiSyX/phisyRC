/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::command;

command! { impl IncomingUnregisteredCommand
	<- PASS
	<- NICK
	<- USER
	<- SERVER
	<- QUIT
}

command! { impl IrcClientCommand
	#[doc = include_str!("../docs/CONNECTION_REGISTRATION.md")]
	/// >
	/// > La commande `PASS` est utilisée pour définir un "mot de passe de
	/// > connexion". Le mot de passe optionnel PEUT et DOIT être défini
	/// > avant toute tentative d'enregistrement de la connexion.
	/// > Actuellement, cela nécessite que l'utilisateur envoie une
	/// > commande `PASS` avant d'envoyer la combinaison `NICK`/`USER`.
	<- PASS { password }
		| ERR_NEEDMOREPARAMS | ERR_ALREADYREGISTRED

	#[doc = include_str!("../docs/CONNECTION_REGISTRATION.md")]
	/// >
	/// > La commande `NICK` est utilisée pour donner un pseudonyme à
	/// > l'utilisateur ou changer le pseudonyme existant.
	<- NICK { nickname }
		| ERR_NONICKNAMEGIVEN | ERR_ERRONEUSNICKNAME
		| ERR_NICKNAMEINUSE   | ERR_NICKCOLLISION
		| ERR_UNAVAILRESOURCE | ERR_RESTRICTED

	#[doc = include_str!("../docs/CONNECTION_REGISTRATION.md")]
	/// >
	/// > La commande `USER` est utilisée au début de la connexion pour
	/// > spécifier le nom d'utilisateur, le nom d'hôte et le nom réel d'un
	/// > nouvel utilisateur.
	/// >
	/// > Le paramètre `<mode>` doit être un numérique, et peut être
	/// > utilisé pour définir automatiquement les modes utilisateur lors
	/// > de l'enregistrement avec le serveur.  Ce paramètre est un
	/// > masque de bits, avec seulement 2 bits ayant une signification :
	/// > si le bit 2 est défini, le mode utilisateur 'w' sera défini et
	/// > si le bit 3 est défini, le mode utilisateur 'i' sera défini.
	/// > (Voir la section 3.1.5 "Modes utilisateur").
	/// >
	/// > Le `<realname>` peut contenir des caractères d'espacement.
	<- USER { user, mode, _unused, realname }
		| ERR_NEEDMOREPARAMS | ERR_ALREADYREGISTRED
}
