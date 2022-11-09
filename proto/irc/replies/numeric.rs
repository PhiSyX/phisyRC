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

	/// Envoyé au client pour le rediriger vers un autre serveur.
	///
	/// Le texte `info` varie selon le logiciel du serveur est les raisons de la
	/// redirection. Comme ce `<numeric>` ne précise pas s'il faille activer le
	/// SSL ou non, et qu'il n'est pas interprété par tous les clients, il est
	/// recommandé de ne pas l'utiliser.
	///
	/// Ce `<numeric>` est également connu sous le nom de `RPL_REDIR` par
	/// certains logiciels.
	| 010 <-> RPL_BOUNCE { servername, port, info }
		=> "{servername} {port}: {info}"

	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_200..210-261-262.md")]
	| 200 <-> RPL_TRACELINK {
		version_debuglevel, destination,
		next_server, protocol_version,
		link_uptime_secs, backstream_sendq, upstream_sendq
	} => "Link {version_debuglevel} {destination} {next_server} V{protocol_version} {link_uptime_secs} {backstream_sendq} {upstream_sendq}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_200..210-261-262.md")]
	| 201 <-> RPL_TRACECONNECTING { class, server }
		=> "Try. {class} {server}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_200..210-261-262.md")]
	| 202 <-> RPL_TRACEHANDSHAKE { class, server }
		=> "H.S. {class} {server}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_200..210-261-262.md")]
	| 203 <-> RPL_TRACEUNKNOWN { class, client_ip_address }
		=> "???? {class} {client_ip_address}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_200..210-261-262.md")]
	| 204 <-> RPL_TRACEOPERATOR { class, nick }
		=> "Oper {class} {nick}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_200..210-261-262.md")]
	| 205 <-> RPL_TRACEUSER { class, nick }
		=> "User {class} {nick}"
	/// **RFC 2812:** -> "Serv <class> <int>S <int>C <server> <nick!user|*!*>@<host|server> V<protocol version>"
	///
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_200..210-261-262.md")]
	| 206 <-> RPL_TRACESERVER {
		class,
		int_s, int_c,
		server,
		/// ( `<nick>` "!" `<user>` ) / ( "\*!\*" )
		client_nick_user,
		/// ( `<host>` / `<server>` )
		host_server
	} => "Serv {class} {int_s}S {int_c}C {server} {client_nick_user}@{host_server}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_200..210-261-262.md")]
	| 207 <-> RPL_TRACESERVICE { class, name, trace_type, active_trace_type }
		=> "Service {class} {name} {trace_type} {active_trace_type}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_200..210-261-262.md")]
	| 209 <-> RPL_TRACECLASS { class, count}
		=> "Class {class} {count}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_200..210-261-262.md")]
	/// Inutilisé.
	| 210 <-> RPL_TRACERECONNECT => ""
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_200..210-261-262.md")]
	| 208 <-> RPL_TRACENEWTYPE { newtype, client_name }
		=> "{newtype} 0 {client_name}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_200..210-261-262.md")]
	| 261 <-> RPL_TRACELOG { logfile, debuglevel }
		=> "File {logfile} {debuglevel}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_200..210-261-262.md")]
	| 262 <-> RPL_TRACEEND { servername, version_debuglevel }
		=> "{servername} {version_debuglevel} :End of TRACE"

	/// Présente des statistiques sur une connexion. `<linkname>` identifie la
	/// connexion particulière, `<sendq>` est la quantité de données en file
	/// d'attente et en attente d'être envoyées `<sent_messages>` le nombre de
	/// messages envoyés, et `<sent_kbytes>` la quantité de données envoyées, en
	/// kbytes. `<received_messages>` et `<received_kbytes>` sont l'équivalent
	/// de `<sent_messages>` et `<sent_kbytes>` pour les données reçues,
	/// respectivement.  `<time_open>` indique depuis combien de temps la
	/// connexion a été ouverte, en secondes.
	| 211 <-> RPL_STATSLINKINFO {
		linkname,
		sendq, sent_messages, sent_kbytes,
		received_messages, received_kbytes,
		time_open
	}
	=> "{linkname} {sendq} {sent_messages} {sent_kbytes} {received_messages} {received_kbytes} {time_open}"
	/// Rapporte des statistiques sur l'utilisation des commandes
	| 212 <-> RPL_STATSCOMMANDS { command, count, byte_count, remote_count }
		=> "{command} {count} {byte_count} {remote_count}"
	| 213 <-> RPL_STATSCLINE { host, name, port, class }
		=> "C {host} * {name} {port} {class}"
	| 214 <-> RPL_STATSNLINE { host, name, port, class }
		=> "N {host} * {name} {port} {class}"
	| 215 <-> RPL_STATSILINE { host, port, class }
		=> "I {host} * {host} {port} {class}"
	| 216 <-> RPL_STATSKLINE { host, username, port, class}
		=> "K {host} * {username} {port} {class}"
	| 218 <-> RPL_STATSYLINE {
		class,
		ping_frequency, connect_frequency,
		max_sendq
	} => "Y {class} {ping_frequency} {connect_frequency} {max_sendq}"
	| 219 <-> RPL_ENDOFSTATS { stats_letter }
		=> "{stats_letter} :End of STATS report"
	| 241 <-> RPL_STATSLLINE { hostmask, servername, maxdepth }
		=> "L {hostmask} * {servername} {maxdepth}"
	/// **RFC 2812:** -> ":Server Up %d days %d:%02d:%02d"
	/// Rapporte le temps de fonctionnement du serveur.
	| 242 <-> RPL_STATSUPTIME { days, hours, minutes, seconds }
		=> ":Server Up {days} days {hours}:{minutes}:{seconds}"
	/// Indique les hôtes autorisés à partir desquels les utilisateurs peuvent
	/// devenir des opérateurs IRC.
	| 243 <-> RPL_STATSOLINE { hostmask, name }
		=> "O {hostmask} * {name}"
	| 244 <-> RPL_STATSHLINE { hostmask, servername }
		=> "H {hostmask} * {servername}"

	/// Pour répondre à une requête concernant le propre mode d'un client,
	/// `RPL_UMODEIS` est renvoyé.
	| 221 <-> RPL_UMODEIS { user_mode }
		=> "{user_mode}"

	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_234-235.md")]
	| 234 <-> RPL_SERVLIST { name, server, mask, serv_type, hopcount, info }
		=> "{name} {server} {mask} {serv_type} {hopcount} {info}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_234-235.md")]
	| 235 <-> RPL_SERVLISTEND { mask, serv_type }
		=> "{mask} {serv_type} :End of service listing"

	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_251..255.md")]
	| 251 <-> RPL_LUSERCLIENT { total_users, total_services, total_servers }
		=> ":There are {total_users} users and {total_services} services on {total_servers} servers"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_251..255.md")]
	| 252 <-> RPL_LUSEROP { total_online_operators }
		=> "{total_online_operators} :operator(s) online"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_251..255.md")]
	| 253 <-> RPL_LUSERUNKNOWN { total_unknown_connections }
		=> "{total_unknown_connections} :unknown connection(s)"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_251..255.md")]
	| 254 <-> RPL_LUSERCHANNELS { total_channels }
		=> "{total_channels} :channels formed"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_251..255.md")]
	| 255 <-> RPL_LUSERME { total_clients, total_servers }
		=> ":I have {total_clients} clients and {total_servers} servers"

	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_256..259.md")]
	| 256 <-> RPL_ADMINME { server }
		=> "{server} :Administrative info"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_256..259.md")]
	| 257 <-> RPL_ADMINLOC1 { info }
		=> ":{info}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_256..259.md")]
	| 258 <-> RPL_ADMINLOC2 { info }
		=> ":{info}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_256..259.md")]
	| 259 <-> RPL_ADMINEMAIL { info }
		=> ":{info}"

	/// Lorsqu'un serveur abandonne une commande sans la traiter, il doit
	/// utiliser la réponse `RPL_TRYAGAIN` pour informer le client d'origine.
	| 263 <-> RPL_TRYAGAIN { command }
		=> "{command} :Please wait a while and try again."

	/// Envoyé en réponse à la commande `LUSERS`.
	/// `<current_local_users>` et `<max_local_users>` sont des entiers non
	/// négatifs et représentent respectivement le nombre de clients actuels et
	/// le nombre maximal de clients qui ont été connectés directement à ce
	/// serveur en une seule fois.
	///
	/// Les deux paramètres facultatifs devraient être fournis pour permettre
	/// aux clients de mieux extraire ces nombres.
	| 265 <-> RPL_LOCALUSERS { current_local_users, max_local_users }
		=> ":Current local users: {current_local_users}, max {max_local_users}"
	/// Envoyé en réponse à la commande `LUSERS`.
	/// `<current_global_users>` et `<max_global_users>` sont des entiers non
	/// négatifs. `<current_global_users>` représente le nombre de clients
	/// actuellement connectés à ce serveur, globalement (directement et par le
	/// biais d'autres liens de serveurs). `<max_global_users>` représente le
	/// nombre maximum de clients qui ont été connectés à ce serveur en une
	/// seule fois, globalement.
	///
	/// Les deux paramètres facultatifs devraient être fournis pour permettre
	/// aux clients de mieux extraire ces nombres.
	| 266 <-> RPL_GLOBALUSERS { current_global_users, max_global_users }
		=> ":Current global users: {current_global_users}, max {max_global_users}"

	/// Code numérique fictif. Non utilisé.
	| 300 <-> RPL_NONE => ""

	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_301-305-306.md")]
	| 301 <-> RPL_AWAY { nick, away_message }
		=> "{nick} :{away_message}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_301-305-306.md")]
	| 305 <-> RPL_UNAWAY
		=> ":You are no longer marked as being away"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_301-305-306.md")]
	| 306 <-> RPL_NOWAWAY
		=> ":You have been marked as being away"

	/// **RFC 1459:** -> "`:[<reply>{<space><reply>}]`"
	///
	/// **RFC 2812:** -> "`:*1<reply> *( " " <reply> )`"
	///
	/// Format de réponse utilisé par `USERHOST` pour lister les réponses à la
	/// liste des requêtes. La chaîne de caractères de la réponse est composée
	/// comme suit:
	///
	/// `reply = nickname [ "*" ] "=" ( "+" / "-" ) hostname`
	///
	/// Le caractère '`*`' indique si le client s'est enregistré en tant
	/// qu'opérateur. Les caractères '`-`' et '`+`' indiquent si le client a
	/// défini respectivement un message `AWAY` ou non.
	| 302 <-> RPL_USERHOST {
		/// `reply = nickname [ "*" ] "=" ( "+" / "-" ) hostname`
		replies
	} => ":{replies}"

	/// **RFC 1459:** -> "`:[<nick> {<space><nick>}]`"
	///
	/// **RFC 2812:** -> "`:*1<nick> *( " " <nick> )`"
	///
	/// Format de réponse utilisé par `ISON` pour lister les réponses à la liste
	/// de requêtes.
	///
	/// Liste les pseudonymes qui sont présents sur le réseau.
	| 303 <-> RPL_ISON { nicks }
		=> ":{nicks}"


	/// Envoyé en réponse à la commande `WHOIS`, ce numérique indique
	/// l'empreinte du certificat SSL/TLS utilisé par le client avec le surnom
	/// (`nickname`). Les clients doivent recevoir ce numéro uniquement s'ils
	/// utilisent la commande `WHOIS` sur eux-mêmes ou s'ils sont un opérateur.
	| 276 <-> RPL_WHOISCERTFP { nickname, fingerprint }
		=> "{nickname} :has client certificate fingerprint {fingerprint}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_311..313-317..319.md")]
	| 311 <-> RPL_WHOISUSER { nick, user, host, realname }
		=> "{nick} {user} {host} * :{realname}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_311..313-317..319.md")]
	| 312 <-> RPL_WHOISSERVER { nick, server, serverinfo }
		=> "{nick} {server} :{serverinfo}"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_311..313-317..319.md")]
	| 313 <-> RPL_WHOISOPERATOR { nick }
		=> "{nick} :is an IRC operator"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_311..313-317..319.md")]
	| 317 <-> RPL_WHOISIDLE { nick, integer }
		=> "{nick} {integer} :seconds idle"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_311..313-317..319.md")]
	| 318 <-> RPL_ENDOFWHOIS { nick }
		=> "{nick} :End of /WHOIS list"
	#[doc = include_str!("../../../docs/protocols/irc/replies/RPL_311..313-317..319.md")]
	| 319 <-> RPL_WHOISCHANNELS { nick, channels_with_status }
		=> "{nick} :{channels_with_status}"

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
