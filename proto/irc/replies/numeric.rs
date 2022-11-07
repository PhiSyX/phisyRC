/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use crate::numeric;

numeric! { impl Numeric
	// -------- //
	// Réponses //
	// -------- //

	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_001-004.md")]
	| 001 <-> RPL_WELCOME { nick, user, host }
		=> ":Welcome to the Internet Relay Network {nick}!{user}@{host}"

	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_001-004.md")]
	| 002 <-> RPL_YOURHOST { servername, ver }
	=> ":Your host is {servername}, running version {ver}"

	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_001-004.md")]
	| 003 <-> RPL_CREATED { date }
		=> ":This server was created {date}"

	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_001-004.md")]
	| 004 <-> RPL_MYINFO {
		servername, version,
		available_user_modes, available_channel_modes
	} => ":{servername} {version} {available_user_modes} {available_channel_modes}"

	/// `005` est maintenant utilisé pour `RPL_ISUPPORT`.
	///
	/// Comme le nombre maximum de paramètres d'un message pour une réponse est
	/// de `15`, le nombre maximum de tokens `RPL_ISUPPORT` qui peuvent être
	/// annoncés est de `13`. Pour contrer cela, un serveur peut émettre
	/// plusieurs tokens numériques `RPL_ISUPPORT`. Un serveur DOIT émettre au
	/// moins un token numérique `RPL_ISUPPORT` après l'enregistrement du
	/// client. Il doit être émis avant que d'autres commandes du client ne
	/// soient traitées.
	///
	/// Lorsque les clients envoient une commande `VERSION` à un serveur
	/// externe (c'est-à-dire qui n'est pas celui auquel ils sont
	/// actuellement connectés), ils reçoivent les informations appropriées de
	/// ce serveur. Les tokens `ISUPPORT` de ce serveur externe sont envoyés au
	/// client en utilisant le code numérique `105` (`RPL_REMOTEISUPPORT`) au
	/// lieu de `005`, afin de s'assurer que les clients ne traitent pas et ne
	/// commencent pas à utiliser ces tokens envoyés par un serveur externe.
	/// Le format du message `105` est exactement le même que celui du message
	/// `RPL_ISUPPORT` - le code numérique lui-même est la seule différence.
	///
	/// Un token est de la forme `PARAMETER`, `PARAMETER=VALEUR` ou
	/// `-PARAMETER`. Les serveurs doivent envoyer le paramètre sous forme de
	/// texte en majuscules.
	///
	/// Les tokens de la forme `PARAMETER` ou `PARAMETER=VALUE` sont utilisés
	/// pour annoncer des fonctionnalités ou des informations aux clients.
	/// Un paramètre peut avoir une valeur par défaut et la valeur peut être
	/// vide lorsqu'elle est envoyée par les serveurs. Sauf indication
	/// contraire, lorsqu'un paramètre contient une valeur, celle-ci doit être
	/// considérée comme sensible à la casse. La valeur peut contenir plusieurs
	/// champs, si c'est le cas, les champs doivent être délimités par une
	/// virgule (",", 0x2C).
	///
	/// Si la valeur d'un paramètre change, le serveur devrait annoncer à
	/// nouveau le paramètre avec la nouvelle valeur dans une réponse
	/// `RPL_ISUPPORT`. Un exemple de ceci est un client qui devient un
	/// opérateur IRC et dont le `CHANLIMIT` change.
	///
	/// Les tokens de la forme `-PARAMETER` sont utilisés pour annuler un
	/// paramètre précédemment spécifié. Si le client reçoit un token de ce
	/// type, il doit considérer que le paramètre est supprimé et revenir au
	/// comportement qui se produirait si le paramètre n'était pas spécifié. Le
	/// client doit agir comme si le paramètre ne lui était plus annoncé. Ces
	/// tokens sont destinés à permettre aux serveurs de modifier leurs
	/// fonctionnalités sans déconnecter les clients. Les tokens de cette forme
	/// ne doivent pas contenir de champ de valeur.
	///
	/// Le serveur peut annuler des paramètres qui n'ont pas été annoncés
	/// précédemment ; dans ce cas, le client doit ignorer le token.
	///
	/// Une seule réponse `RPL_ISUPPORT` ne doit pas contenir le même paramètre
	/// plusieurs fois ni annoncer et nier le même paramètre. Cependant, le
	/// serveur est libre d'annoncer ou de nier le même paramètre dans des
	/// réponses séparées.
	///
	/// #
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_ISUPPORT.md")]
	| 005 <-> RPL_ISUPPORT { supports }
		=> "{supports} :are supported by this server"

	// ------- //
	// Erreurs //
	// ------- //

	/// Renvoyé à un client enregistré pour indiquer que la commande envoyée
	/// est inconnue du serveur.
	| 421 <-> ERR_UNKNOWNCOMMAND { command }
		=> "{command} :Unknown command"

	/// Renvoyé quand un paramètre de surnom (`nickname`) attendu pour une
	/// commande n'est pas trouvé.
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

	/// Renvoyé par le serveur à tout client qui tente de modifier une partie
	/// des données enregistrées (comme le mot de passe (PASS) ou les données
	/// de l'utilisateur à partir du deuxième message (USER)).
	| 462 <-> ERR_ALREADYREGISTRED
		=> ":Unauthorized command (already registered)"
}
