/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::numeric;

numeric! { impl IrcCommandNumeric
	// -------- //
	// Réponses //
	// -------- //

	#[doc = include_str!("../docs/001-004.md")]
	| 001 <-> RPL_WELCOME { nick, user, host }
		=> ":Welcome to the Internet Relay Network {nick}!{user}@{host}"

	#[doc = include_str!("../docs/001-004.md")]
	| 002  <->  RPL_YOURHOST { servername, ver }
		=> ":Your host is {servername}, running version {ver}"

	#[doc = include_str!("../docs/001-004.md")]
	| 003  <->  RPL_CREATED { date }
		=> ":This server was created {date}"


	// ------- //
	// Erreurs //
	// ------- //

	/// Renvoyé à un client enregistré pour indiquer que la commande  envoyée
	/// est inconnue du serveur.
	| 421 <-> ERR_UNKNOWNCOMMAND { command }
		=> "{command} :Unknown command"

	/// Renvoyé quand un paramètre de surnom (`nickname`) attendu pour une
	/// commande et n'est pas trouvé.
	| 431 <-> ERR_NONICKNAMEGIVEN
		=> ":No nickname given"

	/// Renvoyé lorsqu'un message `NICK` est traité qui résulte en une tentative
	/// de changement vers un surnom (`nickname`) existant.
	| 433 <-> ERR_NICKNAMEINUSE { nick }
		=> "{nick} :Nickname is already in use"

	/// Renvoyé par le serveur par de nombreuses commandes pour indiquer au
	/// client qu'il n'a pas fourni suffisamment de paramètres.
	| 461 <-> ERR_NEEDMOREPARAMS { command }
		=> "{command} :Not enough parameters"
}
