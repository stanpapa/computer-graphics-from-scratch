use std::ops::Index;

#[derive(Copy, Clone)]
pub struct Vec4 {
    p: [f64; 4],
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, h: f64) -> Self {
        Self { p: [x, y, z, h] }
    }
}

impl Index<usize> for Vec4 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.p[i]
    }
}
