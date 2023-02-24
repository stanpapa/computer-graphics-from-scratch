mod canvas;
mod color;
mod light;
mod point3d;
mod sphere;

use canvas::Canvas;
use color::Color;
use light::{Light, LightType};
use point3d::{Dot, Length, Point3D};
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
            y: -1.0,
            z: 3.0,
        },
        radius: 1.0,
        color: Color(255, 0, 0),
        specular: 500,
    },
    Sphere {
        center: Point3D {
            x: 2.0,
            y: 0.0,
            z: 4.0,
        },
        radius: 1.0,
        color: Color(0, 0, 255),
        specular: 500,
    },
    Sphere {
        center: Point3D {
            x: -2.0,
            y: 0.0,
            z: 4.0,
        },
        radius: 1.0,
        color: Color(0, 255, 0),
        specular: 10,
    },
    Sphere {
        center: Point3D {
            x: 0.0,
            y: -5001.0,
            z: 0.0,
        },
        radius: 5000.0,
        color: Color(255, 255, 0),
        specular: 1000,
    },
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let camera = Point3D::new(0.0, 0.0, 0.0);

    let mut canvas = Canvas::new(600, 600);
    for x in (-(canvas.width as isize) / 2)..(canvas.width as isize / 2) {
        for y in (-(canvas.height as isize) / 2)..(canvas.height as isize / 2) {
            // direction of ray
            let direction = canvas.to_viewport(camera, x, y);

            // color
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

fn trace_ray(o: Point3D, d: Point3D, t_min: f64, t_max: f64) -> Color {
    let mut t_closest = std::f64::INFINITY;
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
            sphere.color * compute_lighting(p, normal, -1.0 * d, sphere.specular)
        }
        None => Color::default(),
    }
}

/// the ray equation
/// P = O + t(V - O) = O + tD
/// -inf < t < +inf
/// the sphere equation
/// < P - C, P - C> = r^2
///
/// ray meets sphere
/// <O + tD - C, O + tD - C> = r^2
/// CO = O - C
/// < CO + tD, CO +tD > = r^2
/// < CO, CO > + < tD, CO > + < CO, tD > + < tD, tD > = r^2
/// < tD, tD > + 2 < CO, tD > + < CO, CO > = r^2
/// t^2 < D, D > + 2 t < CO, D > + < CO, CO > - r^2 = 0
///
/// a = < D, D >
/// b = 2 < CO, D >
/// c = < CO, CO > - r^2
/// at^2 + bt + c = 0
/// {t_1, t_2} = ( -b +- sqrt(b^2 - 4ac) ) / 2a
fn intersect_ray_sphere(o: Point3D, d: Point3D, sphere: &Sphere) -> Option<(f64, f64)> {
    let oc = o - sphere.center;

    let a = d.dot(&d);
    let b = 2.0 * oc.dot(&d);
    let c = oc.dot(&oc) - sphere.radius * sphere.radius;

    let discriminant = b * b - 4.0 * a * c;

    // no intersection
    if discriminant < 0.0 {
        return None;
    }

    // compute solutions
    let t_1 = (-b + discriminant.sqrt()) / (2.0 * a);
    let t_2 = (-b - discriminant.sqrt()) / (2.0 * a);

    Some((t_1, t_2))
}

fn compute_lighting(point: Point3D, normal: Point3D, v: Point3D, s: i32) -> f64 {
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

                // diffuse
                let dot_nl = normal.dot(&l);
                if dot_nl > 0.0 {
                    intensity += light.intensity * dot_nl / (normal.length() * l.length());
                }

                // specular
                if s != -1 {
                    let r = 2.0 * normal * dot_nl - l;
                    let dot_rv = r.dot(&v);
                    if dot_rv > 0.0 {
                        intensity +=
                            light.intensity * f64::powi(dot_rv / (r.length() * v.length()), s);
                    }
                }
            }
        }
    }

    intensity
}

/// produce image of scene
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