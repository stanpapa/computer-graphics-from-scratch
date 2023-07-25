use std::ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Neg, Sub};

// use rand::Rng;

pub trait DotProduct {
    type Output;

    fn dot(&self, rhs: &Self) -> Self::Output;
}

pub trait CrossProduct {
    type Output;

    fn cross(&self, rhs: &Self) -> Self::Output;
}

pub trait Length {
    type Output;

    fn length(&self) -> Self::Output;
    fn length_squared(&self) -> Self::Output;
}

pub trait Normalize {
    type Output;

    fn normalize(&self) -> Self::Output;
}

pub trait Rotate {
    fn rotate(&mut self, rotation_matrix: [[f64; 3]; 3]);
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vec3 {
    p: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { p: [x, y, z] }
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.p[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.p[i]
    }
}

impl Normalize for Vec3 {
    type Output = Self;

    fn normalize(&self) -> Self::Output {
        let n = self.length();

        Self::Output {
            p: [self[0] / n, self[1] / n, self[2] / n],
        }
    }
}

impl Rotate for Vec3 {
    fn rotate(&mut self, rotation_matrix: [[f64; 3]; 3]) {
        let mut rotated = [0.0; 3];

        for i in 0..3 {
            for j in 0..3 {
                rotated[i] += rotation_matrix[i][j] * self[j];
            }
        }

        self.p = rotated;
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            p: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            p: [self[0] / rhs, self[1] / rhs, self[2] / rhs],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self::Output {
            p: [self * rhs[0], self * rhs[1], self * rhs[2]],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::Output {
            p: [-self.p[0], -self.p[1], -self.p[2]],
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            p: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }
}

impl DotProduct for Vec3 {
    type Output = f64;

    fn dot(&self, rhs: &Self) -> Self::Output {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }
}

impl CrossProduct for Vec3 {
    type Output = Self;

    fn cross(&self, rhs: &Self) -> Self::Output {
        Self::Output {
            p: [
                self[1] * rhs[2] - self[2] * rhs[1],
                self[2] * rhs[0] - self[0] * rhs[2],
                self[0] * rhs[1] - self[1] * rhs[0],
            ],
        }
    }
}

impl Length for Vec3 {
    type Output = f64;

    fn length(&self) -> Self::Output {
        self.dot(self).sqrt()
    }

    fn length_squared(&self) -> Self::Output {
        self.dot(self)
    }
}
