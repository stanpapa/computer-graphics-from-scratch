use std::ops::{Add, Div, Index, Mul, Neg, Sub};

use rand::Rng;

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

#[derive(PartialEq, Copy, Clone)]
pub struct Point3D {
    p: [f64; 3],
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { p: [x, y, z] }
    }

    pub const fn new_const(x: f64, y: f64, z: f64) -> Self {
        Self { p: [x, y, z] }
    }

    fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();

        Point3D::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_unit() -> Self {
        loop {
            let p = Point3D::random(-1., 1.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    /// return true if the vector is near 0 in all dimensions
    pub fn near_zero(&self) -> bool {
        let threshold = 1e-8;
        self[0].abs() < threshold && self[1].abs() < threshold && self[2].abs() < threshold
    }
}

impl Index<usize> for Point3D {
    type Output = f64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.p[i]
    }
}

impl Normalize for Point3D {
    type Output = Self;

    fn normalize(&self) -> Self::Output {
        let n = self.length();

        Self::Output {
            p: [self[0] / n, self[1] / n, self[2] / n],
        }
    }
}

impl Rotate for Point3D {
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

impl Add for Point3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            p: [self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]],
        }
    }
}

impl Div<f64> for Point3D {
    type Output = Point3D;

    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            p: [self[0] / rhs, self[1] / rhs, self[2] / rhs],
        }
    }
}

impl Mul<Point3D> for f64 {
    type Output = Point3D;

    fn mul(self, rhs: Point3D) -> Self::Output {
        Self::Output {
            p: [self * rhs[0], self * rhs[1], self * rhs[2]],
        }
    }
}

impl Mul<f64> for Point3D {
    type Output = Point3D;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl Neg for Point3D {
    type Output = Point3D;

    fn neg(self) -> Self::Output {
        Self::Output {
            p: [-self.p[0], -self.p[1], -self.p[2]],
        }
    }
}

impl Sub for Point3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            p: [self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]],
        }
    }
}

impl DotProduct for Point3D {
    type Output = f64;

    fn dot(&self, rhs: &Self) -> Self::Output {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }
}

impl CrossProduct for Point3D {
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

impl Length for Point3D {
    type Output = f64;

    fn length(&self) -> Self::Output {
        self.dot(self).sqrt()
    }

    fn length_squared(&self) -> Self::Output {
        self.dot(self)
    }
}
