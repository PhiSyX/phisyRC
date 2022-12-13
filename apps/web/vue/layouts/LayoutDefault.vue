<script lang="ts">
export default {
	name: "LayoutDefault",
};
</script>

<script setup lang="ts">
import Overlayer from "~vue/organisms/Overlayer/Overlayer.vue";
</script>

<template>
	<RouterView />
	<Overlayer />
</template>

<style lang="scss">
@import "design/style.scss";
@import "design/generated.scss";
</style>
