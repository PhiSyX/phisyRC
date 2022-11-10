<script lang="ts" setup></script>

<template>
	<div id="design-system--typo" class="[ size:full scroll:y ]">
		<article>
			<h1 id="font">Font</h1>

			<p>
				Les polices Web choisies pour l'application sont
				<a href="https://fonts.google.com/specimen/Nunito">Nunito</a> et
				<a href="https://fonts.google.com/specimen/Roboto">Roboto</a> à
				partir des liens Google.
			</p>

			<p class="f-family=nunito">
				Exemple de paragraphe avec la police Nunito avec la classe
				utilitaire <code>.f-family=nunito</code>
			</p>
			<p class="f-family=roboto">
				Exemple de paragraphe avec la police Roboto avec la classe
				utilitaire <code>.f-family=roboto</code>
			</p>

			<h2 id="heading">Heading</h2>

			<p>Les en-têtes <code>&lt;h#></code> sont responsive.</p>

			<table>
				<thead>
					<tr>
						<th># en-tête</th>
						<th>Utilitaire</th>
					</tr>
				</thead>
				<tbody>
					<tr>
						<td><code>&lt;h1&gt;</code></td>
						<td class="heading=1"><code>.heading=1</code></td>
					</tr>
					<tr>
						<td><code>&lt;h2&gt;</code></td>
						<td class="heading=2"><code>.heading=2</code></td>
					</tr>
					<tr>
						<td><code>&lt;h3&gt;</code></td>
						<td class="heading=3"><code>.heading=3</code></td>
					</tr>
					<tr>
						<td><code>&lt;h4&gt;</code></td>
						<td class="heading=4"><code>.heading=4</code></td>
					</tr>
					<tr>
						<td><code>&lt;h5&gt;</code></td>
						<td class="heading=5"><code>.heading=5</code></td>
					</tr>
					<tr>
						<td><code>&lt;h6&gt;</code></td>
						<td class="heading=6"><code>.heading=6</code></td>
					</tr>
				</tbody>
			</table>
		</article>
	</div>
</template>

<style lang="scss" scoped>
@import "design/functions";
@import "design/mixins";

#design-system--typo {
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
