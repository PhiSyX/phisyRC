<script lang="ts" setup></script>

<template>
	<div id="design-system--utilities" class="[ size:full scroll:y ]">
		<article>
			<h1 id="utilities">Utilitaires</h1>

			<p>
				Une classe utilitaire applique une seule règle ou un modèle très
				simple et universel.
			</p>

			<p>
				Les classes utilitaires sont utilisées comme des substituts ou
				des classes d'aide. En tant que telles, elles peuvent
				s'appliquer à n'importe quel élément et dans n'importe quel
				contexte, pour faire exactement la même chose à chaque fois.
			</p>
		</article>
	</div>
</template>

<style lang="scss" scoped>
@import "design/functions";
@import "design/mixins";

#design-system--utilities {
	@include --theme using($name) {
		@if $name == dark {
			background: var(--color-black);
		} @else if $name == light {
			background: var(--color-white);
		}
	}
}

article {
	width: 80ch;
	margin: 0 auto;
}
</style>
