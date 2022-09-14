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
  Cela signifie que **TOUS les logs** seront affichés par défaut. — [env:`DEBUG=`] [default: "`*`"]

### Commandes

| COMMAND            | DESCRIPTION                                          |
|--------------------|------------------------------------------------------|
| 1. `client`        | Donne accès aux sous-commandes/options du client     |
| 2. `server`        | Donne accès aux sous-commandes/options du serveur    |
| 3. `make:password` | Génère un mot de passe avec un algorithme de hachage |

1. Commande `client`:

   Les options de la commande `client` sont :

	| OPTION     | DESCRIPTION                                   |
	|------------|-----------------------------------------------|
	| `--tui`    | Lance l'application de Chat en mode textuel   |
	| `--gui`    | Lance l'application de Chat en mode graphique |
	| `--config` | configuration du client.                      |

2. Commande `server`

   Les options de la commande `server` sont :

	| OPTION            | DESCRIPTION                                            | TYPE     | DEFAULT                                            |
	|-------------------|--------------------------------------------------------|----------|----------------------------------------------------|
	| `-d` / `--daemon` | Ouvre les connexions aux serveurs IRC en tâche de fond | `bool`   | `false`                                            |
	| `--config`        | Fichier de configuration du serveur                    | `string` | Voir dans le fichier `.phisyrc/config/global.toml` |

3. Commande `make:password`:

   La commande prend un argument obligatoire, qui est le mot de passe
   à encoder. Le mot de passe est encodé en fonction de la valeur de la
   variable d'environnement `APP_SECRET_KEY`.

   Les options de la commande sont :

	| OPTION   | DESCRIPTION                      | TYPE                  | DEFAULT    |
	|----------|----------------------------------|-----------------------|------------|
	| `--algo` | Algorithme de hachage à utiliser | `"plain"`, `"argon2"` | `"argon2"` |

   Exemple

   ```sh
   $ phisyrc make:password test

   Le mot de passe 'test' généré par Argon2:
   $argon2id$v=19$m=4096,t=3,p=1$bWQ5b2prMXBJY0UyNGRSc29wODZIZGk5ODduZkRLaTU$QwsRN6Ds44/mZb5abqBq8/Lzgb1Y33qRUcpUKXO6GF0
   ```

### Drapeaux

| DRAPEAU            | DESCRIPTION                                |
|--------------------|--------------------------------------------|
| `-h` / `--help`    | Affiche l'aide (strict minimum / complète) |
| `-V` / `--version` | Affiche la version du programme            |

### Options

| OPTION            | DESCRIPTION                            | DEFAULT                        |
|-------------------|----------------------------------------|--------------------------------|
| `-c` / `--config` | Configuration globale de l'application | `.phisyrc/config/global.toml`  |
