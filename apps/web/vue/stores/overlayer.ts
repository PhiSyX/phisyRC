/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import { defineStore } from "pinia";
import { computed, nextTick, ref } from "vue";

import { is_empty } from "@phisyrc/std";
import { to_px } from "@phisyrc/css/houdini/unit";

// ---- //
// Type //
// ---- //

type Layer = {
	id: string | number;
	event: Event & {
		clientX: number;
		clientY: number;
	};
	dom_element: Element;
	/**
	 * Default = false
	 */
	centered: boolean;
	destroyable: "background" | "manual";
	before_destroy: (this: Layer) => void;
	after_destroy: () => void;
	style: {
		top: CSSUnitValue;
		right: CSSUnitValue;
		bottom: CSSUnitValue;
		left: CSSUnitValue;
		width: CSSUnitValue;
		height: CSSUnitValue;
	};
	mouse_position: Partial<{
		top: CSSUnitValue;
		left: CSSUnitValue;
	}>;
};

// -------- //
// Constant //
// -------- //

/// Classe CSS de mise en évidence d'un élément du DOM.
const LAYER_HL_CSS_CLASS = "layer@highlight";

/// Classe CSS de mise en évidence d'un élément du DOM ayant une position CSS
/// différente de `relative`.
const LAYER_HL_CSS_CLASS_ALT = "layer@highlight--alt";

const DEFAULT_DESTROYABLE_BEHAVIOR = "background";
const DEFAULT_CENTERED = false;
const NOOP = () => {
	// no operation
};

const MOUSE_POSITION_PADDING: u8 = 4;

function setup() {
	let list = ref<Map<Layer["id"], Layer>>(new Map());
	let doc_position_element = document.body.getBoundingClientRect();

	function create_layer(
		payload: Omit<
			Optional<
				Layer,
				"centered" | "destroyable" | "after_destroy" | "before_destroy"
			>,
			"style" | "mouse_position"
		>,
	) {
		let {
			dom_element,
			event,
			id,
			destroyable = DEFAULT_DESTROYABLE_BEHAVIOR,
			centered = DEFAULT_CENTERED,
			after_destroy = NOOP,
			before_destroy = NOOP,
		} = payload;

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
		nextTick(() => dom_element.classList.add(layer_css_class));

		let dom_position_element = dom_element.getBoundingClientRect();
		let style: Layer["style"] = {
			top: to_px(dom_position_element.top - MOUSE_POSITION_PADDING),
			right: to_px(dom_position_element.right + MOUSE_POSITION_PADDING),
			bottom: to_px(dom_position_element.bottom - MOUSE_POSITION_PADDING),
			left: to_px(dom_position_element.left - MOUSE_POSITION_PADDING),
			width: to_px(dom_position_element.width + MOUSE_POSITION_PADDING * 2),
			height: to_px(dom_position_element.height + MOUSE_POSITION_PADDING * 2),
		};

		let mouse_position: Layer["mouse_position"] = {};
		if (!centered) {
			let { clientX: deltaX, clientY: deltaY } = event;
			mouse_position["top"] = to_px(deltaY + MOUSE_POSITION_PADDING);
			mouse_position["left"] = to_px(deltaX + MOUSE_POSITION_PADDING);
		}

		create_layer_mut({
			id,
			event,
			dom_element,
			centered,
			destroyable,
			after_destroy,
			before_destroy,
			style,
			mouse_position,
		});
	}

	function create_layer_mut(payload: Layer) {
		list.value.set(payload.id, payload);
	}

	function destroy_layer(layer_id: Layer["id"]) {
		const layer = list.value.get(layer_id)!;
		layer.dom_element.classList.remove(LAYER_HL_CSS_CLASS);
		layer.dom_element.classList.remove(LAYER_HL_CSS_CLASS_ALT);
		layer.before_destroy();
		destroy_layer_mut(layer.id);
		nextTick(() => layer.after_destroy());
	}

	function destroy_layer_mut(layer_id: Layer["id"]) {
		list.value.delete(layer_id);
	}

	function destroy_layers(options: { force: bool } = { force: false }) {
		list.value.forEach((layer) => {
			if (options.force) {
				destroy_layer(layer.id);
				return;
			}

			if (layer.destroyable !== "background") {
				return;
			}
			destroy_layer(layer.id);
		});
	}

	function update_layer(layer_id: Layer["id"]) {
		const layer = list.value.get(layer_id)!;

		let dom_position_element = layer.dom_element.getBoundingClientRect();

		let style: Layer["style"] = {
			top: to_px(dom_position_element.top - MOUSE_POSITION_PADDING),
			right: to_px(dom_position_element.right + MOUSE_POSITION_PADDING),
			bottom: to_px(dom_position_element.bottom - MOUSE_POSITION_PADDING),
			left: to_px(dom_position_element.left - MOUSE_POSITION_PADDING),
			width: to_px(dom_position_element.width + MOUSE_POSITION_PADDING * 2),
			height: to_px(dom_position_element.height + MOUSE_POSITION_PADDING * 2),
		};

		update_layer_mut(layer_id, { style });
	}

	function update_layer_mut(
		layer_id: Layer["id"],
		payload: {
			style?: Layer["style"];
			mouse_position?: Layer["mouse_position"];
		},
	) {
		if (!list.value.has(layer_id)) {
			return;
		}
		let layer = list.value.get(layer_id)!;
		list.value.set(layer_id, { ...layer, ...payload });
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
		update_layer_mut,
		update_layers,
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
