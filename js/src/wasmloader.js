
let copperWASM;

function copperStartWasm(wasmpackLinkFile) {
    import(wasmpackLinkFile).then( (wasmLink) => {
        wasmLink.default().then(() => {
            if(typeof wasmLink.start !== "function") {
                throw new Error(`Provided WASM-module does not have a function called "start".
    Implementation in Rust (requires crate "wasm-bindgen"):
    ==================================================
    // needed only once at start of file
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    pub fn start() {
        // code to run on initialization here
    }
    ==================================================`);
            }
            if(typeof wasmLink.frame !== "function") {
                throw new Error(`Provided WASM-module does not have a function called "frame".
    Implementation in Rust (requires crate "wasm-bindgen"):
    ==================================================
    // needed only once at start of file
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    pub fn frame()
    {
        // code to run on every frame here
    }
    ==================================================`);
            }
            copperWASM = wasmLink;
            copperStart();
            console.info('%c[copper.rs] WebAssembly binary loaded successfully.', 'color: #bada55');
        });
    });
}