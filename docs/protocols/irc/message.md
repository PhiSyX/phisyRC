> **RFC**:\
> Les messages IRC sont toujours terminés par une paire CR-LF (Carriage Return -
> Line Feed), et ces messages ne doivent pas dépasser 512 caractères, en
> comptant tous les caractères y compris la paire CR-LF de fin de ligne. \r\n

À l'époque où les RFC's IRC ont été écrites, les métadonnées n'existaient pas
encore. Maintenant que nous pouvons ajouter des métadonnées aux messages, 512
bytes pour un message n'est pas raisonnable.

Une taille limite a été fixée à 8191 bytes pour les métadonnées, cependant la
taille limite du reste du message reste inchangée: de 512 bytes. Cette limite
est séparée entre les métadonnées ajoutées par le serveur et les métadonnées
envoyées par le client. Cela empêche les serveurs de dépasser la limite globale
en ajoutant des métadonnées à un message client envoyé dans la limite autorisée.

Chaque message IRC peut comporter jusqu'à quatre parties principales :

1. les métadonnées (qui sont facultatives)

   > La présence de métadonnées est indiquée par le caractère '@' qui doit être
   > le premier caractère du message lui-même. Il ne doit pas y avoir d'espace
   > entre le premier caractère et le caractère qui le suit.

   Résumé du format ABNF des métadonnées :

   ```text
   tags  =  "@" tag *[ ";" tag ]
   tag   =  tagkey [ "=" tagvalue ]
   ```

   Exemples de ce que peuvent représenter le format des métadonnées:

   ```text
   @admin                                     ("@" tagkey)
   @id=1;first-name=Mike                      ("@" tagkey "=" tagvalue ";" tagkey "=" tagvalue)
   @example.org/foo=bar                       ("@" tagvendor tagkeyname "=" tagvalue)
   @+icon=https://example.com/favicon.png     ("@" tagclient tagkeyname "=" tagvalue)
   ```

2. le préfixe (qui est facultatif)

   > Le préfixe est placé après les métadonnées s'ils existent. La présence d'un
   > préfixe est indiquée par le caractère ':'. Il ne doit pas y avoir d'espace
   > entre caractère ':' et le caractère qui le suit. Si les métadonnées
   > n'existent pas, le caractère ':' doit être le premier caractère du message
   > lui-même.\
   > Le préfixe est utilisé par les **SERVEURS** pour indiquer la véritable
   > origine d'un message.\
   > Les **CLIENTS** ne doivent pas utiliser de préfixe lorsqu'ils envoient un
   > message. S'ils en utilisent un, le seul préfixe valide est le pseudonyme
   > (`nickname`) enregistré qui est associé au client.

   Résumé du format ABFN d'un préfixe :

   ```text
   prefix  =  ":" servername / ( nickname [ [ "!" user ] "@" host ] )
   ```

   Exemple de ce que peut représenter le format d'un préfixe:

   ```text
   :127.0.0.1               (servername)
   :irc.local.host          (servername)
   :john                    (nickname)
   :john@127.0.0.1          (nickname "@" host)
   :john!doe@john.doe.host  (nickname "!" user "@" host)
   ```

3. la commande

   > **RFC**:\
   > La commande doit être une commande IRC valide.\
   > Une commande est une suite de caractères alphabétiques (ex: `PRIVMSG`) ou
   > une suite de **3** chiffres (ex: `001`).

   > **phisyrc**:\
   > Pour qu'une commande soit considérée comme valide, les **SERVEURS** doivent
   > définir au préalable les commandes qu'ils peuvent accepter par les clients.
   > les commandes numériques que l'IRCd gèrent sont décrits à l'adresse du
   > dépôt GitHub: [PhiSyX/phisyrc](./irc/numeric)

   > Seuls les **SERVEURS** peuvent envoyer des commandes qui ne contiennent que
   > 3 chiffres. Ces chiffres représentent les différents codes numériques de
   > réponses que peut renvoyer le serveur. [Numeric Replies]

   Résumé du format ABNF d'une commande:

   ```text
   command = 1*letter / 3digit
   ```

   Exemples de ce que peut représenter le format d'une commande:

   ```text
   PRIVMSG     (1*letter)
   401         (3digit)
   ```

4. les paramètres de la commande (conditionnelle)

   > **RFC**:\
   > Les paramètres sont des suites de caractères séparés par un espace blanc.\
   > Les paramètres ne doivent pas dépasser 15 paramètres.

   > **phisyrcd**:\
   > L'absence de paramètres pour une commande qui en nécessitent est considérée
   > comme invalide.\
   > La présence de paramètres pour une commande qui n'en nécessitent pas est
   > considérée comme invalide.

   Résumé du format ABNF des paramètres:

   ```text
   params     =  *14( SPACE middle ) [ SPACE ":" trailing ]
              =/  14( SPACE middle ) [ SPACE [ ":" ] trailing ]
   ```

   Exemples de ce que peuvent représenter le format des paramètres:

   ```text
   #irc :mon super message
   PhiSyX AWAYLEN=307 BOT=B CASEMAPPING=ascii CHANLIMIT=#:10 CHANMODES=beI,kLf,lH,psmntirzMQNRTOVKDdGPZSCc CHANNELLEN=32 CHANTYPES=# CLIENTTAGDENY=*,-draft/typing,-typing DEAF=d ELIST=MNUCT EXCEPTS EXTBAN=~,GptmTSOcarnqjf :are supported by this server",
   ```

Exemples de ce peut retourner un message IRC complet:

```text
message    =  [ "@" tag *[ ";" tag ] SPACE ] [ ":" prefix SPACE ] command [ params ] crlf
```

```text
@id=1 :foo!bar@foo.bar.host TOPIC #irchan :param blabla\r\n       (tags prefix command params)
:foo!bar@foo.bar.host TOPIC #irchan :param blabla\r\n             (prefix command params)
TOPIC #irchan :params\r\n                                         (command params)
AWAY :\r\n                                                        (command)
```

Ce module permet de vérifier la présence de ces parties principales.
