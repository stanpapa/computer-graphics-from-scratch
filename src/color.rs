use std::{
    default::Default,
    ops::{Add, Mul, Sub},
};

/// RGB support only
///
/// clamping to the [0-255] range
/// any value over 255 is 255, and any value below 0 is 0
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Color(pub u8, pub u8, pub u8);

impl Default for Color {
    fn default() -> Self {
        Self(255, 255, 255)
    }
}

impl Color {
    pub fn black() -> Color {
        Self(0, 0, 0)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let r = (u16::from(self.0) + u16::from(rhs.0))
            .max(u16::from(std::u8::MIN))
            .min(u16::from(std::u8::MAX)) as u8;
        let g = (u16::from(self.1) + u16::from(rhs.1))
            .max(u16::from(std::u8::MIN))
            .min(u16::from(std::u8::MAX)) as u8;
        let b = (u16::from(self.2) + u16::from(rhs.2))
            .max(u16::from(std::u8::MIN))
            .min(u16::from(std::u8::MAX)) as u8;

        Self(r, g, b)
    }
}

// impl AddAssign for Color {
//     fn add_assign(&mut self, rhs: Self) {
//         *self = Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2);
//     }
// }

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let r = (i16::from(self.0) - i16::from(rhs.0))
            .max(i16::from(std::u8::MIN))
            .min(i16::from(std::u8::MAX)) as u8;
        let g = (i16::from(self.1) - i16::from(rhs.1))
            .max(i16::from(std::u8::MIN))
            .min(i16::from(std::u8::MAX)) as u8;
        let b = (i16::from(self.2) - i16::from(rhs.2))
            .max(i16::from(std::u8::MIN))
            .min(i16::from(std::u8::MAX)) as u8;

        Self(r, g, b)
    }
}

// impl SubAssign for Color {
//     fn sub_assign(&mut self, rhs: Self) {
//         *self = Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2);
//     }
// }

// impl Mul<u8> for Color {
//     type Output = Self;

//     fn mul(self, rhs: u8) -> Self::Output {
//         Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
//     }
// }

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let r = (f64::from(self.0) * rhs)
            .max(f64::from(std::u8::MIN))
            .min(f64::from(std::u8::MAX))
            .round() as u8;
        let g = (f64::from(self.1) * rhs)
            .max(f64::from(std::u8::MIN))
            .min(f64::from(std::u8::MAX))
            .round() as u8;
        let b = (f64::from(self.2) * rhs)
            .max(f64::from(std::u8::MIN))
            .min(f64::from(std::u8::MAX))
            .round() as u8;

        Self(r, g, b)
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

// impl MulAssign<u8> for Color {
//     fn mul_assign(&mut self, rhs: u8) {
//         *self = Self(self.0 * rhs, self.1 * rhs, self.2 * rhs);
//     }
// }

// impl Display for Color {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!("");
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let c1 = Color(0, 0, 255);
        let c2 = Color(10, 10, 10);

        assert_eq!(c1 + c2, Color(10, 10, 255));
    }

    #[test]
    fn sub() {
        let c1 = Color(100, 100, 255);
        let c2 = Color(150, 10, 30);

        assert_eq!(c1 - c2, Color(0, 90, 225));
    }

    #[test]
    fn mul() {
        let c = Color(11, 19, 234);

        assert_eq!(1.3 * c, Color(14, 25, 255));
    }
}
