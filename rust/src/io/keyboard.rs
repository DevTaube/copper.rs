
use wasm_bindgen::prelude::*;
use crate::io::Key;

#[wasm_bindgen]
extern {
    fn copperCreateKeyboard(element_id: &str) -> usize;

    fn copperKeyboardPressing(pointer: usize, key: usize) -> bool;

    fn copperDropKeyboard(pointer: usize);
}

fn get_key_js(key: Key) -> usize {
    match key {
        Key::Ctrl => 0,
        Key::Shift => 1,
        Key::Space => 2,
        Key::Backspace => 3,
        Key::Enter => 4,
        Key::Alt => 5,
        Key::A => 6, Key::B => 7, Key::C => 8, Key::D => 9, Key::E => 10,
        Key::F => 11, Key::G => 12, Key::H => 13, Key::I => 14, Key::J => 15,
        Key::K => 16, Key::L => 17, Key::M => 18, Key::N => 19, Key::O => 20,
        Key::P => 21, Key::Q => 22, Key::R => 23, Key::S => 24, Key::T => 25,
        Key::U => 26, Key::V => 27, Key::W => 28, Key::X => 29, Key::Y => 30,
        Key::Z => 31,
        Key::ArrowUp => 32, Key::ArrowDown => 33, Key::ArrowLef => 34, Key::ArrowRight => 35,
        Key::D0 => 36, Key::D1 => 37, Key::D2 => 38, Key::D3 => 39, Key::D4 => 40,
        Key::D5 => 41, Key::D6 => 42, Key::D7 => 43, Key::D8 => 44, Key::D9 => 45,
        Key::F1 => 46, Key::F2 => 47, Key::F3 => 48, Key::F4 => 49, Key::F5 => 50,
        Key::F6 => 51, Key::F7 => 52, Key::F8 => 53, Key::F9 => 54, Key::F10 => 55,
        Key::F11 => 56, Key::F12 => 57,
        Key::Escape => 58,
    }
}

pub struct Keyboard {
    pointer: usize
}

impl Keyboard {
    pub fn from_document() -> Keyboard { Keyboard { pointer: copperCreateKeyboard("") } }
    pub fn from_element(element_id: &str) -> Keyboard { Keyboard { pointer: copperCreateKeyboard(element_id) } }

    pub fn pressing(&self, key: Key) -> bool { copperKeyboardPressing(self.pointer, get_key_js(key)) }
}

impl Drop for Keyboard {
    fn drop(&mut self) {
        copperDropKeyboard(self.pointer);
    }
}