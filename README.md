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
   Le mot de passe est encodé en fonction de la valeur de la variable
   d'environnement `APP_SECRET_KEY`.

   Les options de la commande sont:

   | OPTION   | DESCRIPTION                      | TYPE                  | DEFAULT    |
   | -------- | -------------------------------- | --------------------- | ---------- |
   | `--algo` | Algorithme de hachage à utiliser | `"plain"`, `"argon2"` | `"argon2"` |

   Exemple

   ```sh
   $ phisyrc make:password test

   Le mot de passe 'test' généré par Argon2: $argon2id$v=19$m=4096,t=3,p=1$bWQ5b2prMXBJY0UyNGRSc29wODZIZGk5ODduZkRLaTU$QwsRN6Ds44/mZb5abqBq8/Lzgb1Y33qRUcpUKXO6GF0
   ```

### Drapeaux

| DRAPEAU            | DESCRIPTION                                |
| ------------------ | ------------------------------------------ |
| `-h` / `--help`    | Affiche l'aide (strict minimum / complète) |
| `-V` / `--version` | Affiche la version du programme            |

## Variables d'environnement

Les variables d'environnement suivantes sont acceptées dans l'application:

| VARIABLE         | DESCRIPTION                           |
| ---------------- | ------------------------------------- |
| `APP_SECRET_KEY` | Clé secrète de l'application (requis) |
| `DEBUG`          | Voir plus haut.                       |

### TODO

- [ ] Pouvoir définir les variables dans un fichier en fonction du mode
      d'execution.
  - [ ] Environnement de prod : `.env`
  - [x] Environnement de dev : `.env.local`
  - [ ] Environnement de test : `.env.test.local`

## A propos du code, de la technique

1) Tout n'est pas parfait dans le code actuel. Le créateur en est conscient.

2) Dans certaines parties du code, les commentaires qui expliquent ce que font
certains bloc de code ne sont là qu'à titre informatifs et/ou de mémorisations.
Le créateur en est conscient que les commentaires NE DOIVENT PAS être là pour
expliquer ce qui se passe dans les blocs de codes qui suivent les commentaires.
Tout n'est pas commenté ou documenté ; en effet, il peut arriver qu'il ait
des noms de fonctions, de variables, etc. qui sont assez explicites pour
comprendre ce qu'il va se passer. Et parfois, c'est par oubli, ou parfois
par fainéantise. Évidemment, le créateur écrira les documentations de code
manquantes au fur et à mesure.

3) **phisyRC** n'a pas été pensé avec une architecture particulière.
Pas de MVC, Clean Architecture, et que sais-je encore. Le créateur en est
conscient qu'on PEUT tirer des bénéfices sur certains points de ces concepts,
mais n'en tiendra pas rigueur pour le moment. Néanmoins, le projet dans sa
globalité essaie de garder la même structure, d'être le plus cohérent possible
avec le reste ou l'intégralité du projet.
