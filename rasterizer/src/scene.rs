use crate::{color::Color, line::Line, vec3::Vec3};

// use rand::Rng;

pub struct Scene {
    // image
    pub aspect_ratio: f64,
    pub width: usize,
    pub height: usize,
    // pub samples_per_pixel: usize,
    // pub depth_max: isize,
    // camera
    // pub camera: Camera,
    // world
    pub objects: Vec<Line>,
}

impl Default for Scene {
    fn default() -> Self {
        // image
        let aspect_ratio = 3. / 2.;
        let image_width: usize = 1200;
        let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
        // let samples_per_pixel = 500;
        // let depth_max = 50;

        // camera
        // let look_from = Point3D::new(13., 2., 3.);
        // let look_at = Point3D::new(0., 0., 0.);
        // let camera = Camera::new(
        //     look_from,
        //     look_at,
        //     Point3D::new(0., 1., 0.),
        //     20.,
        //     aspect_ratio,
        //     0.1,
        //     10.,
        // );

        Self {
            aspect_ratio,
            width: image_width,
            height: image_height,
            // samples_per_pixel,
            // depth_max,
            // camera,
            objects: vec![
                Line::new(
                    Vec3::new(-200., -100., 0.),
                    Vec3::new(240., 120., 0.),
                    Color::black(),
                ),
                Line::new(
                    Vec3::new(-50., -200., 0.),
                    Vec3::new(60., 240., 0.),
                    Color::black(),
                ),
            ],
        }
    }
}
