mod canvas;
mod color;
mod light;
mod point3d;
mod sphere;

use canvas::Canvas;
use color::Color;
use light::{Light, LightType};
use point3d::{DotProduct, Length, Point3D};
use sphere::Sphere;

use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};

use std::fs::File;

/// hard-code light sources for now
const LIGHTS: [Light; 3] = [
    Light {
        typ: LightType::Ambient,
        intensity: 0.2,
        point: None,
    },
    Light {
        typ: LightType::Point,
        intensity: 0.6,
        point: Some(Point3D {
            x: 2.0,
            y: 1.0,
            z: 0.0,
        }),
    },
    Light {
        typ: LightType::Directional,
        intensity: 0.2,
        point: Some(Point3D {
            x: 1.0,
            y: 4.0,
            z: 4.0,
        }),
    },
];

/// hard-code scene as a couple of spheres
const SCENE: [Sphere; 4] = [
    Sphere {
        center: Point3D {
            x: 0.0,
            y: -1.,
            z: 3.0,
        },
        radius: 1.0,
        color: Color(255, 0, 0),
    },
    Sphere {
        center: Point3D {
            x: 2.0,
            y: 0.0,
            z: 4.0,
        },
        radius: 1.0,
        color: Color(0, 0, 255),
    },
    Sphere {
        center: Point3D {
            x: -2.0,
            y: 0.0,
            z: 4.0,
        },
        radius: 1.0,
        color: Color(0, 255, 0),
    },
    Sphere {
        center: Point3D {
            x: 0.0,
            y: -5001.0,
            z: 0.0,
        },
        radius: 5000.0,
        color: Color(255, 255, 0),
    },
];

// the sphere equation
// < P - C, P - C> = r^2

// ray meets sphere
// <O + tD - C, O + tD - C> = r^2
// CO = O - C
// < CO + tD, CO +tD > = r^2
// < CO, CO > + < tD, CO > + < CO, tD > + < tD, tD > = r^2
// < tD, tD > + 2 < CO, tD > + < CO, CO > = r^2
// t^2 < D, D > + 2 t < CO, D > + < CO, CO > - r^2 = 0

// a = < D, D >
// b = 2 < CO, D >
// c = < CO, CO > - r^2
// at^2 + bt + c = 0
// {t_1, t_2} = ( -b +- sqrt(b^2 - 4ac) ) / 2a
fn intersect_ray_sphere(o: Point3D, d: Point3D, sphere: &Sphere) -> Option<(f64, f64)> {
    let oc = o.clone() - sphere.center;

    let a = d.dot_product(&d);
    let b = 2.0 * oc.dot_product(&d);
    let c = oc.dot_product(&oc) - sphere.radius * sphere.radius;

    let discriminant = b * b - 4.0 * a * c;

    // no intersection
    if discriminant < 0.0 {
        return None;
    }

    //
    let t_1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let t_2 = (-b - discriminant.sqrt()) / (2.0 * a);

    Some((t_1, t_2))
}

fn trace_ray(o: Point3D, d: Point3D, t_min: f64, t_max: f64) -> Color {
    let mut t_closest = 100000.0; // really large number
    let mut sphere_closest = None;

    for sphere in SCENE.iter() {
        if let Some((t_1, t_2)) = intersect_ray_sphere(o, d, sphere) {
            if t_1 < t_closest && t_1 > t_min && t_1 < t_max {
                t_closest = t_1;
                sphere_closest = Some(sphere);
            }
            if t_2 < t_closest && t_2 > t_min && t_2 < t_max {
                t_closest = t_2;
                sphere_closest = Some(sphere);
            }
        }
    }

    match sphere_closest {
        Some(sphere) => {
            let p = o + t_closest * d;
            let normal = sphere.normal(p);

            // sphere.color
            sphere.color * compute_lighting(p, normal)
        }
        None => Color::default(),
    }
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    encoder
        .write_image(pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Rgb8)
        .unwrap();
    Ok(())
}

fn compute_lighting(point: Point3D, normal: Point3D) -> f64 {
    let mut intensity = 0.0;

    for light in LIGHTS {
        match light.typ {
            LightType::Ambient => intensity += light.intensity,
            _ => {
                let l = if light.typ == LightType::Point {
                    light.point.unwrap() - point
                } else {
                    light.point.unwrap()
                };

                let dot = normal.dot_product(&l);

                if dot > 0.0 {
                    intensity += light.intensity * dot / (normal.length() * l.length());
                }
            }
        }
    }

    intensity
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let camera = Point3D::new(0.0, 0.0, 0.0);

    let mut canvas = Canvas::new(600, 600);
    for x in (-(canvas.width as isize) / 2)..(canvas.width as isize / 2) {
        for y in (-(canvas.height as isize) / 2)..(canvas.height as isize / 2) {
            // the ray equation
            // P = O + t(V - O) = O + tD
            // -inf < t < +inf

            // direction of ray
            let direction = canvas.to_viewport(camera, x, y);

            let color = trace_ray(camera, direction, 1.0, 1000000.0);

            canvas.put_pixel(x, y, color)
        }
    }

    write_image(
        "test.png",
        &canvas.to_pixels(),
        (canvas.width, canvas.height),
    )?;

    Ok(())
}
