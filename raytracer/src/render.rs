use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    material::Scatterable,
    point3d::Normalize,
    ray::Ray,
    scene::Scene,
    sphere::Sphere,
};

use std::fs::File;

use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};

use rand::Rng;

use rayon::prelude::*;

/// Conceptually, an "infinitesimaly small" real number
const EPSILON: f64 = 0.001;

pub fn render(scene: &Scene, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut pixels = vec![0; scene.width * scene.height * 3];
    let lines: Vec<(usize, &mut [u8])> = pixels.chunks_mut(scene.width * 3).enumerate().collect();

    // render
    lines.into_par_iter().for_each(|(y, line)| {
        render_line(line, scene, y);
    });

    write_image(filename, &pixels, scene.width, scene.height)
        .expect("Failed to write image to file");

    Ok(())
}

fn render_line(pixels: &mut [u8], scene: &Scene, y: usize) {
    let mut rng = rand::thread_rng();

    for x in 0..scene.width {
        let mut color = Color::black();

        for _s in 0..scene.samples_per_pixel {
            let u = (x as f64 + rng.gen::<f64>()) / (scene.width as f64 - 1.0);
            let v =
                (scene.height as f64 - y as f64 + rng.gen::<f64>()) / (scene.height as f64 - 1.0);

            let ray = scene.camera.get_ray(u, v);

            color += ray_color(&ray, &scene.objects, scene.depth_max);
        }

        let bytes = color.to_bytes(scene.samples_per_pixel);

        pixels[x * 3] = bytes[0];
        pixels[x * 3 + 1] = bytes[1];
        pixels[x * 3 + 2] = bytes[2];
    }
}

fn ray_color(ray: &Ray, scene: &[Sphere], depth: isize) -> Color {
    if depth <= 0 {
        return Color::black();
    }

    if let Some(hit_record) = hit_world(&scene, ray, EPSILON, f64::INFINITY) {
        if let Some((scattered, attenuation)) = hit_record.material.scatter(ray, &hit_record) {
            return attenuation * ray_color(&scattered, scene, depth - 1);
        }

        return Color::black();
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction[1] + 1.0);
    (1.0 - t as f32) * Color(1., 1., 1.) + t as f32 * Color(0.5, 0.7, 1.0)
}

fn hit_world(world: &[Sphere], ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let mut hit_record = None;
    let mut t_closest = t_max;

    for i in world.iter() {
        if let Some(hit) = i.hit(ray, t_min, t_closest) {
            t_closest = hit.t;
            hit_record = Some(hit);
        }
    }

    hit_record
}

/// produce image of scene
fn write_image(
    filename: &str,
    pixels: &[u8],
    width: usize,
    height: usize,
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    encoder
        .write_image(pixels, width as u32, height as u32, ColorType::Rgb8)
        .unwrap();
    Ok(())
}
