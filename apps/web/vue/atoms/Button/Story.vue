<script lang="ts" setup>
import { ref } from "vue";

import IconAdd from "../Icons/IconAdd.vue";

import Button from "./Button.vue";

let toggle_state = ref(false);
</script>

<template>
	<section class="[ flex gap=4 p=1 ]">
		<Button>Simple bouton</Button>

		<Button id="toggle_state" v-model:toggle="toggle_state">
			Avec état : {{ toggle_state }}
		</Button>

		<p>
			Bouton icône: <Button> <IconAdd /> </Button>
		</p>
	</section>
</template>
