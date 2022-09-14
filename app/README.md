## Application

**phisyRC** est une application complète de Chat utilisant le protocole
**I**nternet **R**elay **C**hat (**IRC**) comme moyen de communication.

L'**I**nterface **U**tilisateur de l'application de Chat **phisyRC** est
développée sous plusieurs formes :

1. [ ] **T**extuel (**TUI**) pour le terminal.

   - L'interface va sûrement être minimaliste. Nous allons probablement
	 utiliser `tui-rs` pour créer cette interface.

2. [ ] **G**raphique (**GUI**) pour le bureau.

   Nous n'avons pas encore décidé la technologie à utiliser pour construire une
   interface graphique avec Rust. L'écosystème de Rust n'est sensiblement pas
   encore assez développé pour créer de **belles** interfaces graphiques, du
   moins comme le créateur les aiment !

	Quelques pistes:
	- Créer notre propre GUI avec Rust. Pas impossible, mais cela nous
      prendrait un temps monstre.
    - Avec Rust, `tauri` peut correspondre à notre besoin, cependant la
	  compilation du projet devient trop longue. Ce qui est un enfer
	  lorsque nous développons avec.
    - Créer une application de bureau avec `flutter`? Pourquoi pas.
    - Créer une application de bureau avec `C#`? Pourquoi pas.

3. [ ] **W**eb pour les navigateurs modernes.

	Quelques pistes:
    - Créer une application web avec `vue` ou `react`?
    - Créer une application web avec `yew` (Rust w/ WASM)?

4. [ ] **M**obile pour les appareils mobiles.

   - Android
   - iOS

## Administration

Un panel d'administration est disponible pour les **op**érateurs **IRC**
(IRCop's) des serveurs et permet de gérer la configuration IRC, les sanctions
des utilisateurs, les canaux, etc, de _leurs_ serveurs IRC.
