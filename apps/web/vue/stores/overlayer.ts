/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import { defineStore } from "pinia";
import { computed, ref } from "vue";

import { is_empty } from "@phisyrc/std/lang/is_empty";
import { to_px } from "@phisyrc/css/houdini/unit";

// ---- //
// Type //
// ---- //

type Layer = {
	id: string | number;
	event: Event;
	dom_element: Element;
	destroyable: "background" | "manual";
	style: {
		top: CSSUnitValue;
		right: CSSUnitValue;
		bottom: CSSUnitValue;
		left: CSSUnitValue;
		width: CSSUnitValue;
		height: CSSUnitValue;
	}
};

// -------- //
// Constant //
// -------- //

/// Classe CSS de mise en évidence d'un élément du DOM.
const LAYER_HL_CSS_CLASS = "layer@highlight";

/// Classe CSS de mise en évidence d'un élément du DOM ayant une position CSS
/// différente de `relative`.
const LAYER_HL_CSS_CLASS_ALT = "layer@highlight--alt";

function setup() {
	let list = ref<Map<Layer["id"], Layer>>(new Map());

	function create_layer(
		payload: Omit<Optional<Layer, "destroyable">, "style">,
	) {
		let { dom_element, event, id, destroyable = "background" } = payload;
		if (
			dom_element.classList.contains(LAYER_HL_CSS_CLASS) ||
			dom_element.classList.contains(LAYER_HL_CSS_CLASS_ALT)
		) {
			return;
		}

		let computed_style = window.getComputedStyle(dom_element);
		let css_position_element = computed_style.position;

		let layer_css_class = LAYER_HL_CSS_CLASS;
		if (["absolute", "fixed"].includes(css_position_element)) {
			layer_css_class = LAYER_HL_CSS_CLASS_ALT;
		}
		dom_element.classList.add(layer_css_class);
		let doc_position_element = dom_element.getBoundingClientRect();
		let style: Layer["style"] = {
			top: to_px(doc_position_element.top),
			right: to_px(doc_position_element.right),
			bottom: to_px(doc_position_element.bottom),
			left: to_px(doc_position_element.left),
			width: to_px(doc_position_element.width),
			height: to_px(doc_position_element.height),
		};

		create_layer_mut({
			id,
			event,
			dom_element,
			destroyable,
			style
		});
	}

	function create_layer_mut(payload: Layer) {
		if (!payload.destroyable) {
			payload.destroyable = "background";
		}
		list.value.set(payload.id, payload);
	}

	function destroy_layer(layer_id: Layer["id"]) {
		const layer = list.value.get(layer_id)!;
		layer.dom_element.classList.remove(LAYER_HL_CSS_CLASS);
		layer.dom_element.classList.remove(LAYER_HL_CSS_CLASS_ALT);
		destroy_layer_mut(layer.id);
	}

	function destroy_layer_mut(layer_id: Layer["id"]) {
		list.value.delete(layer_id);
	}

	function destroy_layers() {
		list.value.forEach((layer) => {
			if (layer.destroyable !== "background") {
				return;
			}
			destroy_layer(layer.id);
		});
	}

	function update_layer(layer_id: Layer["id"]) {
		const layer = list.value.get(layer_id)!;

		let element_dom_position = layer.dom_element.getBoundingClientRect();

		let style: Layer["style"] = {
			top: to_px(element_dom_position.top - 5),
			right: to_px(element_dom_position.right),
			bottom: to_px(element_dom_position.bottom),
			left: to_px(element_dom_position.left),
			width: to_px(element_dom_position.width),
			height: to_px(element_dom_position.height + 8),
		};

		update_layer_mut(layer_id, style);
	}

	function update_layer_mut(layer_id: Layer["id"], style: Layer["style"]) {
		if (!list.value.has(layer_id)) {
			return;
		}
		let layer = list.value.get(layer_id)!;
		list.value.set(layer_id, { ...layer, style })
	}

	function update_layers() {
		list.value.forEach((layer) => update_layer(layer.id));
	}

	const layers = computed(() => list.value!);
	const has_layers = computed(() => is_empty(list.value));

	return {
		layers,
		has_layers,
		create_layer,
		destroy_layer,
		destroy_layers,
		update_layer,
		update_layers
	};
}

function use_overlayer_store() {
	return defineStore("overlayer", setup);
}

// ------ //
// export //
// ------ //

export type { Layer };

export { use_overlayer_store };
