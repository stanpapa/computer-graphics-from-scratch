use std::ops::{Add, AddAssign, Div, Mul, Sub};

// use rand::Rng;

/// RGB support only
///
/// clamping to the [0-255] range
/// any value over 255 is 255, and any value below 0 is 0
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color(pub f32, pub f32, pub f32);

fn clamp(value: f32) -> f32 {
    // if value > 255. {
    //     return 255.;
    if value > 1. {
        return 1.;
    } else if value < 0. {
        return 0.;
    }

    value
}

impl Color {
    pub const fn white() -> Self {
        Self(1., 1., 1.)
    }

    pub const fn black() -> Self {
        Self(0., 0., 0.)
    }

    pub const fn red() -> Self {
        Self(1., 0., 0.)
    }

    pub const fn green() -> Self {
        Self(0., 1., 0.)
    }

    pub const fn blue() -> Self {
        Self(0., 0., 1.)
    }

    pub const fn yellow() -> Self {
        Self(1., 1., 0.)
    }

    pub const fn purple() -> Self {
        Self(1., 0., 1.)
    }

    pub const fn cyan() -> Self {
        Self(0., 1., 1.)
    }

    pub fn to_bytes(self, samples: usize) -> [u8; 3] {
        // sqrt is for gamma correction (= 2)
        // let scaled = (self / samples as f32).sqrt();
        let scaled = self / samples as f32;

        [
            (clamp(scaled.0) * 255.) as u8,
            (clamp(scaled.1) * 255.) as u8,
            (clamp(scaled.2) * 255.) as u8,
        ]
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl Div<f32> for Color {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let c1 = Color(0., 0., 255.);
        let c2 = Color(10., 10., 10.);

        assert_eq!((c1 + c2).to_bytes(1), [10, 10, 255]);
    }

    #[test]
    fn sub() {
        let c1 = Color(100., 100., 255.);
        let c2 = Color(150., 10., 30.);

        assert_eq!((c1 - c2).to_bytes(1), [0, 90, 225]);
    }

    #[test]
    fn mul() {
        let c = Color(11., 19., 234.);

        assert_eq!((1.3 * c).to_bytes(1), [14, 25, 255]);
    }
}
