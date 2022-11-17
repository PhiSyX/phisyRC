<template>
	<div class="app@waves [ pos-a:full s:full ]">
		<svg
			version="1.1"
			xmlns="http://www.w3.org/2000/svg"
			width="100%"
			height="100%"
			viewBox="0 0 960 560"
			preserveAspectRatio="xMidYMid slice"
		>
			<path
				fill="var(--wave-1)"
				d="M-5.8,569.6l-0.2-33.6c48-30.2,86.9-85.8,103.6-114.5c18.2-31.4,103.3-166.7,252.9-91.8
	c131.7,65.9,284.2,149.4,476.4,35.5c21-12.4,76-57.8,139.2-55v259.5"
			/>
			<path
				fill="var(--wave-2)"
				d="M-5.8,569.6l-0.2-33.6c48-30.2,86.9-85.8,103.6-114.5c18.2-31.4,103.3-166.7,252.9-91.8
	c131.7,65.9,284.2,149.4,476.4,35.5c21-12.4,76-57.8,139.2-55v259.5"
			/>
			<path
				fill="var(--wave-3)"
				d="M-5.8,569.6l-0.2-33.6c48-30.2,86.9-85.8,103.6-114.5c18.2-31.4,103.3-166.7,252.9-91.8
	c131.7,65.9,284.2,149.4,476.4,35.5c21-12.4,76-57.8,139.2-55v259.5"
			/>
		</svg>
	</div>
</template>

<style lang="scss">
@import "design/functions";
@import "design/mixins";

@include selector-class("app@waves") {
	pointer-events: none;

	@include --theme using ($name) {
		@if $name == dark {
			--wave-1: var(--color-grey800);
			--wave-2: var(--color-grey);
			--wave-3: #212121;
		} @else if $name == light {
			--wave-1: var(--color-indigo50);
			--wave-2: var(--color-indigo200);
			--wave-3: var(--color-indigo400);
		}
	}

	> svg path {
		animation: wave-anim 30s ease-out infinite alternate-reverse;
		opacity: 0.9;
	}

	> svg path:nth-child(1) {
		animation-delay: -5s;
		animation-duration: 15s;
	}

	> svg path:nth-child(2) {
		animation-delay: -5s;
		animation-duration: 10s;
	}

	> svg path:nth-child(3) {
		animation-delay: -5s;
		animation-duration: 5s;
	}
}

@keyframes wave-anim {
	0% {
		transform: translate3d(calc(0 - space(10)), 0, 0);
	}
	100% {
		transform: translate3d(space(10), 0, 0);
	}
}
</style>
