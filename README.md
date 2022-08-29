# phisyRC

## Application

**phisyRC** est une application complète de Chat utilisant le protocole
**I**nternet **R**elay **C**hat (**IRC**) comme moyen de communication.

L'**I**nterface **U**tilisateur de l'application de Chat **phisyRC** est
développée sous plusieurs formes:

1. **T**extuel (**TUI**) pour le terminal.

   L'interface est minimaliste.

2. **G**raphique (**GUI**) pour le bureau.

3. **W**eb pour les navigateurs modernes.

4. **M**obile pour les appareils mobiles.

## Administration

Un panel d'administration est disponible pour les **op**érateurs **IRC**
(IRCop's) des serveurs et permet de gérer la configuration IRC, les sanctions
des utilisateurs, les canaux, etc, de _leurs_ serveurs IRC.

## IRC - [**I**nternet **R**elay **C**hat](./irc/README.md).

**phisyRC** utilise le protocole
[**IRC**](https://datatracker.ietf.org/doc/html/rfc1459) comme moyen de
communication. Les spécifications suivantes sont utilisées:

- [[IRC-ARCH](https://datatracker.ietf.org/doc/html/rfc2810)] : Architecture
- [[IRC-CHAN](https://datatracker.ietf.org/doc/html/rfc2811)] : Channel
  Management
- [[IRC-CLIENT](https://datatracker.ietf.org/doc/html/rfc2812)] : Client
  Protocol
- [[IRC-SERVER](https://datatracker.ietf.org/doc/html/rfc2813)] : Server
  Protocol

Toutefois, **phisyRC** se laisse la liberté de ne pas implémenter TOUTES les
fonctionnalités du protocole IRC, et d'implémenter des fonctionnalités qui ne
sont pas dans le protocole IRC ;-).

## CLI - [**C**ommand **L**ine **I**nterface](./cli/README.md).

**phisyRC** vient avec un outil en ligne de commande permettant de:

1. Lancer l'application de Chat (front-end).

   ```sh
   phisyrc client --tui # en mode textuel.
   # ou
   phisyrc client --gui # en mode graphique.
   ```

2. Ouvrir les connexions aux serveurs IRC (back-end).
   ```sh
   phisyrc server    # en premier plan.
   # ou
   phisyrc server -d # en arrière-plan. (-d pour daemon)
   ```

### Syntaxe de la ligne de commande

```sh
[ENV=] phisyrc [COMMAND] [FLAG] [OPTIONS]
```

### Variables d'environnement

- `DEBUG` : Filtrer les logs en fonction de la directive.

  Une directive est la cible d'un log qui est définie en fonction d'un module.\
  Par exemple, un log défini dans le module `crate::network::connection` aura
  comme directive `phisyrc::network::connection`.

  La valeur attendue est une chaîne de caractères pouvant contenir des
  caractères génériques.\
  Les caractères génériques sont `?` et `*`.\
  Le caractère `?` correspond à un seul caractère, n'importe lequel.\
  Le caractère `*` correspond à aucun ou plusieurs caractères, n'importe
  lesquels.\
  La valeur par défaut est `*`. Autrement dit qui correspond à **TOUS** les
  caractères.\
  Cela signifie que **TOUS les logs** seront affichés par défaut. — [env:
  `DEBUG=`] [default: "`*`"]

### Commandes

| COMMAND            | DESCRIPTION                                          |
| ------------------ | ---------------------------------------------------- |
| 1. `client`        | Donne accès aux sous-commandes/options du client     |
| 2. `server`        | Donne accès aux sous-commandes/options du serveur    |
| 3. `make:password` | Génère un mot de passe avec un algorithme de hachage |

1. Commande `client`:

   Les options de la commande `client` sont:

   | OPTION  | DESCRIPTION                                   |
   | ------- | --------------------------------------------- |
   | `--tui` | Lance l'application de Chat en mode textuel   |
   | `--gui` | Lance l'application de Chat en mode graphique |

2. Commande `server`

   Les options de la commande `server` sont:

   | OPTION            | DESCRIPTION                                            | TYPE     | DEFAULT                       |
   | ----------------- | ------------------------------------------------------ | -------- | ----------------------------- |
   | `-d` / `--daemon` | Ouvre les connexions aux serveurs IRC en tâche de fond | `bool`   | `false`                       |
   | `--config`        | Fichier de configuration IRC                           | `string` | `".phisyrc/config/ircd.toml"` |

3. Commande `make:password`:

   La commande prend un argument obligatoire, qui est le mot de passe à encoder.

   Les options de la commande sont:

   | OPTION   | DESCRIPTION                      | TYPE                  | DEFAULT    |
   | -------- | -------------------------------- | --------------------- | ---------- |
   | `--algo` | Algorithme de hachage à utiliser | `"plain"`, `"argon2"` | `"argon2"` |

### Drapeaux

| DRAPEAU            | DESCRIPTION                                |
| ------------------ | ------------------------------------------ |
| `-h` / `--help`    | Affiche l'aide (strict minimum / complète) |
| `-V` / `--version` | Affiche la version du programme            |
