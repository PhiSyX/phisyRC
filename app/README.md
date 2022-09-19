## Application

**phisyRC** est une application complète de Chat utilisant le protocole
**I**nternet **R**elay **C**hat (**IRC**) comme moyen de communication.

L'**I**nterface **U**tilisateur de l'application de Chat **phisyRC** est
développée sous plusieurs formes :

1. [ ] **T**extuel (**TUI**) pour le terminal.

	L'interface va sûrement être minimaliste.
	Nous n'avons pas encore décidé la technologie à utiliser pour construire une
	interface textuelle.

	Néanmoins voici quelques pistes:
	- Créer notre propre application textuelle. Pas impossible, mais cela nous
	  prendrait trop de temps pour le développer.
	- Créer une application avec `tui-rs`.

2. [ ] **G**raphique (**GUI**) pour le bureau.

   Nous n'avons pas encore décidé la technologie à utiliser pour construire une
   interface graphique. L'écosystème de Rust n'est sensiblement pas encore
   assez développé pour créer de **belles** interfaces graphiques, du moins
   comme le créateur les aiment !

	Néanmoins, voici quelques pistes avec Rust:
	- Créer notre propre application graphique. Pas impossible, mais cela nous
	  prendrait trop de temps pour le développer
	- Créer une application graphique avec `druid`.
    - Créer une application graphique avec `tauri`.
	  Cela correspondrait parfaitement à nos besoins. Cependant la compilation
	  du projet avec ce dernier devient trop importante. Ce qui constitue est
	  un frein pour ce projet.

	Autre pistes:
    - Créer une application de bureau avec `dart` / `flutter`.
    - Créer une application de bureau avec `C#`.

3. [ ] **W**eb pour les navigateurs modernes.

	Nous n'avons pas encore décidé la technologie à utiliser pour construire une
	interface web.

	Néanmoins, voici quelques pistes:
	- Créer une application web En JavaScript vanilla.
    - Créer une application web avec `vue` / `react`.
    - Créer une application web en utilisant `WASM`.

4. [ ] **M**obile pour les appareils mobiles.

   - Android
   - iOS

## Administration

Un panel d'administration est disponible pour les **op**érateurs **IRC**
(IRCop's) des serveurs et permet de gérer la configuration IRC, les sanctions
des utilisateurs, les canaux, etc, de _leurs_ serveurs IRC.
