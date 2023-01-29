
use wasm_bindgen::prelude::*;
use crate::io::MouseButton;

#[wasm_bindgen]
extern {
    fn copperCreatePointer(element_id: &str) -> usize;

    fn copperPointerIsMouse(pointer: usize) -> bool;
    fn copperPointerPressing(pointer: usize, button: usize) -> bool;
    fn copperPointerMouseX(pointer: usize) -> f32;
    fn copperPointerMouseY(pointer: usize) -> f32;
    fn copperPointerTouchCount(pointer: usize) -> usize;
    fn copperPointerTouchX(pointer: usize, index: usize) -> f32;
    fn copperPointerTouchY(pointer: usize, index: usize) -> f32;

    fn copperDropPointer(pointer: usize);
}

fn get_mouse_button_js(button: MouseButton) -> usize {
    match button {
        MouseButton::Left => 0,
        MouseButton::Scrollwheel => 1,
        MouseButton::Right => 2,
    }
}

pub struct Pointer {
    pointer: usize
}

impl Pointer {
    pub fn from_document() -> Pointer { Pointer { pointer: copperCreatePointer("") } }
    pub fn from_element(element_id: &str) -> Pointer { Pointer { pointer: copperCreatePointer(element_id) } }

    pub fn is_mouse(&self) -> bool { copperPointerIsMouse(self.pointer) }
    pub fn pressing(&self, button: MouseButton) -> bool { copperPointerPressing(self.pointer, get_mouse_button_js(button)) }
    pub fn mouse_x(&self) -> f32 { copperPointerMouseX(self.pointer) }
    pub fn mouse_y(&self) -> f32 { copperPointerMouseY(self.pointer) }

    pub fn is_touch(&self) -> bool { !copperPointerIsMouse(self.pointer) }
    pub fn touches(&self) -> Vec<[f32; 2]> {
        let mut result: Vec<[f32; 2]> = vec![[0.0; 2]; copperPointerTouchCount(self.pointer)];
        for i in 0..result.len() {
            result[i] = [copperPointerTouchX(self.pointer, i), copperPointerTouchY(self.pointer, i)];
        }
        result
    }
}

impl Drop for Pointer {
    fn drop(&mut self) {
        copperDropPointer(self.pointer);
    }
}