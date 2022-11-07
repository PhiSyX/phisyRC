# Construire à partir des sources

## Requiert

phisyRC requiert d'avoir les exécutables suivant sur son système d'exploitation:

1. git
2. [Rust](https://www.rust-lang.org/tools/install)
3. [Node](https://nodejs.org/) et [pnpm](https://pnpm.io/installation)

À savoir que les versions utilisées par le mainteneur sont:

-   Version de Cargo: **1.67.0-nightly**
    -   La feature `let_chains` est malheureusement instable. Elle n'est donc
        pas disponible en version `stable`.
-   Version de Node et pnpm: **v19.0.1** && **7.13.2**

---

## Installation

1. **Cloner le dépôt**

```shell
git clone https://github.com/PhiSyX/phisyRC
cd phisyRC
```

2. **Installer les dépendances node (npm)**

```bash
pnpm install
```

## Utilisation

-   [ ] **Desktop**: TODO.
-   [ ] **Mobile**: TODO.
-   [ ] **Terminal**: TODO.
-   [ ] **Web** : TODO.
-   [x] Serveur **Web** de développement: `pnpm web dev`

Pour ouvrir un serveur de Chat

```bash
cargo run
```
