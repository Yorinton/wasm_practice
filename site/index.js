const js = import('../pkg/wasm_practice_bg.js');

js.then((js) => {
    js.greet('Kasu', 29);
});
