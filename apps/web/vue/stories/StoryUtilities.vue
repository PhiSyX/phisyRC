<script lang="ts" setup></script>

<template>
	<div id="design-system--utilities" class="[ size:full scroll:y ]">
		<article class="pb=3">
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

			<h2 id="alignments">Alignements</h2>
			<h3 id="text-align">Alignement de texte:</h3>
			<ul>
				<li class="align-t:left">
					<strong>.align-t:</strong><code>left</code>
				</li>
				<li class="align-t:center">
					<strong>.align-t:</strong><code>center</code>
				</li>
				<li class="align-t:right">
					<strong>.align-t:</strong><code>right</code>
				</li>
			</ul>

			<h3 id="flex-align">Alignement Flexbox et Grid:</h3>

			<p>
				Les classes utilitaires suivantes font références à la propriété
				CSS
				<code>align-items</code> (<code>.align-i:</code>) et
				<code>align-self</code> (<code>.align-s:</code>).
			</p>

			<ul>
				<li class="align-i:start">
					<strong>.align-i:</strong><code>start</code>
				</li>
				<li class="align-i:center">
					<strong>.align-i:</strong><code>center</code>
				</li>
				<li class="align-i:end">
					<strong>.align-i:</strong><code>end</code>
				</li>
				<li class="align-s:start">
					<strong>.align-s:</strong><code>start</code>
				</li>
				<li class="align-s:center">
					<strong>.align-s:</strong><code>center</code>
				</li>
				<li class="align-s:end">
					<strong>.align-s:</strong><code>end</code>
				</li>
			</ul>

			<p>
				Les classes utilitaires suivantes font références à la propriété
				CSS
				<code>justify-content</code> (<code>.gap=</code>).
			</p>

			<ul>
				<li class="gap=space-between">
					<strong>.gap=</strong><code>space-between</code>
				</li>
				<li class="align-j:space-around">
					<strong>.align-j:</strong><code>space-around</code>
				</li>
				<li class="align-j:space-evenly">
					<strong>.align-j:</strong><code>space-evenly</code>
				</li>
				<li class="align-j:stretch">
					<strong>.align-j:</strong><code>stretch</code>
				</li>
			</ul>

			<p>
				Les classes utilitaires suivantes font références à la propriété
				CSS
				<code>gap</code> ou <code>grid-gap</code> (<code>.gap=#</code>).
				Ici <code>#</code> correspond à une unité (nombre) de 0 à 20.
				Chaque unité correspond environs à <code>1rem</code>, mais cela
				dépendra en réalité de la variable CSS <code>--space</code>.
			</p>

			<p>Exemples:</p>
			<ul>
				<li class="gap=1"><strong>.gap=</strong><code>1</code></li>
				<li class="gap=12"><strong>.gap=</strong><code>12</code></li>
				<li class="gap=20"><strong>.gap=</strong><code>20</code></li>
			</ul>

			<h2 id="backgrounds">Arrière-plan</h2>

			<h3 id="backgrounds-corners">Border radius</h3>
			<p>
				Les classes utilitaires suivantes font références à la propriété
				CSS
				<code>border-radius</code> (<code>.border:radius=#</code>). Ici
				<code>#</code> correspond à un nombre de 0 à 20.
			</p>

			<p>Exemples:</p>
			<ul>
				<li class="border:radius=3">
					<strong>.border:radius=</strong><code>3</code>
				</li>
				<li class="border-t:radius=6">
					<strong>.border-t:radius=</strong><code>6</code>
				</li>
				<li class="border-r:radius=9">
					<strong>.border-r:radius=</strong><code>9</code>
				</li>
				<li class="border-b:radius=12">
					<strong>.border-b:radius=</strong><code>12</code>
				</li>
				<li class="border-l:radius=16">
					<strong>.border-l:radius=</strong><code>16</code>
				</li>
			</ul>

			<ul>
				<li class="border:radius=full">
					<strong>.border:radius=</strong><code>full</code>
				</li>
			</ul>

			<h3 id="backgrounds-box-shadow">Box shadow</h3>
			<p>
				Les classes utilitaires suivantes font références à la propriété
				CSS
				<code>box-shadow</code> (<code>.box:shadow</code>) ou
				<code>filter: drop-shadow</code> (<code>.box-fd:shadow</code>)
			</p>

			<p>Exemples</p>
			<ul>
				<li class="size=15 [ box:shadow ]">.box:shadow</li>
			</ul>
			<ul class="list:reset gap=2 flex">
				<li
					class="size=20"
					style="
						background-image: var(--logo);
						background-size: cover;
					"
				>
					sans .box-fd:shadow
				</li>
				<li
					class="size=20 [ box-fd:shadow ]"
					style="
						background-image: var(--logo);
						background-size: cover;
					"
				>
					avec .box-fd:shadow
				</li>
			</ul>
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
