use crate::{draw::Draw, scene::Scene};

use std::fs::File;

use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};

// use rayon::prelude::*;

/// Conceptually, an "infinitesimaly small" real number
// const EPSILON: f64 = 0.001;

pub fn render(scene: &Scene, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut pixels = vec![255; scene.width * scene.height * 3];

    scene.objects.iter().for_each(|object| {
        object.draw(
            &mut pixels,
            scene.width,
            scene.height,
            scene.viewport_size,
            scene.projection_plane_z,
        )
    });

    write_image(filename, &pixels, scene.width, scene.height)
        .expect("Failed to write image to file");

    Ok(())
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
