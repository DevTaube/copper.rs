
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn copperAudioListenerSetPosition(x: f32, y: f32, z: f32);
    fn copperAudioListenerSetOrientation(lx: f32, ly: f32, lz: f32, ux: f32, uy: f32, uz: f32);
}

pub struct AudioListener;

impl AudioListener {
    pub fn set_position(x: f32, y: f32, z: f32) { copperAudioListenerSetPosition(x, y, z); }
    pub fn set_orientation(look_x: f32, look_y: f32, look_z: f32, up_x: f32, up_y: f32, up_z: f32) { copperAudioListenerSetOrientation(look_x, look_y, look_z, up_x, up_y, up_z); }
}