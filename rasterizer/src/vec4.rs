use std::ops::Index;

#[derive(Copy, Clone)]
pub struct Vec4 {
    p: [f32; 4],
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, h: f32) -> Self {
        Self { p: [x, y, z, h] }
    }
}

impl Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        &self.p[i]
    }
}
