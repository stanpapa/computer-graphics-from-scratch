use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    point3d::{DotProduct, Length, Normalize, Point3D},
    ray::Ray,
};

#[derive(Clone, Copy)]
pub struct Sphere {
    pub center: Point3D, // center of sphere
    pub radius: f64,     // radius of sphere
    pub material: Material,
}

// P is any point on the sphere surface
// distance(P,C) = r
// | P - C | = r

impl Sphere {
    pub fn new(center: Point3D, radius: f64, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn normal(&self, p: Point3D) -> Point3D {
        let normal = p - self.center;

        // check if point is on sphere surface
        if normal.length() - self.radius > 1e-8 {
            panic!("Point p not on sphere surface");
        }

        normal.normalize()
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;

        let a = ray.direction.length_squared();
        let b_half = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = b_half * b_half - a * c;

        // no intersection
        if discriminant < 0.0 {
            return None;
        }

        // find the neaest root that lies in the acceptable range
        let d_sqrt = discriminant.sqrt();
        let mut root = (-b_half - d_sqrt) / a;
        if root < t_min || root > t_max {
            root = (-b_half + d_sqrt) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        // found a hit, so return the intersection
        let p = ray.at(root);
        let normal = (p - self.center) / self.radius;
        let front_face = ray.direction.dot(&normal) < 0.0;

        Some(HitRecord {
            point: p,
            normal: if front_face { normal } else { -normal },
            t: root,
            front_face,
            material: self.material,
        })
    }
}
