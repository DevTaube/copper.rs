
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn copperDeltaTime() -> f64;
    fn copperLog(text: String);
}

pub struct Copper;

impl Copper {
    pub fn log(text: String) { copperLog(text); }
    pub fn delta_time() -> f64 { copperDeltaTime() }
}