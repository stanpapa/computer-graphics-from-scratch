use crate::{
    point3d::{CrossProduct, Normalize, Point3D},
    ray::Ray,
};

pub struct Camera {
    origin: Point3D,
    lower_left_corner: Point3D,
    horizontal: Point3D,
    vertical: Point3D,

    // orthonormal basis vectors
    u: Point3D,
    v: Point3D,
    // w: Point3D,
    lens_radius: f64,
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
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        // construct orthonormal basis
        let w = (look_from - look_at).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = look_from - horizontal / 2. - vertical / 2. - focus_distance * w;

        Self {
            origin: look_from,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            // w,
            lens_radius: aperture / 2.,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let random_disk = self.lens_radius * Point3D::random_in_unit_disk();
        let offset = self.u * random_disk[0] + self.v * random_disk[1];

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
