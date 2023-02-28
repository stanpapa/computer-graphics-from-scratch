use crate::{point3d::Point3D, ray::Ray};

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

impl Camera {
    //     pub fn new(
    //         origin: Point3D,
    //         viewport_height: f64,
    //         viewport_width: f64,
    //         focal_length: f64,
    //     ) -> Self {
    //         Self {
    //             origin,
    //             viewport_height,
    //             viewport_width,
    //             focal_length,
    //         }
    //     }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
