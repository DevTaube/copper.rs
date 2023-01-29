
pub trait Vec {
    fn get_component_count(&self) -> usize;
    fn get_component(&self, index: usize) -> f32;
    fn set_component(&mut self, index: usize, value: f32);

    fn abs(&mut self) -> &mut Self {
        for i in 0..self.get_component_count() { self.set_component(i,
            self.get_component(i).abs()
        ); }
        self
    }
    fn add(&mut self, other: &impl Vec) -> &mut Self {
        for i in 0..self.get_component_count() { self.set_component(i,
            self.get_component(i) + other.get_component(i)
        ); }
        self
    }
    fn sub(&mut self, other: &impl Vec) -> &mut Self {
        for i in 0..self.get_component_count() { self.set_component(i,
            self.get_component(i) - other.get_component(i)
        ); }
        self
    }
    fn mul(&mut self, other: &impl Vec) -> &mut Self {
        for i in 0..self.get_component_count() { self.set_component(i,
            self.get_component(i) * other.get_component(i)
        ); }
        self
    }
    fn div(&mut self, other: &impl Vec) -> &mut Self {
        for i in 0..self.get_component_count() { self.set_component(i,
            self.get_component(i) / other.get_component(i)
        ); }
        self
    }
    fn modulo(&mut self, other: &impl Vec) -> &mut Self {
        for i in 0..self.get_component_count() { self.set_component(i,
            self.get_component(i) % other.get_component(i)
        ); }
        self
    }
    fn min(&mut self, other: &impl Vec) -> &mut Self {
        for i in 0..self.get_component_count() { self.set_component(i,
            self.get_component(i).min(other.get_component(i))
        ); }
        self
    }
    fn max(&mut self, other: &impl Vec) -> &mut Self {
        for i in 0..self.get_component_count() { self.set_component(i,
            self.get_component(i).max(other.get_component(i))
        ); }
        self
    }

    fn len(&self) -> f32 {
        let mut length_squared: f32 = 0.0;
        for i in 0..self.get_component_count() {
            length_squared += self.get_component(i).powi(2);
        }
        length_squared.sqrt()
    }
    fn neg(&mut self) -> &mut Self {
        for i in 0..self.get_component_count() { self.set_component(i,
            -self.get_component(i)
        ); }
        self
    }
    fn scale(&mut self, f: f32) -> &mut Self {
        for i in 0..self.get_component_count() { self.set_component(i,
            self.get_component(i) * f
        ); }
        self
    }
    fn normal(&mut self) -> &mut Self {
        let length: f32 = self.len();
        if length == 0.0 { return self; }
        self.scale(1.0 / length);
        self
    }

    fn dist(&self, other: &impl Vec) -> f32 {
        let mut dist_squared: f32 = 0.0;
        for i in 0..self.get_component_count() {
            dist_squared += (self.get_component(i) - other.get_component(i)).powi(2);
        }
        dist_squared.sqrt()
    }
    fn dot(&self, other: &impl Vec) -> f32 {
        let mut dot_product: f32 = 0.0;
        for i in 0..self.get_component_count() {
            dot_product += self.get_component(i) * other.get_component(i);
        }
        dot_product
    }
    fn angle(&self, other: &impl Vec) -> f32 {
        (self.dot(other) / (self.len() * other.len())).acos()
    }
}


#[derive(Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 { Vec2 { x, y } }
}
impl Vec for Vec2 {
    fn get_component_count(&self) -> usize { 2 }
    fn get_component(&self, index: usize) -> f32 {
        match index {
            0 => self.x,
            1 => self.y,
            _ => 0.0
        }
    }
    fn set_component(&mut self, index: usize, value: f32) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            _ => {}
        }
    }
}
impl ToString for Vec2 {
    fn to_string(&self) -> String {
        format!("[{}, {}]", self.x, self.y)
    }
}


#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 { Vec3 { x, y, z } }
}
impl Vec for Vec3 {
    fn get_component_count(&self) -> usize { 3 }
    fn get_component(&self, index: usize) -> f32 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => 0.0
        }
    }
    fn set_component(&mut self, index: usize, value: f32) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            _ => {}
        }
    }
}
impl ToString for Vec3 {
    fn to_string(&self) -> String {
        format!("[{}, {}, {}]", self.x, self.y, self.z)
    }
}


#[derive(Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32
}
impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 { Vec4 { x, y, z, w } }
}
impl Vec for Vec4 {
    fn get_component_count(&self) -> usize { 4 }
    fn get_component(&self, index: usize) -> f32 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => 0.0
        }
    }
    fn set_component(&mut self, index: usize, value: f32) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            3 => self.w = value,
            _ => {}
        }
    }
}
impl ToString for Vec4 {
    fn to_string(&self) -> String {
        format!("[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}