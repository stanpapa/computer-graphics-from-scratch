use crate::{
    color::Color,
    object::{
        FilledTriangle, Line, Object, ShadedTriangle, WireframeCube, WireframeObject,
        WireframeTriangle,
    },
    vec3::Vec3,
    vec4::Vec4,
};

pub struct Scene {
    // image
    pub aspect_ratio: f32,
    // canvas
    pub width: usize,
    pub height: usize,
    // camera
    pub viewport_size: usize,
    pub projection_plane_z: f32,
    // world
    pub objects: Vec<Object>,
}

impl Default for Scene {
    fn default() -> Self {
        // image
        let aspect_ratio = 3. / 2.;
        let image_width: usize = 1200;
        let image_height: usize = (image_width as f32 / aspect_ratio) as usize;

        Self {
            aspect_ratio,
            width: image_width,
            height: image_height,
            viewport_size: 1,
            projection_plane_z: 1.,
            objects: vec![
                Object::Line(Line::new(
                    Vec3::new(-200., -100., 0.),
                    Vec3::new(240., 120., 0.),
                    Color::black(),
                )),
                Object::Line(Line::new(
                    Vec3::new(-50., -200., 0.),
                    Vec3::new(60., 240., 0.),
                    Color::black(),
                )),
            ],
        }
    }
}

impl Scene {
    pub fn new_filled_triangle() -> Self {
        let aspect_ratio = 3. / 2.;
        let image_width: usize = 1200;
        let image_height: usize = (image_width as f32 / aspect_ratio) as usize;

        Self {
            aspect_ratio,
            width: image_width,
            height: image_height,
            viewport_size: 1,
            projection_plane_z: 1.,
            objects: vec![
                Object::FilledTriangle(FilledTriangle::new(
                    Vec3::new(-200., -250., 0.),
                    Vec3::new(200., 50., 0.),
                    Vec3::new(20., 250., 0.),
                    Color::green(),
                )),
                Object::WireframeTriangle(WireframeTriangle::new(
                    Vec3::new(-200., -250., 0.),
                    Vec3::new(200., 50., 0.),
                    Vec3::new(20., 250., 0.),
                    Color::black(),
                )),
            ],
        }
    }

    pub fn new_shaded_triangle() -> Self {
        let aspect_ratio = 3. / 2.;
        let image_width: usize = 1200;
        let image_height: usize = (image_width as f32 / aspect_ratio) as usize;

        Self {
            aspect_ratio,
            width: image_width,
            height: image_height,
            viewport_size: 1,
            projection_plane_z: 1.,
            objects: vec![Object::ShadedTriangle(ShadedTriangle::new(
                Vec4::new(-200., -250., 0., 0.3),
                Vec4::new(200., 50., 0., 0.1),
                Vec4::new(20., 250., 0., 1.),
                Color::green(),
            ))],
        }
    }

    pub fn new_wireframe_cube() -> Self {
        Self {
            aspect_ratio: 1.,
            width: 600,
            height: 600,
            viewport_size: 1,
            projection_plane_z: 1.,
            objects: vec![Object::WireframeCube(WireframeCube::default())],
        }
    }

    pub fn new_wireframe_object() -> Self {
        Self {
            aspect_ratio: 1.,
            width: 600,
            height: 600,
            viewport_size: 1,
            projection_plane_z: 1.,
            objects: vec![
                Object::WireframeObject(WireframeObject::new_cube(Vec3::new(-1.5, 0., 7.))),
                Object::WireframeObject(WireframeObject::new_cube(Vec3::new(1.25, 2., 7.5))),
            ],
        }
    }
}
