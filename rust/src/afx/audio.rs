
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    fn copperCreateAudio(audio_url: &str) -> usize;

    fn copperAudioLoaded(pointer: usize) -> bool;

    fn copperDropAudio(pointer: usize);
}

pub struct Audio {
    pub(crate) pointer: usize
}

impl Audio {
    pub fn new(audio_url: &str) -> Audio { Audio { pointer: copperCreateAudio(audio_url) } }

    pub fn loaded(&self) -> bool { copperAudioLoaded(self.pointer) }
}

impl Drop for Audio {
    fn drop(&mut self) {
        copperDropAudio(self.pointer);
    }
}