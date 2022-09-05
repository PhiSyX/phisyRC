# language: fr

Fonctionnalité: Analyse d'un message

	Plan du scénario: gestion des erreurs d'un message utilisateur
		Lorsque on analyse la ligne : <ligne>
		Alors la ligne DOIT être considérée comme étant `<état>`

		Exemples:
			| ligne                                                        | état                                          |
			| "@\r\n"                                                      | invalide(tags): caractère invalide -> \r      |
			| "@=\r\n"                                                     | invalide(tags): clé vide                      |
			| ":\r\n"                                                      | invalide(préfixe): caractère invalide -> \r   |
			| ""                                                           | invalide(commande): erreur d'analyse          |
			| "\n"                                                         | invalide(commande): caractère invalide -> \n  |
			| "\r\n"                                                       | invalide(commande): caractère invalide -> \r  |
			| " \r\n"                                                      | invalide(commande): caractère invalide -> \s  |
			| "01\r\n"                                                     | invalide(commande): code numérique trop court |
			| "001\r\n"                                                    | valide                                        |
			| "0001\r\n"                                                   | invalide(commande): code numérique trop long  |
			| "A\r\n"                                                      | valide                                        |
			| "JOIN\r\n"                                                   | valide                                        |
			| "CAP LS 302\r\n"                                             | valide                                        |
			| "AWAY\r\n"                                                   | valide                                        |
			| "4WAY\r\n"                                                   | invalide(commande): caractère invalide -> W   |
			| "AWAY :test\r\n"                                             | valide                                        |
			| "CUST0M :x\r\n"                                              | valide                                        |
			| ":nick!user@host.org PRIVMSG nick_1 :Hi ?"                   | invalide(commande): ligne non terminée        |
			| ":nick!user@host.org PRIVMSG nick_1 :Hi ?\r\n"               | valide                                        |
			| "@admin :nick!user@host.org PRIVMSG nick_1 :Hi ?\r\n"        | valide                                        |
			| "@admin=false :nick!user@host.org PRIVMSG nick_1 :Hi ?\r\n"  | valide                                        |
			| ": PRIVMSG nick_1 :Hi ?\r\n"                                 | invalide(préfixe): caractère invalide -> \s   |
			| ":!user@host.org PRIVMSG nick_1 :Hi ?\r\n"                   | invalide(préfixe): pseudonyme manquant        |
			| ":nick!user PRIVMSG nick_1 :Hi ?\r\n"                        | invalide(préfixe): nom d'hôte manquant        |
			| ":-irc.local.host PRIVMSG nick_1 :Hi ?\r\n"                  | invalide(préfixe): 1er caractère invalide     |
			| ":xx.x7.xx3.x2- PRIVMSG nick_1 :Hi ?\r\n"                    | invalide(préfixe): dernier caractère invalide |
			| ":xx.x7.xx3.x2 PRIVMSG nick_1 :Hi ?\r\n"                     | valide                                        |
			| "@admin=42 :xx.x7.xx3.x2 PRIVMSG nick_1 :Hi ?\r\n"           | valide                                        |

	Plan du scénario: les métadonnées d'un message
		Lorsque on analyse un message IRC valide : <ligne>
		Alors la présence de métadonnées est "<conditionnelle>"
		Et les métadonnées du message sont `<métadonnées>`

		Exemples:
			| ligne                                                                                | conditionnelle | métadonnées                                                 |
			| "@id=1;name=Mike\sPhiSyX;age=21;admin :nick!user@host.org PRIVMSG nick_1 :Hi ?\r\n"  | vraie          | {"id":"1","name":"Mike PhiSyX","age":"21","admin":"true"}   |
			| ":nick!user@host.org PRIVMSG nick_1 :Hi ?\r\n"                                       | fausse         | {}                                                          |

	Plan du scénario: le préfixe d'un message
		Lorsque on analyse un message IRC valide : <ligne>
		Alors la présence d'un préfixe est "<conditionnelle>"
		Et le préfixe du message est "<préfixe>"

		Exemples:
			| ligne                                          | conditionnelle | préfixe            |
			| ":nick PRIVMSG nick_1 :Hi ?\r\n"               | fausse         |                    |
			| ":nick@host.org PRIVMSG nick_1 :Hi ?\r\n"      | fausse         |                    |
			| ":nick!user@host.org PRIVMSG nick_1 :Hi ?\r\n" | vraie          | nick!user@host.org |
			| "PRIVMSG nick_1 :Hi ?\r\n"                     | fausse         |                    |
			| ":irc.local.host PRIVMSG nick_1 :Hi ?\r\n"     | vraie          | irc.local.host     |
			| ":localhost PRIVMSG nick_1 :Hi ?\r\n"          | vraie          | localhost          |
			| ":127.0.0.1 PRIVMSG nick_1 :Hi ?\r\n"          | vraie          | 127.0.0.1          |

	Plan du scénario: les paramètres d'une commande IRC
		Lorsque on analyse un message IRC valide : <ligne>
		Alors la commande du message est "<commande>"
		Et les paramètres de la commande sont: `<paramètres>`

		Exemples:
			| ligne             | commande | paramètres  |
			| "PASS test\r\n"   | PASS     | ["test"]    |
