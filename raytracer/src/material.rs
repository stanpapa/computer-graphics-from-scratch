use rand::Rng;

use crate::{
    color::Color,
    hittable::HitRecord,
    point3d::{Dot, Length, Normalize, Point3D},
    ray::Ray,
};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
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
            Material::Dielectric(d) => d.scatter(ray_in, hit_record),
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
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1. { fuzz } else { 1. },
        }
    }
}

fn reflect(incoming: Point3D, normal: Point3D) -> Point3D {
    incoming - 2.0 * incoming.dot(&normal) * normal
}

impl Scatterable for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = reflect(ray_in.direction.normalize(), hit_record.normal);
        let scattered = Ray::new(
            hit_record.point,
            reflected + self.fuzz * Point3D::random_unit(),
        );
        let attenuation = self.albedo;

        if scattered.direction.dot(&hit_record.normal) <= 0. {
            return None;
        }

        Some((scattered, attenuation))
    }
}

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub refraction_index: f64,
}

fn refract(uv: Point3D, normal: Point3D, eta_over_eta_prime: f64) -> Point3D {
    let cos_theta = (-uv).dot(&normal).min(1.0);
    let r_out_perpendicular = eta_over_eta_prime * (uv + cos_theta * normal);
    let r_out_parallel = -(1. - r_out_perpendicular.length_squared()).abs().sqrt() * normal;

    r_out_perpendicular + r_out_parallel
}

/// use Schlick's approximation for reflectance
fn reflectance(cos: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 *= r0;
    r0 + (1. - r0) * (1. - cos).powi(5)
}

impl Scatterable for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::white();
        let refraction_ratio = if hit_record.front_face {
            1. / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(&hit_record.normal).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let mut rng = rand::thread_rng();
        let direction = if refraction_ratio * sin_theta > 1.
            || reflectance(cos_theta, refraction_ratio) > rng.gen::<f64>()
        {
            // cannot refract
            reflect(unit_direction, hit_record.normal)
        } else {
            refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        Some((Ray::new(hit_record.point, direction), attenuation))
    }
}
