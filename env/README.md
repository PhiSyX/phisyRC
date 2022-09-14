## Variables d'environnement

Les variables d'environnement suivant sont acceptées dans l'application :

| VARIABLE         | DESCRIPTION                           |
|------------------|---------------------------------------|
| `APP_SECRET_KEY` | Clé secrète de l'application (requis) |
| `DEBUG`          | Voir plus haut.                       |

### TODO

- [ ] Pouvoir définir les variables dans un fichier en fonction du mode
      d'exécution.
  - [ ] Environnement de prod : `.env`
  - [x] Environnement de dev : `.env.local`
  - [ ] Environnement de test : `.env.test.local`
