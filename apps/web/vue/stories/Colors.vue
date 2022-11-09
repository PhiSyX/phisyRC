<script lang="ts" setup>
import { ref } from "vue";

const colors_variant_list: FixedArray<str, 14> = [
	"50",
	"100",
	"200",
	"300",
	"400",
	"500",
	"600",
	"700",
	"800",
	"900",
	"a100",
	"a200",
	"a400",
	"a700",
];

const colors_names_list: FixedArray<str, 19> = [
	"red",
	"pink",
	"purple",
	"deep-purple",
	"indigo",
	"blue",
	"light-blue",
	"cyan",
	"teal",
	"green",
	"light-green",
	"lime",
	"yellow",
	"amber",
	"orange",
	"deep-orange",
	"brown",
	"grey",
	"blue-grey",
];

const enum CopyValueType {
	Var = "Variable",
	ColorName = "Nom de couleur",
	Hexadecimal = "Couleur hexadécimal",
	Rgb = "RGV",
	Hls = "HLS",
}

let copy_mode = ref(CopyValueType.Var);

function copy2clipboard(color_name: str, variant: str) {
	let write_str = "";

	switch (copy_mode.value) {
		case CopyValueType.Var: {
			write_str = `var(--color-${color_name}${variant})`;
			break;
		}

		case CopyValueType.ColorName: {
			write_str = `--color-${color_name}${variant}`;
			break;
		}

		case CopyValueType.Hexadecimal: {
			write_str = getComputedStyle(
				document.documentElement
			).getPropertyValue(`--color-${color_name}${variant}`);
			break;
		}

		case CopyValueType.Rgb: {
			write_str = getComputedStyle(document.documentElement)
				.getPropertyValue(`--color-${color_name}${variant}_rgb`)
				.trim();
			if (write_str.length != 0) {
				write_str = `rgb(${write_str})`;
			}
			break;
		}

		case CopyValueType.Hls: {
			write_str = getComputedStyle(document.documentElement)
				.getPropertyValue(`--color-${color_name}${variant}_hsl`)
				.trim();
			if (write_str.length != 0) {
				write_str = `hsl(${write_str})`;
			}
			break;
		}
	}

	if (write_str.length == 0) {
		return;
	}

	navigator.clipboard.writeText(write_str);
}
</script>

<template>
	<div class="[ flex! p=1 align-i:center size:full scroll:y ]">
		<p>Copie au format : {{ copy_mode }}</p>

		<div class="[ flex gap=1 mb=2 ]">
			<button @click="copy_mode = CopyValueType.Var">
				var(--color-name)
			</button>

			<button @click="copy_mode = CopyValueType.ColorName">
				--color-name
			</button>

			<button @click="copy_mode = CopyValueType.Hexadecimal">
				#xxxxxx
			</button>

			<button @click="copy_mode = CopyValueType.Rgb">
				rgb(red, green, blue)
			</button>
			<button @click="copy_mode = CopyValueType.Hls">
				hsl(hue, saturation, lightness)
			</button>
		</div>

		<div
			v-for="variant in colors_variant_list"
			class="[ flex align-i:center ]"
		>
			<div
				v-for="color_name in colors_names_list"
				class="square [ flex align-i:center size=6 ]"
				tabindex="-1"
				:style="{
					backgroundColor: `var(--color-${color_name}${variant})`,
				}"
				:title="`--color-${color_name}${variant}`"
				@click="copy2clipboard(color_name, variant)"
			/>
		</div>
	</div>
</template>

<style lang="scss" scoped>
@import "design/functions";
@import "design/mixins";

button {
	padding: space(1);
	border: 0;
	border-radius: 2px;

	color: var(--color-black);

	@include --theme using($name) {
		background-color: var(--color-blue200);
		@if $name == dark {
			&:focus {
				outline: 2px solid var(--color-blue400);
			}
		} @else if $name == light {
			&:focus {
				outline: 2px solid var(--color-blue800);
			}
		}
	}
}

.square:hover {
	border-radius: 50%;
	transform: scale(1.5);
}

.square:focus::after {
	mix-blend-mode: difference;
	content: "Copié";
	font-size: 12px;
}
</style>
