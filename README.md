# copper.rs
Copper.rs is a simple library for 2D games written in Rust targeting HTML5.

# Usage in a webpage
To use copper.rs in a webpage, first write your project code using the `copper.rs`-crate make it have `crate-type = ["cdylib", "rlib"]`, make it depend on `wasm-bindgen = "0.2"` and build it using `wasm-pack` for `--target web`.

(example for a `lib.rs`)
```rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn start() {
    // code to run on start here
}

#[wasm_bindgen]
pub fn frame() {
    // code to run on frame here
}
```

Then, when built, a directory with a `.wasm`-file and a `<crate-name>.js` (and other files) will be generated. Make this folder accessible to your webpage, as well as `copper.js` (download directly from the lastest release from the repository).

(example for your webpage)
```html
<!doctype html>
<html>
    <head>
        <script src="copper.js"></script> <!-- the downloaded copper.js -->
        <script>
            // load the generated game WASM and run it
            copperStartWasm("./pkg/<crate-name>.js"); // change "./pkg/<crate-name>.js" to the actual name of the file
        </script>
    </head>
    <!-- "touch-action: none;" disables multi-touch gestures -->
    <body style="margin: 0px; overflow: hidden; touch-action: none;">
        <!-- make the canvas fill the entire screen -->
        <canvas id="game_canvas" style="width: 100vw; height: 100vh;"></canvas>
    </body>
</html>
```
