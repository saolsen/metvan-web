const memory = new WebAssembly.Memory({ initial: 2 });

function js_reset_arrays() {
    var buf = memory.buffer;
    memory.U8 = new Uint8Array(buf);
    memory.S32 = new Int32Array(buf);
    memory.U32 = new Uint32Array(buf);
    memory.F32 = new Float32Array(buf);
    memory.F64 = new Float64Array(buf);
}

js_reset_arrays();

function getInt(ptr) {
    return memory.S32[ptr >> 2];
}

function getString(ptr) {
    var start = (ptr >>>= 0);
    while (memory.U8[ptr++]);
    getString.bytes = ptr - start;
    return String.fromCharCode.apply(null, memory.U8.subarray(start, ptr - 1));
}

function console_log(ptr, base, more) {
    let str = getString(ptr);
    console.log(str, base, more);
}

const importObj = {
    memoryBase: 0,
    env: {
        js_reset_arrays,
        memory,
    }
};

WebAssembly.instantiateStreaming(fetch('metvan.wasm'), importObj)
    .then(obj => {
        let exports = obj.instance.exports;
        window.exports = exports;
        console.log(exports);
        let test = exports.test;
        console.log("Test: ", test());
    });
