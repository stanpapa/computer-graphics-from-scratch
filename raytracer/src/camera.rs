use crate::point3d::Point3D;

use std::default::Default;

pub struct Camera {
    origin: Point3D,

    /// viewport
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
}

/// 16:9 aspect ratio
impl Default for Camera {
    fn default() -> Self {
        Self {
            origin: Point3D::new(0.0, 0.0, 0.0),
            viewport_height: 2.0,
            viewport_width: 16.0 / 9.0 * 2.0,
            focal_length: 1.0,
        }
    }
}

impl Camera {
    pub fn new(
        origin: Point3D,
        viewport_height: f64,
        viewport_width: f64,
        focal_length: f64,
    ) -> Self {
        Self {
            origin,
            viewport_height,
            viewport_width,
            focal_length,
        }
    }
}
