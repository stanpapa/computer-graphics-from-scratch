use crate::color::Color;
use crate::point3d::{Length, Normalize, Point3D};

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point3D, // center of sphere
    pub radius: f64,     // radius of sphere
    pub color: Color,
    pub specular: i32, // shininess
    pub reflective: f64,
}

// P is any point on the sphere surface
// distance(P,C) = r
// | P - C | = r

impl Sphere {
    // pub fn new(center: Point3D, radius: f64, color: Color) -> Self {
    //     Self {
    //         center,
    //         radius,
    //         color,
    //     }
    // }

    pub fn normal(&self, p: Point3D) -> Point3D {
        let normal = p - self.center;

        // check if point is on sphere surface
        if normal.length() - self.radius > 1e-8 {
            panic!("Point p not on sphere surface");
        }

        normal.normalize()
    }
}
