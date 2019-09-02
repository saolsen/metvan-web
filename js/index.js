import("../pkg/index.js")
    .then(module => {
        let platform = module.Platform.new();

        function onkey(ev, key, pressed) {
            platform.onkey(key, pressed);
            ev.preventDefault();
        }

        document.addEventListener('keydown', function (ev) { return onkey(ev, ev.keyCode, true); }, false);
        document.addEventListener('keyup', function (ev) { return onkey(ev, ev.keyCode, false); }, false);

        function renderLoop() {
            let t = window.performance.now();
            platform.update(t);
            requestAnimationFrame(renderLoop);
        }
        renderLoop();
    }).catch(console.error);
