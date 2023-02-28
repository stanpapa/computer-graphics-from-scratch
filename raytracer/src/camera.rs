use crate::{
    point3d::{CrossProduct, Normalize, Point3D},
    ray::Ray,
};

use std::default::Default;

pub struct Camera {
    origin: Point3D,
    lower_left_corner: Point3D,

    horizontal: Point3D,
    vertical: Point3D,
    // focal_length: f64,
}

/// 16:9 aspect ratio
impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.;
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Point3D::new(0.0, 0.0, 0.0);
        let horizontal = Point3D::new(viewport_width, 0.0, 0.0);
        let vertical = Point3D::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Point3D::new(0.0, 0.0, focal_length);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
}

fn degrees_to_radians(vfov: f64) -> f64 {
    vfov * std::f64::consts::PI / 180.
}

impl Camera {
    pub fn new(
        look_from: Point3D,
        look_at: Point3D,
        vup: Point3D, // view up
        vfov: f64,    //vertical field-of-view in degrees
        aspect_ratio: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        // construct orthonormal basis
        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = look_from - horizontal / 2. - vertical / 2. - w;

        Self {
            origin: look_from,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
