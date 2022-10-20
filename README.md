# Bienvenue sur phisyRC

**phisyRC** est une application de Chat complète.

L'**I**nterface **U**tilisateur de l'application de Chat **phisyRC** est\
développée sous plusieurs formes :

- [ ] **T**extuelle
- [ ] **G**raphique
- [ ] Mobile
- [ ] Web

**phisyRC** vient avec un outil en ligne de commande permettant de lancer un\
[serveur de Chat](docs/developers/chat-server.md).


# Table des matières
- [phisyRC Chat Server](#phisyrc-chat-server)
	- [Syntaxe de la ligne de commande](#syntaxe-de-la-ligne-de-commande)
	- [Variables d'environnement](#variables-denvironnement)
	- [Commandes](#commandes)
	- [Options](#options)

# phisyRC Chat Server

**phisyRC** vient avec un outil en ligne de commande permettant de lancer un\
serveur de Chat.

## Syntaxe de la ligne de commande

```sh
[ENV=] phisyrc-server [COMMAND] [OPTIONS]
```

## Variables d'environnement

- `DEBUG` : Filtrer les logs en fonction de la directive.

  Une directive est la cible d'un log qui est définie en fonction d'un module.\
  Par exemple, un log défini dans le module `crate::network::connection` aura\
  comme directive `phisyrc::network::connection`.

  La valeur attendue est une chaîne de caractères pouvant contenir des\
  caractères génériques.\
  Les caractères génériques sont `?` et `*`.\
  Le caractère `?` correspond à un seul caractère, n'importe lequel.\
  Le caractère `*` correspond à aucun ou plusieurs caractères, n'importe\
  lesquels.\
  La valeur par défaut est `*`. Autrement dit qui correspond à **TOUS** les\
  caractères.\
  Cela signifie que **TOUS les logs** seront affichés par défaut.

  [env:`DEBUG=`] [default: "`*`"]

- `PROCESS_ENV` : Dans quel mode le serveur doit être lancé.

  Les modes sont utilisés pour:

  1. choisir le fichier d'environnement à utiliser ;
  2. le système de log ;
  3. des informations renvoyées aux clients concernant des
     \
     messages comportements du programme spécifique à certains modes ;
  4. ...

## Commandes

| COMMAND            | DESCRIPTION                                          |
| ------------------ | ---------------------------------------------------- |
| 1. `config`        | Gérer la configuration serveur                       |
| 2. `make:password` | Génère un mot de passe avec un algorithme de hachage |

1. Commande `config`

   La commande `config` permet de gérer la configuration du serveur.

   | OPTION     | DESCRIPTION                                    | TYPE   | DEFAULT |
   | ---------- | ---------------------------------------------- | ------ | ------- |
   | `--delete` | Supprime le fichier de configuration           | `bool` | `false` |
   | `--show`   | Affiche le contenu du fichier de configuration | `bool` | `true`  |

2. Commande `make:password`:

   La commande prend un argument obligatoire, qui est le mot de passe à\
   encoder. Le mot de passe est encodé en fonction de la valeur de la\
   variable d'environnement `APP_SECRET_KEY`.

   Les options de la commande sont :

   | OPTION   | DESCRIPTION                      | TYPE       | DEFAULT    |
   | -------- | -------------------------------- | ---------- | ---------- |
   | `--algo` | Algorithme de hachage à utiliser | `"argon2"` | `"argon2"` |

## Options

| OPTION            | DESCRIPTION                         | TYPE         | DEFAULT                   |
| ----------------- | ----------------------------------- | ------------ | ------------------------- |
| `-c` / `--config` | Fichier de configuration du serveur | `string`     | `$phisyrc`/server.toml    |
| `--mode`          | Mode                                | `ProcessEnv` | `ProcessEnv::DEVELOPMENT` |

La valeur de `$phisyrc` dépend de l'OS.

| `$phisyrc`  | DESTINATION                                             |
| ----------- | ------------------------------------------------------- |
| **Linux**   | /home/&lt;user&gt;/.config/phisyrc                      |
| **Mac OS**  | /Users/&lt;user&gt;/Library/Application Support/phisyrc |
| **Windows** | C:\Users\\&lt;user&gt;\AppData\Roaming\phisyrc          |


# Table des matières
- [phisyRC Chat Server](#phisyrc-chat-server)
	- [Syntaxe de la ligne de commande](#syntaxe-de-la-ligne-de-commande)
	- [Variables d'environnement](#variables-denvironnement)
	- [Commandes](#commandes)
	- [Options](#options)
