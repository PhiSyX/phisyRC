Ce fichier décrit comment les sélecteurs CSS ont été pensé pour l'application.

# Déclaration simple

## Syntaxe: `class-name`

Exemple: `<div class="app">`

## Syntaxe: `...:prop`

Un nom de propriété PEUT être une propriété `CSS` ou une propriété\
personnalisée.\
Elle DOIT être attachée à un `class-name`.

### Exemples de propriétés:

-   `background`, `bg`
-   `width`, `w`
-   `size`, `s`,
-   etc...

### Exemples d'utilisation:

-   `<div class="app:bg">` == ( `class-name` + `:prop` )
-   `<div class="app:color">` == ( `class-name` + `:prop` )

## Syntaxe: `...=value`

Une valeur associée à une propriété (`:prop`) ou à un nom de classe\
(`class-name`).

### Exemples d'utilisation:

-   `<div class="size=1">` == ( `class-name` + ` =value` )
-   `<div class="input:f-family=roboto">` == ( `class-name` + `:prop` + `=value` )
-   `<div class="app:bg=primary">` == ( `class-name` + `:prop` + `=value` )

## Syntaxe: `...(:pseudo-el)`

Pseudo-élément CSS ou un état personnalisé.

### Exemples d'utilisation:

-   `<button class="btn(:focused)">` == ( `class-name` + `:pseudo-el` )
-   `<div class="app:bg=primary(:hover)">`

# Responsive

## Syntaxe: `prefix:...`

Les préfixes responsive sont les suivants:

-   `xs`, `*xs`, `xs*`
-   `sm`, `*sm`, `sm*`
-   `md`, `*md`, `md*`
-   `lg`, `*lg`, `lg*`
-   `xl`, `*xl`, `xl*`

### Exemples d'utilisation:

-   `<div class="xs:gap=1">` ( `responsive` + `class-name` + `=value` )
-   `<div class="*md:gap=1">` ( `responsive` + `class-name` + `=value` )
-   `<div class="lg*:gap=1(:hover)">` ( `responsive` + `class-name` + `=value` )

# Déclaration de module

## Syntaxe: `module` (= `class-name`)

### Exemples d'utilisation:

-   `<div class="network">`

## Syntaxe: `...@submodule`

### Exemples d'utilisation:

-   `<div class="network@server">`

## Syntaxe: `...<selector>`

### Exemples d'utilisation:

-   `<div class="network@server<room>">`
-   `<div class="form<input>">`
