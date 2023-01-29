
use wasm_bindgen::prelude::*;
use crate::gfx::TextAlign;

#[wasm_bindgen]
extern {
    fn copperCreateCanvasSurface(element_id: &str) -> usize;
    fn copperCreateSizedSurface(width: usize, height: usize) -> usize;
    fn copperCreateImageSurface(image_url: &str) -> usize;

    fn copperSurfaceLoaded(pointer: usize) -> bool;
    fn copperSurfaceWidth(pointer: usize) -> usize;
    fn copperSurfaceHeight(pointer: usize) -> usize;
    fn copperSurfaceGetPixelR(pointer: usize, x: usize, y: usize) -> u8;
    fn copperSurfaceGetPixelG(pointer: usize, x: usize, y: usize) -> u8;
    fn copperSurfaceGetPixelB(pointer: usize, x: usize, y: usize) -> u8;
    fn copperSurfaceGetPixelA(pointer: usize, x: usize, y: usize) -> u8;
    fn copperSurfaceResize(pointer: usize, width: usize, height: usize) -> usize;

    fn copperSurfaceClearMode(point: usize);
    fn copperSurfaceDrawMode(point: usize);
    fn copperSurfaceDrawSubSurfaceRotated(pointer: usize, src_pointer: usize, a: u8, sx: usize, sy: usize, sw: usize, sh: usize, x: f32, y: f32, w: f32, h: f32, rx: f32, ry: f32, ra: f32);
    fn copperSurfaceDrawRectRotated(pointer: usize, r: u8, g: u8, b: u8, a: u8, x: f32, y: f32, w: f32, h: f32, rx: f32, ry: f32, ra: f32);
    fn copperSurfaceDrawOvalRotated(pointer: usize, r: u8, g: u8, b: u8, a: u8, x: f32, y: f32, w: f32, h: f32, rx: f32, ry: f32, ra: f32);
    fn copperSurfaceDrawLine(pointer: usize, r: u8, g: u8, b: u8, a: u8, x1: f32, y1: f32, x2: f32, y2: f32, w: f32);
    fn copperSurfaceDrawQuadraticBezier(pointer: usize, r: u8, g: u8, b: u8, a: u8, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, w: f32);
    fn copperSurfaceDrawCubicBezier(pointer: usize, r: u8, g: u8, b: u8, a: u8, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32, w: f32);
    fn copperSurfaceDrawTextRotated(pointer: usize, text: &str, font: &str, r: u8, g: u8, b: u8, a: u8, x: f32, y: f32, align: &str, h: f32, rx: f32, ry: f32, ra: f32);

    fn copperDropSurface(pointer: usize);
}

fn get_text_align_js(align: TextAlign) -> &'static str {
    match align {
        TextAlign::Left => "right",
        TextAlign::Center => "center",
        TextAlign::Right => "left",
    }
}

pub struct Surface {
    pointer: usize
}

impl Surface {
    pub fn from_canvas(element_id: &str) -> Surface { Surface { pointer: copperCreateCanvasSurface(element_id) } }
    pub fn from_size(width: usize, height: usize) -> Surface { Surface { pointer: copperCreateSizedSurface(width, height) } }
    pub fn from_image(image_url: &str) -> Surface { Surface { pointer: copperCreateImageSurface(image_url) } }

    pub fn loaded(&self) -> bool { copperSurfaceLoaded(self.pointer) }
    pub fn width(&self) -> usize { copperSurfaceWidth(self.pointer) }
    pub fn height(&self) -> usize { copperSurfaceHeight(self.pointer) }
    pub fn get_pixel(&self, x: usize, y: usize) -> [u8; 4] {
        [
            copperSurfaceGetPixelR(self.pointer, x, y),
            copperSurfaceGetPixelG(self.pointer, x, y),
            copperSurfaceGetPixelB(self.pointer, x, y),
            copperSurfaceGetPixelA(self.pointer, x, y)
        ]
    }
    pub fn resize(&self, width: usize, height: usize) -> Surface { Surface { pointer: copperSurfaceResize(self.pointer, width, height) } }

    pub fn draw_surface(&self, src: &Surface, alpha: u8, x: f32, y: f32, w: f32, h: f32) {
        copperSurfaceDrawSubSurfaceRotated(self.pointer, src.pointer, alpha, 0, 0, src.width(), src.height(), x, y, w, h, 0f32, 0f32, 0f32);
    }
    pub fn draw_surface_rotated(&self, src: &Surface, alpha: u8, x: f32, y: f32, w: f32, h: f32, rot_x: f32, rot_y: f32, rot_angle: f32) {
        copperSurfaceDrawSubSurfaceRotated(self.pointer, src.pointer, alpha, 0, 0, src.width(), src.height(), x, y, w, h, rot_x, rot_y, rot_angle);
    }
    pub fn draw_subsurface(&self, src: &Surface, alpha: u8, src_x: usize, src_y: usize, src_w: usize, src_h: usize, x: f32, y: f32, w: f32, h: f32) {
        copperSurfaceDrawSubSurfaceRotated(self.pointer, src.pointer, alpha, src_x, src_y, src_w, src_h, x, y, w, h, 0f32, 0f32, 0f32);
    }
    pub fn draw_subsurface_rotated(&self, src: &Surface, alpha: u8, src_x: usize, src_y: usize, src_w: usize, src_h: usize, x: f32, y: f32, w: f32, h: f32, rot_x: f32, rot_y: f32, rot_angle: f32) {
        copperSurfaceDrawSubSurfaceRotated(self.pointer, src.pointer, alpha, src_x, src_y, src_w, src_h, x, y, w, h, rot_x, rot_y, rot_angle);
    }

    pub fn draw_rect(&self, red: u8, green: u8, blue: u8, alpha: u8, x: f32, y: f32, w: f32, h: f32) {
        copperSurfaceDrawRectRotated(self.pointer, red, green, blue, alpha, x, y, w, h, 0f32, 0f32, 0f32);
    }
    pub fn draw_rect_rotated(&self, red: u8, green: u8, blue: u8, alpha: u8, x: f32, y: f32, w: f32, h: f32, rot_x: f32, rot_y: f32, rot_angle: f32) {
        copperSurfaceDrawRectRotated(self.pointer, red, green, blue, alpha, x, y, w, h, rot_x, rot_y, rot_angle);
    }
    pub fn clear_rect(&self, x: f32, y: f32, w: f32, h: f32) {
        copperSurfaceClearMode(self.pointer);
        self.draw_rect(0, 0, 0, 255, x, y, w, h);
        copperSurfaceDrawMode(self.pointer);
    }
    pub fn clear_rect_rotated(&self, x: f32, y: f32, w: f32, h: f32, rot_x: f32, rot_y: f32, rot_angle: f32) {
        copperSurfaceClearMode(self.pointer);
        self.draw_rect_rotated(0, 0, 0, 255, x, y, w, h, rot_x, rot_y, rot_angle);
        copperSurfaceDrawMode(self.pointer);
    }

    pub fn draw_oval(&self, red: u8, green: u8, blue: u8, alpha: u8, x: f32, y: f32, w: f32, h: f32) {
        copperSurfaceDrawOvalRotated(self.pointer, red, green, blue, alpha, x, y, w, h, 0f32, 0f32, 0f32);
    }
    pub fn draw_oval_rotated(&self, red: u8, green: u8, blue: u8, alpha: u8, x: f32, y: f32, w: f32, h: f32, rot_x: f32, rot_y: f32, rot_angle: f32) {
        copperSurfaceDrawOvalRotated(self.pointer, red, green, blue, alpha, x, y, w, h, rot_x, rot_y, rot_angle);
    }
    pub fn clear_oval(&self, x: f32, y: f32, w: f32, h: f32) {
        copperSurfaceClearMode(self.pointer);
        self.draw_oval(0, 0, 0, 255, x, y, w, h);
        copperSurfaceDrawMode(self.pointer);
    }
    pub fn clear_oval_rotated(&self, x: f32, y: f32, w: f32, h: f32, rot_x: f32, rot_y: f32, rot_angle: f32) {
        copperSurfaceClearMode(self.pointer);
        self.draw_oval_rotated(0, 0, 0, 255, x, y, w, h, rot_x, rot_y, rot_angle);
        copperSurfaceDrawMode(self.pointer);
    }

    pub fn draw_line(&self, red: u8, green: u8, blue: u8, alpha: u8, x1: f32, y1: f32, x2: f32, y2: f32, w: f32) {
        copperSurfaceDrawLine(self.pointer, red, green, blue, alpha, x1, y1, x2, y2, w);
    }
    pub fn clear_line(&self, x1: f32, y1: f32, x2: f32, y2: f32, w: f32) {
        copperSurfaceClearMode(self.pointer);
        self.draw_line(0, 0, 0, 255, x1, y1, x2, y2, w);
        copperSurfaceDrawMode(self.pointer);
    }

    pub fn draw_quadratic_bezier(&self, red: u8, green: u8, blue: u8, alpha: u8, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, w: f32) {
        copperSurfaceDrawQuadraticBezier(self.pointer, red, green, blue, alpha, x1, y1, x2, y2, x3, y3, w);
    }
    pub fn clear_quadratic_bezier(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, w: f32) {
        copperSurfaceClearMode(self.pointer);
        self.draw_quadratic_bezier(0, 0, 0, 255, x1, y1, x2, y2, x3, y3, w);
        copperSurfaceDrawMode(self.pointer);
    }

    pub fn draw_cubic_bezier(&self, red: u8, green: u8, blue: u8, alpha: u8, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32, w: f32) {
        copperSurfaceDrawCubicBezier(self.pointer, red, green, blue, alpha, x1, y1, x2, y2, x3, y3, x4, y4, w);
    }
    pub fn clear_cubic_bezier(&self, x1: f32, y1: f32, x2: f32, y2: f32, x3: f32, y3: f32, x4: f32, y4: f32, w: f32) {
        copperSurfaceClearMode(self.pointer);
        self.draw_cubic_bezier(0, 0, 0, 255, x1, y1, x2, y2, x3, y3, x4, y4, w);
        copperSurfaceDrawMode(self.pointer);
    }

    pub fn draw_text(&self, text: &str, font: &str, red: u8, green: u8, blue: u8, alpha: u8, x: f32, y: f32, align: TextAlign, h: f32) {
        copperSurfaceDrawTextRotated(self.pointer, text, font, red, green, blue, alpha, x, y, get_text_align_js(align), h, 0.0, 0.0, 0.0);
    }
    pub fn clear_text(&self, text: &str, font: &str, x: f32, y: f32, align: TextAlign, h: f32) {
        copperSurfaceClearMode(self.pointer);
        self.draw_text(text, font, 0, 0, 0, 255, x, y, align, h);
        copperSurfaceDrawMode(self.pointer);
    }
    pub fn draw_text_rotated(&self, text: &str, font: &str, red: u8, green: u8, blue: u8, alpha: u8, x: f32, y: f32, align: TextAlign, h: f32, rot_x: f32, rot_y: f32, rot_angle: f32) {
        copperSurfaceDrawTextRotated(self.pointer, text, font, red, green, blue, alpha, x, y, get_text_align_js(align), h, rot_x, rot_y, rot_angle);
    }
    pub fn clear_text_rotated(&self, text: &str, font: &str, x: f32, y: f32, align: TextAlign, h: f32, rot_x: f32, rot_y: f32, rot_angle: f32) {
        copperSurfaceClearMode(self.pointer);
        self.draw_text_rotated(text, font, 0, 0, 0, 255, x, y, align, h, rot_x, rot_y, rot_angle);
        copperSurfaceDrawMode(self.pointer);
    }

    pub fn fill(&self, red: u8, green: u8, blue: u8, alpha: u8) {
        self.draw_rect(red, green, blue, alpha, 0.0, 0.0, self.width() as f32, self.height() as f32);
    }
}

impl Drop for Surface {
    fn drop(&mut self) {
        copperDropSurface(self.pointer);
    }
}