use crate::color::Color;
use crate::point3d::Point3D;

use std::collections::HashMap;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: HashMap<(isize, isize), Color>,
}

impl Canvas {
    // initialize the canvas as a black screen
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut pixels = HashMap::new();
        for x in 0..width as isize {
            for y in 0..height as isize {
                pixels.insert((x, y), Color::white());
            }
        }

        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn to_viewport(&self, x: isize, y: isize) -> Point3D {
        // viewport dimensions
        let v_w = 1.0;
        let v_h = 1.0;
        let d = 1.0;

        // transform coordinates
        let v_x = x as f64 * v_w / self.width as f64;
        let v_y = y as f64 * v_h / self.height as f64;
        let v_z = d;
        let viewpoint = Point3D::new(v_x, v_y, v_z);

        // calculate vector from camera/origin to viewport
        viewpoint
    }

    pub fn put_pixel(&mut self, x: isize, y: isize, color: Color) {
        // tranform from human to computer basis
        // s_x = width / 2 + width
        // s_y = height / 2 - height
        let s_x = self.width as isize / 2 + x;
        let s_y = self.height as isize / 2 - y - 1;

        // check if pixel is within bounds
        if s_x < 0 || s_x >= self.width as isize {
            return;
        }
        if s_y < 0 || s_y >= self.height as isize {
            return;
        }

        *self.pixels.get_mut(&(s_x, s_y)).unwrap() = color;
    }

    pub fn to_pixels(&self) -> Vec<u8> {
        let mut pixels = vec![255; 3 * self.width * self.height];
        let mut count = 0;
        for y in 0..self.height as isize {
            for x in 0..self.width as isize {
                match self.pixels.get(&(x, y)) {
                    Some(p) => {
                        pixels[3 * count] = p.0 as u8;
                        pixels[3 * count + 1] = p.1 as u8;
                        pixels[3 * count + 2] = p.2 as u8;
                    }
                    None => (),
                }
                count += 1;
            }
        }
        pixels
    }
}
