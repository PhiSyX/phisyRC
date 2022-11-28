/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

import type { Directive } from "vue";
import { createFocusTrap } from "focus-trap";

/// Directive trap focus.
///
/// Utilise la librairie [focus-trap]
const trap_focus: Directive = {
	mounted(el, _binding, _vnode, _prev_node) {
		let trap = createFocusTrap(el, {
			escapeDeactivates: false,
			clickOutsideDeactivates: true,
		});
		trap.activate();
	}
};

export default trap_focus;
