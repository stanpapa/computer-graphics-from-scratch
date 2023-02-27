use crate::point3d::Point3D;

pub struct Ray {
    pub origin: Point3D,
    pub direction: Point3D,
}

impl Ray {
    pub fn new(origin: Point3D, direction: Point3D) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3D {
        self.origin + t * self.direction
    }
}
