use graphics::{
    camera::Camera,
    // canvas::Canvas,
    color::Color,
    hittable::{HitRecord, Hittable},
    // light::{Light, LightType},
    material::{Dielectric, Lambertian, Material, Metal, Scatterable},
    point3d::{Length, Normalize, Point3D},
    ray::Ray,
    sphere::Sphere,
};

use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};

use rand::Rng;

// use rayon::prelude::*;

use std::fs::File;

/// hard-code light sources for now
// const LIGHTS: [Light; 3] = [
//     Light {
//         typ: LightType::Ambient,
//         intensity: 0.2,
//         point: None,
//     },
//     Light {
//         typ: LightType::Point,
//         intensity: 0.6,
//         point: Some(Point3D::new_const(2.0, 1.0, 0.0)),
//     },
//     Light {
//         typ: LightType::Directional,
//         intensity: 0.2,
//         point: Some(Point3D::new_const(1.0, 4.0, 4.0)),
//     },
// ];

/// hard-code scene as a couple of spheres
// const SCENE: [Sphere; 4] = [
//     Sphere {
//         center: Point3D::new_const(0.0, -1.0, 3.0),
//         radius: 1.0,
//         color: Color(255., 0., 0.),
//         specular: 500,
//         reflective: 0.2,
//     },
//     Sphere {
//         center: Point3D::new_const(2.0, 0.0, 4.0),
//         radius: 1.0,
//         color: Color(0., 0., 255.),
//         specular: 500,
//         reflective: 0.3,
//     },
//     Sphere {
//         center: Point3D::new_const(-2.0, 0.0, 4.0),
//         radius: 1.0,
//         color: Color(0., 255., 0.),
//         specular: 10,
//         reflective: 0.4,
//     },
//     Sphere {
//         center: Point3D::new_const(0.0, -5001.0, 0.0),
//         radius: 5000.0,
//         color: Color(255., 255., 0.),
//         specular: 1000,
//         reflective: 0.5,
//     },
// ];
const SCENE: [Sphere; 5] = [
    Sphere {
        center: Point3D::new_const(0.0, -100.5, -1.0),
        radius: 100.0,
        material: Material::Lambertian(Lambertian {
            albedo: Color(0.8, 0.8, 0.0),
        }),
        // color: Color(0., 0., 255.),
        // specular: 500,
        // reflective: 0.3,
    },
    Sphere {
        center: Point3D::new_const(0.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Lambertian(Lambertian {
            albedo: Color(0.1, 0.2, 0.5),
        }),
        // color: Color(255., 0., 0.),
        // specular: 500,
        // reflective: 0.2,
    },
    Sphere {
        center: Point3D::new_const(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Dielectric(Dielectric {
            refraction_index: 1.5,
        }),
        // color: Color(255., 0., 0.),
        // specular: 500,
        // reflective: 0.2,
    },
    Sphere {
        center: Point3D::new_const(-1.0, 0.0, -1.0),
        radius: -0.4,
        material: Material::Dielectric(Dielectric {
            refraction_index: 1.5,
        }),
        // color: Color(255., 0., 0.),
        // specular: 500,
        // reflective: 0.2,
    },
    Sphere {
        center: Point3D::new_const(1.0, 0.0, -1.0),
        radius: 0.5,
        material: Material::Metal(Metal {
            albedo: Color(0.8, 0.6, 0.2),
            fuzz: 0.,
        }),
        // color: Color(255., 0., 0.),
        // specular: 500,
        // reflective: 0.2,
    },
];

/// Conceptually, an "infinitesimaly small" real number.
const EPSILON: f64 = 0.001;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let mut pixels = vec![0; image_width * image_height * 3];
    // let cols: Vec<(usize, &mut [u8])> = pixels
    //     .chunks_mut(image_height as usize * 3)
    //     .enumerate()
    //     .collect();

    // cols.into_par_iter().for_each(|(i, col)| {
    //     for j in image_height - 1..=0 {
    //         // println!("{}:  {:?}", row, band);
    //         let u = i as f64 / (f64::from(image_width) - 1.0);
    //         let v = j as f64 / (f64::from(image_height) - 1.0);
    //         let ray = Ray::new(
    //             origin,
    //             lower_left_corner + u * horizontal + v * vertical - origin,
    //         );

    //         let color = ray_color(&ray);

    //         col[i ]
    //     }
    // });

    // render
    let mut count = 0;
    let mut rng = rand::thread_rng();
    for j in (0..(image_height - 1)).rev() {
        for i in 0..image_width {
            let mut color = Color::black();

            for _s in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                let v = (j as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);

                let ray = camera.get_ray(u, v);

                color += ray_color(&ray, &random_scene(), depth_max);
                // println!("{:?}", color);
            }

            let bytes = color.to_bytes(samples_per_pixel);

            pixels[3 * count] = bytes[0];
            pixels[3 * count + 1] = bytes[1];
            pixels[3 * count + 2] = bytes[2];

            count += 1;
        }
    }

    // let mut canvas = Canvas::new(600, 600);

    // // let x_iter: Vec<_> = ((-(canvas.width as isize) / 2)..(canvas.width as isize / 2)).collect();
    // // let y_iter: Vec<_> = ((-(canvas.height as isize) / 2)..(canvas.height as isize / 2)).collect();
    // // (x_iter, y_iter).par_iter_mut().for_each(|(x, y)| {

    // for x in (-(canvas.width as isize) / 2)..(canvas.width as isize / 2) {
    //     for y in (-(canvas.height as isize) / 2)..(canvas.height as isize / 2) {
    //         // direction of ray
    //         let mut direction = canvas.to_viewport(x, y);

    //         // rotate direction
    //         direction.rotate(ROTATION);

    //         // color
    //         let color = trace_ray(CAMERA, direction, 1.0, std::f64::INFINITY, 3);

    //         canvas.put_pixel(x, y, color)
    //     }
    // }

    write_image("test.png", &pixels, (image_width, image_height))?;

    Ok(())
}

fn random_scene() -> Vec<Sphere> {
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

// fn trace_ray(o: Point3D, d: Point3D, t_min: f64, t_max: f64, recursion_depth: usize) -> Color {
//     let (sphere_closest, t_closest) = closest_intersection(o, d, t_min, t_max);

//     match sphere_closest {
//         Some(sphere) => {
//             // compute local color
//             let p = o + t_closest * d;
//             let normal = sphere.normal(p);
//             let color_local = sphere.color * compute_lighting(p, normal, -1.0 * d, sphere.specular);

//             // if we hit the recursion limit or the object is not reflective we're done
//             if recursion_depth <= 0 || sphere.reflective <= 0.0 {
//                 return color_local;
//             }

//             // compute reflected color
//             let reflected = reflect_ray(-1.0 * d, normal);
//             let reflected_color =
//                 trace_ray(p, reflected, 0.001, std::f64::INFINITY, recursion_depth - 1);

//             color_local * (1.0 - sphere.reflective) + reflected_color * sphere.reflective
//         }
//         None => Color::black(),
//     }
// }

// fn closest_intersection(o: Point3D, d: Point3D, t_min: f64, t_max: f64) -> (Option<Sphere>, f64) {
//     let mut t_closest = std::f64::INFINITY;
//     let mut sphere_closest = None;

//     for sphere in SCENE.iter() {
//         if let Some((t_1, t_2)) = intersect_ray_sphere(o, d, sphere) {
//             if t_1 < t_closest && t_1 > t_min && t_1 < t_max {
//                 t_closest = t_1;
//                 sphere_closest = Some(sphere.clone());
//             }
//             if t_2 < t_closest && t_2 > t_min && t_2 < t_max {
//                 t_closest = t_2;
//                 sphere_closest = Some(sphere.clone());
//             }
//         }
//     }

//     (sphere_closest, t_closest)
// }

// /// the ray equation
// /// P = O + t(V - O) = O + tD
// /// -inf < t < +inf
// /// the sphere equation
// /// < P - C, P - C> = r^2
// ///
// /// ray meets sphere
// /// <O + tD - C, O + tD - C> = r^2
// /// CO = O - C
// /// < CO + tD, CO +tD > = r^2
// /// < CO, CO > + < tD, CO > + < CO, tD > + < tD, tD > = r^2
// /// < tD, tD > + 2 < CO, tD > + < CO, CO > = r^2
// /// t^2 < D, D > + 2 t < CO, D > + < CO, CO > - r^2 = 0
// ///
// /// a = < D, D >
// /// b = 2 < CO, D >
// /// c = < CO, CO > - r^2
// /// at^2 + bt + c = 0
// /// {t_1, t_2} = ( -b +- sqrt(b^2 - 4ac) ) / 2a
// fn intersect_ray_sphere(o: Point3D, d: Point3D, sphere: &Sphere) -> Option<(f64, f64)> {
//     let oc = o - sphere.center;

//     let a = d.dot(&d);
//     let b = 2.0 * oc.dot(&d);
//     let c = oc.dot(&oc) - sphere.radius * sphere.radius;

//     let discriminant = b * b - 4.0 * a * c;

//     // no intersection
//     if discriminant < 0.0 {
//         return None;
//     }

//     // compute solutions
//     let t_1 = (-b + discriminant.sqrt()) / (2.0 * a);
//     let t_2 = (-b - discriminant.sqrt()) / (2.0 * a);

//     Some((t_1, t_2))
// }

// fn compute_lighting(point: Point3D, normal: Point3D, v: Point3D, s: i32) -> f64 {
//     let mut intensity = 0.0;

//     for light in LIGHTS {
//         match light.typ {
//             LightType::Ambient => intensity += light.intensity,
//             _ => {
//                 let (l, t_max) = if light.typ == LightType::Point {
//                     (light.point.unwrap() - point, 1.0)
//                 } else {
//                     (light.point.unwrap(), std::f64::INFINITY)
//                 };

//                 // shadow check
//                 let (sphere_shadow, _) = closest_intersection(point, l, EPS, t_max);
//                 if sphere_shadow.is_some() {
//                     continue;
//                 }

//                 // diffuse
//                 let dot_nl = normal.dot(&l);
//                 if dot_nl > 0.0 {
//                     intensity += light.intensity * dot_nl / (normal.length() * l.length());
//                 }

//                 // specular
//                 if s != -1 {
//                     let r = 2.0 * normal * dot_nl - l;
//                     let dot_rv = r.dot(&v);
//                     if dot_rv > 0.0 {
//                         intensity +=
//                             light.intensity * f64::powi(dot_rv / (r.length() * v.length()), s);
//                     }
//                 }
//             }
//         }
//     }

//     intensity
// }

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
