import("../pkg/index.js")
    .then(module => {
        let game = module.Game.new();

        function renderLoop(t) {
            game.update(t);
            requestAnimationFrame(renderLoop);
        }
        renderLoop(0);
    }).catch(console.error);
