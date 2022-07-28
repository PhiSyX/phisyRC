# phisyRC

## Application

**phisyRC** est une application de Chat utilisant le protocole
**I**nternet **R**elay **C**hat (**IRC**) comme moyen de communication.

## CLI - [**C**ommand **L**ine **I**nterface](./cli/README.md).

**phisyRC** vient avec un outil en ligne de commande permettant de:

1. Lancer l'application de Chat (front-end).

    ```sh
    phisyrc client --gui # en mode graphique.
    # ou
    phisyrc client --tui # en mode textuel.
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

-   `DEBUG` : Filtrer les logs en fonction de la directive.

    Une directive est la cible d'un log qui est définie en fonction d'un
    module.\
    Par exemple, un log défini dans le module `crate::network::connection` aura
    comme directive `phisyrc::network::connection`.

    La valeur attendue est une chaîne de caractères pouvant contenir des
    caractères génériques.\
    Les caractères génériques sont `?` et `*`.\
    Le caractère `?` correspond à un seul caractère, n'importe lequel.\
    Le caractère `*` correspond à aucun ou plusieurs caractères,
    n'importe lesquels.\
    La valeur par défaut est `*`. Autrement dit qui correspond à **TOUS** les caractères.\
    Cela signifie que **TOUS les logs** seront affichés par défaut. —
    [env: `DEBUG=`] [default: "`*`"]

### Commandes

| COMMAND     | DESCRIPTION                                       |
| ----------- | ------------------------------------------------- |
| 1. `client` | Donne accès aux sous-commandes/options du client  |
| 2. `server` | Donne accès aux sous-commandes/options du serveur |

1. Les options de la commande `client` sont:
   | OPTION | DESCRIPTION |
   | ------ | ----------- |
   | `--gui` | Lance l'application de Chat en mode graphique |
   | `--tui` | Lance l'application de Chat en mode textuel |

2. Les options de la commande `server` sont:
   | OPTION | DESCRIPTION |
   | ------ | ----------- |
   | `-d` / `--daemon` | Ouvre les connexions aux serveurs IRC en tâche de fond |

### Drapeaux

| DRAPEAU            | DESCRIPTION                                |
| ------------------ | ------------------------------------------ |
| `-h` / `--help`    | Affiche l'aide (strict minimum / complète) |
| `-V` / `--version` | Affiche la version du programme            |
