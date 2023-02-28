use crate::{
    camera::Camera,
    color::Color,
    material::{Dielectric, Lambertian, Material, Metal},
    point3d::{Length, Point3D},
    sphere::Sphere,
};

use rand::Rng;

pub struct Scene {
    // image
    pub aspect_ratio: f64,
    pub width: usize,
    pub height: usize,
    pub samples_per_pixel: usize,
    pub depth_max: isize,
    // camera
    pub camera: Camera,
    // world
    pub objects: Vec<Sphere>,
}

impl Scene {
    pub fn new() -> Self {
        // image
        let aspect_ratio = 3. / 2.;
        let image_width: usize = 1200;
        let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
        let samples_per_pixel = 500;
        let depth_max = 50;

        // camera
        let look_from = Point3D::new(13., 2., 3.);
        let look_at = Point3D::new(0., 0., 0.);
        let camera = Camera::new(
            look_from,
            look_at,
            Point3D::new(0., 1., 0.),
            20.,
            aspect_ratio,
            0.1,
            10.,
        );

        Self {
            aspect_ratio,
            width: image_width,
            height: image_height,
            samples_per_pixel,
            depth_max,
            camera,
            objects: random_world(),
        }
    }
}

fn random_world() -> Vec<Sphere> {
    let mut scene = vec![
        Sphere {
            // ground
            center: Point3D::new(0., -1000., -1.),
            radius: 1000.,
            material: Material::Lambertian(Lambertian::new(Color(0.5, 0.5, 0.5))),
        },
        Sphere {
            center: Point3D::new(0., 1., 0.),
            radius: 1.,
            material: Material::Dielectric(Dielectric::new(1.5)),
        },
        Sphere {
            center: Point3D::new(-4., 1., 0.),
            radius: 1.,
            material: Material::Lambertian(Lambertian::new(Color(0.4, 0.2, 0.1))),
        },
        Sphere {
            center: Point3D::new(4., 1., 0.),
            radius: 1.,
            material: Material::Metal(Metal::new(Color(0.7, 0.6, 0.5), 0.)),
        },
    ];

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rng.gen::<f64>();
            let center = Point3D::new(
                f64::from(a) + 0.9 * rng.gen::<f64>(),
                0.2,
                f64::from(b) + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3D::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_material < 0.8 {
                    let albedo = Color::random() * Color::random();
                    // diffuse
                    scene.push(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian(Lambertian::new(albedo)),
                    ));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::random_range(0.5, 1.);
                    let fuzz = rng.gen_range(0.0..0.5);
                    scene.push(Sphere::new(
                        center,
                        0.2,
                        Material::Metal(Metal::new(albedo, fuzz)),
                    ));
                } else {
                    // glass
                    scene.push(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric(Dielectric::new(1.5)),
                    ));
                }
            }
        }
    }

    scene
}
