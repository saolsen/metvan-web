const typeSizes = {
    "i8": 1, "u8": 1,
    "i16": 2, "u16": 2,
    "i32": 4, "u32": 4,
    "f32": 4, "f64": 8
};

const typeArrays = {
    "i8": Int8Array, "u8": Uint8Array,
    "i16": Int16Array, "u16": Uint16Array,
    "i32": Int32Array, "u32": Uint32Array,
    "f32": Float32Array, "f64": Float64Array
};

const typeGetters = {
    "i8": "getInt8", "u8": "getUint8",
    "i16": "getInt16", "u16": "getUint16",
    "i32": "getInt32", "u32": "getUint32",
    "f32": "getFloat32", "f64": "getFloat64"
};

const typeSetters = {
    "i8": "setInt8", "u8": "setUint8",
    "i16": "setInt16", "u16": "setUint16",
    "i32": "setInt32", "u32": "setUint32",
    "f32": "setFloat32", "f64": "setFloat64"
};

function typeAlignment(type) {
    console.assert('kind' in type);
    if (typeof type.kind === 'object') {
        return type.kind.alignment;
    } else if (type.kind === 'array') {
        // Array Type
        let { of, count } = type;
        return typeAlignment(of);
    } else if (type.kind === 'ptr') {
        // Pointers are 32 bit in wasm32.
        return typeSizes['u32'];
    } else {
        // Primitive Type
        console.assert(type.kind in typeSizes, "unknown type: ", type.kind);
        return typeSizes[type.kind];
    }
}

function typeSize(type) {
    console.assert('kind' in type);
    if (typeof type.kind === 'object') {
        return type.kind.size;
    } else if (type.kind === 'array') {
        // Array Type
        let { of, count } = type;
        console.assert(count > 0, "count must be > 0");
        let elementSize = typeSize(of);
        return elementSize * count;
    } else if (type.kind === 'ptr') {
        // Pointers are 32 bit in wasm32.
        return typeSizes['u32'];
    } else {
        // Primitive Type
        console.assert(type.kind in typeSizes, "unknown type: ", type.kind);
        return typeSizes[type.kind];
    }
}

function layoutFields(fields) {
    let fieldOffsets = {};
    let fieldTypes = {};
    let offset = 0;

    let structAlignment = 1;

    for (let i = 0; i < fields.length; i++) {
        let { name, type } = fields[i];
        fieldTypes[name] = type;
        let alignment = typeAlignment(type);
        structAlignment = Math.max(structAlignment, alignment);
        let padding = -offset & (alignment - 1);
        offset += padding;
        fieldOffsets[name] = offset;
        offset += typeSize(type);
    }

    let padding = -offset & (structAlignment - 1);
    offset += padding;

    return {
        types: fieldTypes,
        offsets: fieldOffsets,
        alignment: structAlignment,
        size: offset
    }
}

function typeGetter(type, target, offset) {
    console.assert('kind' in type);
    if (typeof type.kind === 'object') {
        //return type.kind.size;
    } else if (type.kind === 'array') {
        let { of, count } = type;
        console.assert(count > 0, "count must be > 0");
        let elementSize = typeSize(of);
        return elementSize * count;
    } else {
        // @TODO: Pointer
        let getter = typeGetters[type.kind];
        return target.dataView[getter](offset, true);
    }
}

function typeProxy(storage, proxyType, offset) {
    console.assert('kind' in proxyType);
    console.assert(typeof proxyType.kind === 'object'
        || proxyType.kind === 'array'
        || proxyType.kind === 'ptr');
    if (proxyType.kind === 'array') {
        // Array
        let { of, count } = proxyType;
        console.assert('kind' in of);
        if (typeof of.kind === 'string') {
            // Array of primitive type.
            console.assert(of.kind in typeArrays);
            let arrayType = typeArrays[of.kind];
            return new arrayType(storage.buffer, offset, count); // @TODO: Test this out.
        } else {
            // Array of array or struct or pointers.
            let elementSize = typeSize(of);
            return new Proxy(storage, {
                get(target, i, proxy) {
                    if (i < 0 || i >= count) {
                        console.error("Error: out of range, count is :", count, "index: ", i);
                        return undefined;
                    }
                    // @TODO: array of pointers!
                    return typeProxy(storage, of, offset + elementSize * i);
                },
                set(target, name, value, proxy) {
                    return undefined;
                }
            });
        }
    } else if (proxyType.kind === 'ptr') {
        // Ptr
        return typeProxy
    } else {
        // Struct
        let structType = proxyType.kind;
        console.assert('types' in structType);
        console.assert('offsets' in structType);
        return new Proxy(storage, {
            get(target, name, proxy) {
                if (!(name in structType.types)) {
                    return undefined;
                }
                let type = structType.types[name];
                let fieldOffset = structType.offsets[name];
                if (typeof type.kind === 'object' || type.kind === 'array') {
                    return typeProxy(storage, type, offset + fieldOffset);
                } else if (type.kind === 'ptr') {
                    console.assert('to' in type);
                    let to = type.to;

                    let getter = typeGetters['u32'];
                    let addr = target.dataView[getter](offset + fieldOffset, true);
                    if (typeof to.kind === 'object' || to.kind === 'array' || to.kind === 'ptr') {
                        return typeProxy(storage, to, addr);
                    } else if (to.kind === 'void') {
                        return "<<native type>>"
                    } else {
                        let getter = typeGetters[to.kind];
                        return target.dataView[getter](addr, true);
                    }
                } else {
                    let getter = typeGetters[type.kind];
                    return target.dataView[getter](offset + fieldOffset, true);
                }
            },
            set(target, name, value, proxy) {
                if (!(name in structType.types)) {
                    return undefined;
                }
                let type = structType.types[name];
                let fieldOffset = structType.offsets[name];
                if (typeof type.kind === 'object' || type.kind === 'array') {
                    // Can't set a sub array or ptr, set individual items.
                    return undefined;
                } else if (type.kind === 'ptr') {
                    console.assert('to' in type);
                    let to = type.to;
                    let getter = typeGetters['u32'];
                    let addr = target.dataView[getter](offset + fieldOffset, true);
                    if (typeof to.kind === 'object' || to.kind === 'array' || to.kind === 'ptr') {
                        // Can't set a pointer to a struct or array or another pointer. Get it
                        // then you can set the values individually.
                        return undefined;
                    } else if (to.kind === 'void') {
                        return undefined;
                    } else {
                        let setter = typeSetters[to.kind];
                        return target.dataView[setter](addr, value, true);
                    }
                } else {
                    let setter = typeSetters[type.kind];
                    return target.dataView[setter](offset + fieldOffset, value, true);
                }
            }
        })
    }
}

function struct(name, fields) {
    let s = layoutFields(fields);
    s.name = name;
    return s;
}

const memory = new WebAssembly.Memory({ initial: 2 });
let platformPtr = -1;
let platform = null;

const inputFields = [
    { name: "up", type: { kind: "u32" } },
    { name: "down", type: { kind: "u32" } },
    { name: "left", type: { kind: "u32" } },
    { name: "right", type: { kind: "u32" } },
    { name: "jump", type: { kind: "u32" } },
    { name: "view_map", type: { kind: "u32" } },
];
const inputStruct = struct("Input", inputFields);

const renderRectFields = [
    { name: "world_center_x", type: { kind: "f64" } },
    { name: "world_center_y", type: { kind: "f64" } },
    { name: "world_extent_x", type: { kind: "f64" } },
    { name: "world_extent_y", type: { kind: "f64" } },
    { name: "color", type: { kind: "u32" } },
];
const renderRectStruct = struct("RenderRect", renderRectFields);

const platformFields = [
    { name: "magic", type: { kind: "u8" } },
    { name: "another_thing", type: { kind: "u32" } },
    { name: "pointer_to_foo", type: { kind: "ptr", to: { kind: "u32" } } },
    { name: "input", type: { kind: inputStruct } },
    { name: "render_rects", type: { kind: "array", of: { kind: renderRectStruct }, count: 128 } },
    { name: "render_rects_count", type: { kind: "u32" } },
    { name: "gamestate", type: { kind: "ptr", to: { kind: "void" } } },
];
const platformStruct = struct("Platform", platformFields);

function js_resetMemoryViews() {
    var buf = memory.buffer;
    memory.U8 = new Uint8Array(buf);
    memory.dataView = new DataView(buf, 0);

    if (platformPtr >= 0) {
        // Set up the platform proxy object.
        platform = typeProxy(memory, { kind: platformStruct }, platformPtr);
    }
}

js_resetMemoryViews();

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

let buf = [];
function js_putc(c) {
    let s = String.fromCharCode(c);
    buf.push(s);
    if (s === "\n") {
        console.log("[c]", buf.join(""));
        buf.length = 0;
    }
}

const importObj = {
    memoryBase: 0,
    env: {
        js_resetMemoryViews,
        js_putc,
        memory,
    }
};

const colors = [
    "pink",
    "#000000",
    "#262144",
    "#355278",
    "#60748a",
    "#898989",
    "#5aa8b2",
    "#91d9f3",
    "#ffffff",
    "#f4cd72",
    "#bfb588",
    "#c58843",
    "#9e5b47",
    "#5f4351",
    "#dc392d",
    "#6ea92c",
    "#1651dd",
];

WebAssembly.instantiateStreaming(fetch('metvan.wasm'), importObj)
    .then(obj => {
        let exports = obj.instance.exports;
        window.exports = exports;

        platformPtr = exports.init();
        js_resetMemoryViews();

        // record keypresses n shit urself.
        let canvas = document.getElementById("canvas");
        let ctx = canvas.getContext("2d");
        let dpr = window.devicePixelRatio;
        console.log("Dpr: ", dpr);

        let last_t = 0.0;
        let dt = 0.0;

        function onkey(ev, key, pressed) {
            let handled = true;
            switch (key) {
                case KeyboardEvent.DOM_VK_W: {
                    platform.input.up = pressed ? 1 : 0;
                } break;
                case KeyboardEvent.DOM_VK_A: {
                    platform.input.left = pressed ? 1 : 0;
                } break;
                case KeyboardEvent.DOM_VK_S: {
                    platform.input.down = pressed ? 1 : 0;
                } break;
                case KeyboardEvent.DOM_VK_D: {
                    platform.input.right = pressed ? 1 : 0;
                } break;
                case KeyboardEvent.DOM_VK_SPACE: {
                    platform.input.jump = pressed ? 1 : 0;
                } break;
                case KeyboardEvent.DOM_VK_M: {
                    platform.input.view_map = pressed ? 1 : 0;
                } break;
                default: {
                    handled = false;
                } break;
            }
            if (handled) {
                ev.preventDefault();
            }
        }

        document.addEventListener('keydown', function (ev) { return onkey(ev, ev.keyCode, true); }, false);
        document.addEventListener('keyup', function (ev) { return onkey(ev, ev.keyCode, false); }, false);

        function renderLoop() {
            let t = window.performance.now();
            //platform.update(t);
            exports.update_and_render(platformPtr);

            // Render in js.
            let displayWidth = canvas.clientWidth * dpr;
            let displayHeight = canvas.clientHeight * dpr;

            if (canvas.width != displayWidth || canvas.height != displayHeight) {
                canvas.width = displayWidth;
                canvas.height = displayHeight;
                ctx.scale(dpr, dpr);
                console.log("Resizing");
            }

            let width = self.canvas.clientWidth;
            let height = self.canvas.clientHeight;
            let aspect_ratio = width / height;

            // Tile Size
            let ts = width / 32.0;

            ctx.clearRect(0.0, 0.0, width, height);
            ctx.save();
            ctx.translate(0.0, height);

            ctx.fillRect(0, -ts, ts, ts);
            for (let i = 0; i < platform.render_rects_count; i++) {
                ctx.save();
                let rect = platform.render_rects[i];
                let color = colors[rect.color];
                let x = (rect.world_center_x - rect.world_extent_x) * ts;
                let y = (rect.world_center_y + rect.world_extent_y) * ts;
                let w = rect.world_extent_x * 2.0 * ts;
                let h = rect.world_extent_y * 2.0 * ts;
                ctx.fillStyle = color;
                ctx.fillRect(x, -y, w, h);
                ctx.restore();
            }

            ctx.restore();

            requestAnimationFrame(renderLoop);
        }
        renderLoop();

        // console.log("Magic");
        // console.log(platform.magic);
        // console.log("Another Thing");
        // console.log(platform.another_thing);
        // console.log("Pointer to foo");
        // console.log(platform.pointer_to_foo);
        // console.log("Pointer to gamestate");
        // console.log(platform.gamestate);
        // // this fuckin works!

        // fuck yeah, now we can set values.
        // platform.another_thing = 666;
        // platform.pointer_to_foo = 111;

        // console.log(platform.render_rects[0].world_center_x);
        // console.log(platform.render_rects[0].world_center_y);
    }).catch(console.error);;


// Things I need to get this working.
// Struct access from c.