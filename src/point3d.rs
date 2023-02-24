use std::ops::{Add, Mul, Sub};

pub trait Dot {
    type Output;

    fn dot(&self, rhs: &Self) -> Self::Output;
}

pub trait Length {
    type Output;

    fn length(&self) -> Self::Output;
}

#[derive(PartialEq, Copy, Clone)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn normalize(&self) -> Self {
        let n = self.length();

        Self {
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }
}

impl Add for Point3D {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<Point3D> for f64 {
    type Output = Point3D;

    fn mul(self, rhs: Point3D) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<f64> for Point3D {
    type Output = Point3D;

    fn mul(self, rhs: f64) -> Self::Output {
        rhs * self
    }
}

impl Sub for Point3D {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Dot for Point3D {
    type Output = f64;

    fn dot(&self, rhs: &Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Length for Point3D {
    type Output = f64;

    fn length(&self) -> Self::Output {
        self.dot(self).sqrt()
    }
}
