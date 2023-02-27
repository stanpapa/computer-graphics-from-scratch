use crate::{point3d::Point3D, ray::Ray};

pub struct HitRecord {
    pub point: Point3D,
    pub normal: Point3D,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
