<script lang="ts" setup>
type Props = {
	name: string;
};

defineProps<Props>();
</script>

<template>
	<dialog open class="dialog@file-attachment [ pos-r p=2 border:radius=2 ]">
		<form
			method="post"
			enctype="multipart/form-data"
			class="[ size:full ]"
			@submit.prevent
		>
			<label
				:for="name"
				class="[ flex! align-i:center align-t:center size:full py=2 px=2 border:radius=4 ]"
			>
				<span style="font-size: 18px">Déposez ou cliquez</span>
				<span class="[ text:italic ]">pour envoyer une image</span>
			</label>

			<input
				:id="name"
				type="file"
				name="files[]"
				accept="image/*"
				class="[ pos-a:full ]"
			/>
		</form>
	</dialog>
</template>

<style lang="scss">
@import "design/app/organisms/editbox-dialog";
</style>
