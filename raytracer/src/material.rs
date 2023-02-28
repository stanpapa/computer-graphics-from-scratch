use crate::{
    color::Color,
    hittable::HitRecord,
    point3d::{Dot, Normalize, Point3D},
    ray::Ray,
};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Light,
}

pub trait Scatterable {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}

impl Scatterable for Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        match self {
            Material::Lambertian(l) => l.scatter(ray_in, hit_record),
            Material::Metal(m) => m.scatter(ray_in, hit_record),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatterable for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_record.normal + Point3D::random_unit();

        // catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.point, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

fn reflect_ray(incoming: Point3D, normal: Point3D) -> Point3D {
    incoming - 2.0 * incoming.dot(&normal) * normal
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect_ray(ray_in.direction.normalize(), hit_record.normal);
        let scattered = Ray::new(hit_record.point, reflected);
        let attenuation = self.albedo;

        if scattered.direction.dot(&hit_record.normal) <= 0. {
            return None;
        }

        Some((scattered, attenuation))
    }
}
