import * as wasm from './phisyrc_wasm_bg.wasm';

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let WASM_VECTOR_LEN = 0;

let cachedUint8Memory0 = new Uint8Array();

function getUint8Memory0() {
    if (cachedUint8Memory0.byteLength === 0) {
        cachedUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8Memory0;
}

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachedInt32Memory0 = new Int32Array();

function getInt32Memory0() {
    if (cachedInt32Memory0.byteLength === 0) {
        cachedInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachedInt32Memory0;
}

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}
/**
* @param {string} log_level
*/
export function wasm_initialize_logger(log_level) {
    const ptr0 = passStringToWasm0(log_level, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.wasm_initialize_logger(ptr0, len0);
}

/**
* @param {Array<any>} arr
*/
export function logger_debug(arr) {
    wasm.logger_debug(addHeapObject(arr));
}

/**
* @param {Array<any>} arr
*/
export function logger_info(arr) {
    wasm.logger_info(addHeapObject(arr));
}

/**
* @param {Array<any>} arr
*/
export function logger_warn(arr) {
    wasm.logger_warn(addHeapObject(arr));
}

/**
* @param {Array<any>} arr
*/
export function logger_error(arr) {
    wasm.logger_error(addHeapObject(arr));
}

/**
* @param {Array<any>} arr
*/
export function logger_trace(arr) {
    wasm.logger_trace(addHeapObject(arr));
}

export function __wbindgen_string_get(arg0, arg1) {
    const obj = getObject(arg1);
    const ret = typeof(obj) === 'string' ? obj : undefined;
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export function __wbindgen_object_drop_ref(arg0) {
    takeObject(arg0);
};

export function __wbg_error_3fd2402933b2afd9(arg0, arg1) {
    console.error(getStringFromWasm0(arg0, arg1));
};

export function __wbindgen_string_new(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

export function __wbg_debug_f15cb542ea509609(arg0) {
    console.debug(getObject(arg0));
};

export function __wbg_debug_22d79d6c0bf3440a(arg0, arg1, arg2, arg3, arg4) {
    console.debug(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3), getObject(arg4));
};

export function __wbg_error_ef9a0be47931175f(arg0) {
    console.error(getObject(arg0));
};

export function __wbg_error_b313c7cd0ee7b82a(arg0, arg1, arg2, arg3, arg4) {
    console.error(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3), getObject(arg4));
};

export function __wbg_info_2874fdd5393f35ce(arg0) {
    console.info(getObject(arg0));
};

export function __wbg_info_dab2e234aa2a0123(arg0, arg1, arg2, arg3, arg4) {
    console.info(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3), getObject(arg4));
};

export function __wbg_trace_b57a30721e360d5d(arg0) {
    console.trace(getObject(arg0));
};

export function __wbg_trace_d171f6532114fca1(arg0, arg1, arg2, arg3, arg4) {
    console.trace(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3), getObject(arg4));
};

export function __wbg_warn_58110c4a199df084(arg0) {
    console.warn(getObject(arg0));
};

export function __wbg_warn_4b76257a73881993(arg0, arg1, arg2, arg3, arg4) {
    console.warn(getObject(arg0), getObject(arg1), getObject(arg2), getObject(arg3), getObject(arg4));
};

export function __wbg_get_57245cc7d7c7619d(arg0, arg1) {
    const ret = getObject(arg0)[arg1 >>> 0];
    return addHeapObject(ret);
};

export function __wbg_length_6e3bbe7c8bd4dbd8(arg0) {
    const ret = getObject(arg0).length;
    return ret;
};

export function __wbindgen_throw(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

