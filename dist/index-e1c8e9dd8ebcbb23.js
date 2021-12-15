import { add_event, remove_event } from './snippets/dominator-b34ecf14d9383a43/inline0.js';

let wasm;

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

let heap_next = heap.length;

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function getObject(idx) { return heap[idx]; }

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

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_0.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_22(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h0bad979973224a45(arg0, arg1);
}

let stack_pointer = 32;

function addBorrowedObject(obj) {
    if (stack_pointer == 1) throw new Error('out of js stack');
    heap[--stack_pointer] = obj;
    return stack_pointer;
}
function __wbg_adapter_25(arg0, arg1, arg2) {
    try {
        wasm._dyn_core__ops__function__FnMut___A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h07ec4813b8f164a6(arg0, arg1, addBorrowedObject(arg2));
    } finally {
        heap[stack_pointer++] = undefined;
    }
}

function __wbg_adapter_28(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8b8d148b8c649fd4(arg0, arg1, addHeapObject(arg2));
}

/**
* @returns {Promise<void>}
*/
export function main_js() {
    wasm.main_js();
}

function getCachedStringFromWasm0(ptr, len) {
    if (ptr === 0) {
        return getObject(len);
    } else {
        return getStringFromWasm0(ptr, len);
    }
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

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

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    if (typeof input === 'undefined') {
        input = new URL('index-e1c8e9dd8ebcbb23_bg.wasm', import.meta.url);
    }
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_error_09919627ac0992f5 = function(arg0, arg1) {
        var v0 = getCachedStringFromWasm0(arg0, arg1);
    if (arg0 !== 0) { wasm.__wbindgen_free(arg0, arg1); }
    console.error(v0);
};
imports.wbg.__wbg_new_693216e109162396 = function() {
    var ret = new Error();
    return addHeapObject(ret);
};
imports.wbg.__wbg_stack_0ddaca5d1abfb52f = function(arg0, arg1) {
    var ret = getObject(arg1).stack;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};
imports.wbg.__wbindgen_is_undefined = function(arg0) {
    var ret = getObject(arg0) === undefined;
    return ret;
};
imports.wbg.__wbg_Window_f826a1dec163bacb = function(arg0) {
    var ret = getObject(arg0).Window;
    return addHeapObject(ret);
};
imports.wbg.__wbg_WorkerGlobalScope_967d186155183d38 = function(arg0) {
    var ret = getObject(arg0).WorkerGlobalScope;
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_cb_drop = function(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    var ret = false;
    return ret;
};
imports.wbg.__wbg_msCrypto_a2cdb043d2bfe57f = function(arg0) {
    var ret = getObject(arg0).msCrypto;
    return addHeapObject(ret);
};
imports.wbg.__wbg_crypto_98fc271021c7d2ad = function(arg0) {
    var ret = getObject(arg0).crypto;
    return addHeapObject(ret);
};
imports.wbg.__wbg_getRandomValues_98117e9a7e993920 = function() { return handleError(function (arg0, arg1) {
    getObject(arg0).getRandomValues(getObject(arg1));
}, arguments) };
imports.wbg.__wbg_modulerequire_3440a4bcf44437db = function() { return handleError(function (arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    var ret = module.require(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_randomFillSync_64cc7d048f228ca8 = function() { return handleError(function (arg0, arg1, arg2) {
    getObject(arg0).randomFillSync(getArrayU8FromWasm0(arg1, arg2));
}, arguments) };
imports.wbg.__wbg_process_2f24d6544ea7b200 = function(arg0) {
    var ret = getObject(arg0).process;
    return addHeapObject(ret);
};
imports.wbg.__wbg_versions_6164651e75405d4a = function(arg0) {
    var ret = getObject(arg0).versions;
    return addHeapObject(ret);
};
imports.wbg.__wbg_node_4b517d861cbcb3bc = function(arg0) {
    var ret = getObject(arg0).node;
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_is_object = function(arg0) {
    const val = getObject(arg0);
    var ret = typeof(val) === 'object' && val !== null;
    return ret;
};
imports.wbg.__wbindgen_is_string = function(arg0) {
    var ret = typeof(getObject(arg0)) === 'string';
    return ret;
};
imports.wbg.__wbg_addevent_66db3d105a9795d9 = function(arg0, arg1, arg2, arg3, arg4, arg5) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    add_event(getObject(arg0), v0, arg3 !== 0, arg4 !== 0, getObject(arg5));
};
imports.wbg.__wbg_removeevent_1e1f62952bbff820 = function(arg0, arg1, arg2, arg3, arg4) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    remove_event(getObject(arg0), v0, arg3 !== 0, getObject(arg4));
};
imports.wbg.__wbg_instanceof_Window_c4b70662a0d2c5ec = function(arg0) {
    var ret = getObject(arg0) instanceof Window;
    return ret;
};
imports.wbg.__wbg_document_1c64944725c0d81d = function(arg0) {
    var ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_clearTimeout_2c1ba0016d8bca41 = function(arg0, arg1) {
    getObject(arg0).clearTimeout(arg1);
};
imports.wbg.__wbg_setTimeout_df66d951b1726b78 = function() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
    return ret;
}, arguments) };
imports.wbg.__wbg_body_78ae4fd43b446013 = function(arg0) {
    var ret = getObject(arg0).body;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_head_d205ec9bd59f31a7 = function(arg0) {
    var ret = getObject(arg0).head;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_createComment_710531dc62f02c0c = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var ret = getObject(arg0).createComment(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_createElement_86c152812a141a62 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var ret = getObject(arg0).createElement(v0);
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_createTextNode_365db3bc3d0523ab = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var ret = getObject(arg0).createTextNode(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_add_f36d97e1d70d27b0 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).add(v0);
}, arguments) };
imports.wbg.__wbg_remove_89670e56a41482a8 = function() { return handleError(function (arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).remove(v0);
}, arguments) };
imports.wbg.__wbg_length_1ac7ec4672c36486 = function(arg0) {
    var ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_get_04c5c697dff89315 = function(arg0, arg1) {
    var ret = getObject(arg0)[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_cssRules_b9ce8ac851304351 = function() { return handleError(function (arg0) {
    var ret = getObject(arg0).cssRules;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_insertRule_e5808a97c0c5ecbe = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var ret = getObject(arg0).insertRule(v0, arg3 >>> 0);
    return ret;
}, arguments) };
imports.wbg.__wbg_style_fcef2171d2192afa = function(arg0) {
    var ret = getObject(arg0).style;
    return addHeapObject(ret);
};
imports.wbg.__wbg_classList_b666640fdfbcc8ab = function(arg0) {
    var ret = getObject(arg0).classList;
    return addHeapObject(ret);
};
imports.wbg.__wbg_instanceof_HtmlElement_df66c8b4a687aa43 = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLElement;
    return ret;
};
imports.wbg.__wbg_log_3445347661d4505e = function(arg0) {
    console.log(getObject(arg0));
};
imports.wbg.__wbg_clearTimeout_c1f246d6874c0679 = function(arg0, arg1) {
    getObject(arg0).clearTimeout(arg1);
};
imports.wbg.__wbg_setTimeout_5314850184d61a44 = function() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).setTimeout(getObject(arg1), arg2);
    return ret;
}, arguments) };
imports.wbg.__wbg_settype_f777a49b612d94f0 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).type = v0;
};
imports.wbg.__wbg_sheet_6b235d2f91d4d2c1 = function(arg0) {
    var ret = getObject(arg0).sheet;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};
imports.wbg.__wbg_appendChild_d318db34c4559916 = function() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).appendChild(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_insertBefore_5b314357408fbec1 = function() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).insertBefore(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_removeChild_d3ca7b53e537867e = function() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).removeChild(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_replaceChild_b59c728c04a8fb99 = function() { return handleError(function (arg0, arg1, arg2) {
    var ret = getObject(arg0).replaceChild(getObject(arg1), getObject(arg2));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_setdata_903bd12d7af0d5b5 = function(arg0, arg1, arg2) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    getObject(arg0).data = v0;
};
imports.wbg.__wbg_getPropertyValue_a3980b6b5e7fd8a9 = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    var ret = getObject(arg1).getPropertyValue(v0);
    var ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
}, arguments) };
imports.wbg.__wbg_removeProperty_150229ec3a3550ad = function() { return handleError(function (arg0, arg1, arg2, arg3) {
    var v0 = getCachedStringFromWasm0(arg2, arg3);
    var ret = getObject(arg1).removeProperty(v0);
    var ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len1;
    getInt32Memory0()[arg0 / 4 + 0] = ptr1;
}, arguments) };
imports.wbg.__wbg_setProperty_389eb1a127ad49a5 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    var v0 = getCachedStringFromWasm0(arg1, arg2);
    var v1 = getCachedStringFromWasm0(arg3, arg4);
    var v2 = getCachedStringFromWasm0(arg5, arg6);
    getObject(arg0).setProperty(v0, v1, v2);
}, arguments) };
imports.wbg.__wbg_newnoargs_be86524d73f67598 = function(arg0, arg1) {
    var v0 = getCachedStringFromWasm0(arg0, arg1);
    var ret = new Function(v0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_call_888d259a5fefc347 = function() { return handleError(function (arg0, arg1) {
    var ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_resolve_d23068002f584f22 = function(arg0) {
    var ret = Promise.resolve(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_then_2fcac196782070cc = function(arg0, arg1) {
    var ret = getObject(arg0).then(getObject(arg1));
    return addHeapObject(ret);
};
imports.wbg.__wbg_globalThis_3f735a5746d41fbd = function() { return handleError(function () {
    var ret = globalThis.globalThis;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_self_c6fbdfc2918d5e58 = function() { return handleError(function () {
    var ret = self.self;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_window_baec038b5ab35c54 = function() { return handleError(function () {
    var ret = window.window;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_global_1bc0b39582740e95 = function() { return handleError(function () {
    var ret = global.global;
    return addHeapObject(ret);
}, arguments) };
imports.wbg.__wbg_new_a7ce447f15ff496f = function(arg0) {
    var ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
};
imports.wbg.__wbg_newwithlength_929232475839a482 = function(arg0) {
    var ret = new Uint8Array(arg0 >>> 0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_subarray_8b658422a224f479 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};
imports.wbg.__wbg_length_1eb8fc608a0d4cdb = function(arg0) {
    var ret = getObject(arg0).length;
    return ret;
};
imports.wbg.__wbg_set_969ad0a60e51d320 = function(arg0, arg1, arg2) {
    getObject(arg0).set(getObject(arg1), arg2 >>> 0);
};
imports.wbg.__wbg_buffer_397eaa4d72ee94dd = function(arg0) {
    var ret = getObject(arg0).buffer;
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};
imports.wbg.__wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};
imports.wbg.__wbindgen_rethrow = function(arg0) {
    throw takeObject(arg0);
};
imports.wbg.__wbindgen_memory = function() {
    var ret = wasm.memory;
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper4401 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 186, __wbg_adapter_22);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper5231 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 225, __wbg_adapter_25);
    return addHeapObject(ret);
};
imports.wbg.__wbindgen_closure_wrapper5766 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 240, __wbg_adapter_28);
    return addHeapObject(ret);
};

if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
    input = fetch(input);
}



const { instance, module } = await load(await input, imports);

wasm = instance.exports;
init.__wbindgen_wasm_module = module;
wasm.__wbindgen_start();
return wasm;
}

export default init;

