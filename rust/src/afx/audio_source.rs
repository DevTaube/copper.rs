
use wasm_bindgen::prelude::*;
use crate::afx::Audio;

#[wasm_bindgen]
extern {
    fn copperCreateAudioSource() -> usize;

    fn copperAudioSourcePlay(pointer: usize, src_pointer: usize);
    fn copperAudioSourceStop(pointer: usize);
    fn copperAudioSourceRepeat(pointer: usize, repeat: bool);
    fn copperAudioSourcePlaying(pointer: usize) -> bool;
    fn copperAudioSourceSetPosition(pointer: usize, x: f32, y: f32, z: f32);
    fn copperAudioSourceSetVolume(pointer: usize, volume: f32);
    fn copperAudioSourceSetPitch(pointer: usize, pitch: f32);

    fn copperDropAudioSource(pointer: usize);
}

pub struct AudioSource {
    pointer: usize
}

impl AudioSource {
    pub fn new() -> AudioSource { AudioSource { pointer: copperCreateAudioSource() } }

    pub fn play(&self, audio: &Audio) { copperAudioSourcePlay(self.pointer, audio.pointer); }
    pub fn stop(&self) { copperAudioSourceStop(self.pointer); }
    pub fn repeat(&self, repeat: bool) { copperAudioSourceRepeat(self.pointer, repeat); }
    pub fn playing(&self) -> bool { copperAudioSourcePlaying(self.pointer) }
    pub fn set_position(&self, x: f32, y: f32, z: f32) { copperAudioSourceSetPosition(self.pointer, x, y, z); }
    pub fn set_volume(&self, volume: f32) { copperAudioSourceSetVolume(self.pointer, volume); }
    pub fn set_pitch(&self, pitch: f32) { copperAudioSourceSetPitch(self.pointer, pitch); }
}

impl Drop for AudioSource {
    fn drop(&mut self) {
        copperDropAudioSource(self.pointer);
    }
}