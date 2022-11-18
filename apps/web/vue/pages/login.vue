<script lang="ts">
export default {
	name: "Login",
};
</script>

<script setup lang="ts">
import Waves from "~vue/atoms/Waves/Waves.vue";

import LoginForm from "~vue/organisms/LoginForm/LoginForm.vue";
</script>

<template>
	<div id="login-page" class="[ size:full scroll:y scroll:hidden ]">
		<Waves />

		<div class="login@chat<button> [ size:full p=2 flex! gap=6 pos-r ]">
			<h1 class="[ heading=3 mx=1 mt=4 mb=0 ]">Accès direct au Chat</h1>

			<LoginForm id="login-form" />

			<button
				type="submit"
				form="login-form"
				class="[ w:full p=2 border:radius=1 ]"
			>
				Accéder au Chat
			</button>
		</div>
	</div>
</template>

<style lang="scss">
@import "design/app/pages/login";
</style>
