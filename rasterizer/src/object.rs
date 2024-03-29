use crate::{color::Color, draw::Draw, vec3::Vec3, vec4::Vec4};

pub enum Object {
    Line(Line),
    WireframeTriangle(WireframeTriangle),
    FilledTriangle(FilledTriangle),
    ShadedTriangle(ShadedTriangle),
    WireframeCube(WireframeCube),
    WireframeObject(WireframeObject),
}

impl Draw for Object {
    fn draw(
        &self,
        pixels: &mut [u8],
        width: usize,
        height: usize,
        viewport_size: usize,
        projection_plane_z: f32,
    ) {
        match self {
            Object::Line(l) => l.draw(pixels, width, height, viewport_size, projection_plane_z),
            Object::WireframeTriangle(t) => {
                t.draw(pixels, width, height, viewport_size, projection_plane_z)
            }
            Object::FilledTriangle(ft) => {
                ft.draw(pixels, width, height, viewport_size, projection_plane_z)
            }
            Object::ShadedTriangle(st) => {
                st.draw(pixels, width, height, viewport_size, projection_plane_z)
            }
            Object::WireframeCube(wc) => {
                wc.draw(pixels, width, height, viewport_size, projection_plane_z)
            }
            Object::WireframeObject(wo) => {
                wo.draw(pixels, width, height, viewport_size, projection_plane_z)
            }
        }
    }
}

fn interpolate(i0: f32, i1: f32, d0: f32, d1: f32) -> Vec<f32> {
    let mut values = Vec::new();

    let a = (d1 - d0) / (i1 - i0);
    let mut d = d0;

    for _i in i0 as isize..=i1 as isize {
        values.push(d);

        d += a;
    }

    values
}

fn calc_pos(x: isize, y: isize, width: usize) -> usize {
    // println!("{x}, {y}, {width}");
    (y as usize * width + x as usize) * 3
}

fn set_pixel(pixels: &mut [u8], x: isize, y: isize, color: Color, width: usize, height: usize) {
    // shift real coordinates to canvas coordinates
    let x_corrected = width as isize / 2 + x;
    let y_corrected = height as isize / 2 - y;

    // check bounds
    if x_corrected < 0
        || x_corrected >= width as isize
        || y_corrected < 0
        || y_corrected >= height as isize
    {
        println!("{x} -> {x_corrected}, {y} -> {y_corrected}, {width}, {height}");
        return;
    }

    // colour pixels
    let pos = calc_pos(x_corrected, y_corrected, width);
    let c = color.to_bytes(1);
    pixels[pos] = c[0];
    pixels[pos + 1] = c[1];
    pixels[pos + 2] = c[2];
}

#[derive(Debug)]
pub struct Line {
    begin: Vec3,
    end: Vec3,
    color: Color,
}

impl Line {
    pub fn new(begin: Vec3, end: Vec3, color: Color) -> Self {
        Self { begin, end, color }
    }
}

impl Draw for Line {
    #[allow(unused_variables)]
    fn draw(
        &self,
        pixels: &mut [u8],
        width: usize,
        height: usize,
        viewport_size: usize,
        projection_plane_z: f32,
    ) {
        let dx = self.end[0] - self.begin[0];
        let dy = self.end[1] - self.begin[1];
        let c = self.color.to_bytes(1);

        if dx.abs() > dy.abs() {
            // line is horizontal-ish
            let (p0, p1) = if self.begin[0] > self.end[0] {
                (self.end, self.begin)
            } else {
                (self.begin, self.end)
            };

            let ys = interpolate(p0[0], p1[0], p0[1], p1[1]);
            let x0 = p0[0] as isize;
            let x1 = p1[0] as isize;

            for x in x0..=x1 {
                set_pixel(
                    pixels,
                    x,
                    ys[(x - x0) as usize] as isize,
                    self.color,
                    width,
                    height,
                );
            }
        } else {
            // line is vertical-ish
            let (p0, p1) = if self.begin[1] > self.end[1] {
                (self.end, self.begin)
            } else {
                (self.begin, self.end)
            };

            let xs = interpolate(p0[1], p1[1], p0[0], p1[0]);
            let y0 = p0[1] as isize;
            let y1 = p1[1] as isize;

            for y in y0..=y1 {
                set_pixel(
                    pixels,
                    xs[(y - y0) as usize] as isize,
                    y,
                    self.color,
                    width,
                    height,
                );
            }
        }
    }
}

pub struct WireframeTriangle {
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    color: Color,
}

impl WireframeTriangle {
    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, color: Color) -> Self {
        Self { p0, p1, p2, color }
    }
}

impl Draw for WireframeTriangle {
    fn draw(
        &self,
        pixels: &mut [u8],
        width: usize,
        height: usize,
        viewport_size: usize,
        projection_plane_z: f32,
    ) {
        Line::new(self.p0, self.p1, self.color).draw(
            pixels,
            width,
            height,
            viewport_size,
            projection_plane_z,
        );
        Line::new(self.p1, self.p2, self.color).draw(
            pixels,
            width,
            height,
            viewport_size,
            projection_plane_z,
        );
        Line::new(self.p2, self.p0, self.color).draw(
            pixels,
            width,
            height,
            viewport_size,
            projection_plane_z,
        );
    }
}

pub struct FilledTriangle {
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    color: Color,
}

impl FilledTriangle {
    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, color: Color) -> Self {
        let mut lowest = p0;
        let mut middle = p1;
        let mut highest = p2;

        // sort triangle vertices, so that y0 <= y1 <=y2
        if lowest[1] > middle[1] {
            std::mem::swap(&mut lowest, &mut middle);
        }
        if middle[1] > highest[1] {
            std::mem::swap(&mut middle, &mut highest);
        }
        if lowest[1] > middle[1] {
            std::mem::swap(&mut lowest, &mut middle);
        }

        Self {
            p0: lowest,
            p1: middle,
            p2: highest,
            color,
        }
    }
}

impl Draw for FilledTriangle {
    #[allow(unused_variables)]
    fn draw(
        &self,
        pixels: &mut [u8],
        width: usize,
        height: usize,
        viewport_size: usize,
        projection_plane_z: f32,
    ) {
        // compute the x-coordinates of the triangle edges
        let mut x01 = interpolate(self.p0[1], self.p1[1], self.p0[0], self.p1[0]);
        let x12 = interpolate(self.p1[1], self.p2[1], self.p1[0], self.p2[0]);
        let x02 = interpolate(self.p0[1], self.p2[1], self.p0[0], self.p2[0]);

        // concatenate the short sides
        // remove overlapping x01 x12 value. Arbitrarily chosen last value of x01
        x01.pop();
        let x012 = [x01, x12].concat();

        // determine x_left and x_right
        let m = (x02.len() as f64 / 2.).floor() as usize;
        let (x_left, x_right) = if x02[m] < x012[m] {
            (x02, x012)
        } else {
            (x012, x02)
        };

        // draw horizontal segments
        let c = self.color.to_bytes(1);
        let y0 = self.p0[1] as isize;
        let y2 = self.p2[1] as isize;
        for y in y0..=y2 {
            let xl = x_left[(y - y0) as usize] as isize;
            let xr = x_right[(y - y0) as usize] as isize;
            for x in xl..=xr {
                set_pixel(pixels, x, y, self.color, width, height);
            }
        }
    }
}

pub struct ShadedTriangle {
    p0: Vec4,
    p1: Vec4,
    p2: Vec4,
    color: Color,
}

impl ShadedTriangle {
    pub fn new(p0: Vec4, p1: Vec4, p2: Vec4, color: Color) -> Self {
        let mut lowest = p0;
        let mut middle = p1;
        let mut highest = p2;

        // sort triangle vertices, so that y0 <= y1 <=y2
        if lowest[1] > middle[1] {
            std::mem::swap(&mut lowest, &mut middle);
        }
        if middle[1] > highest[1] {
            std::mem::swap(&mut middle, &mut highest);
        }
        if lowest[1] > middle[1] {
            std::mem::swap(&mut lowest, &mut middle);
        }

        Self {
            p0: lowest,
            p1: middle,
            p2: highest,
            color,
        }
    }
}

impl Draw for ShadedTriangle {
    #[allow(unused_variables)]
    fn draw(
        &self,
        pixels: &mut [u8],
        width: usize,
        height: usize,
        viewport_size: usize,
        projection_plane_z: f32,
    ) {
        // compute the x-coordinates of the triangle edges
        let mut x01 = interpolate(self.p0[1], self.p1[1], self.p0[0], self.p1[0]);
        let x12 = interpolate(self.p1[1], self.p2[1], self.p1[0], self.p2[0]);
        let x02 = interpolate(self.p0[1], self.p2[1], self.p0[0], self.p2[0]);

        // interpolate shading values
        let mut h01 = interpolate(self.p0[1], self.p1[1], self.p0[3], self.p1[3]);
        let h12 = interpolate(self.p1[1], self.p2[1], self.p1[3], self.p2[3]);
        let h02 = interpolate(self.p0[1], self.p2[1], self.p0[3], self.p2[3]);

        // concatenate the short sides
        // remove overlapping x01 x12 value. Arbitrarily chosen last value of x01
        x01.pop();
        let x012 = [x01, x12].concat();

        h01.pop();
        let h012 = [h01, h12].concat();

        // determine x_left and x_right
        let m = (x02.len() as f64 / 2.).floor() as usize;
        let (x_left, x_right, h_left, h_right) = if x02[m] < x012[m] {
            (x02, x012, h02, h012)
        } else {
            (x012, x02, h012, h02)
        };

        // draw horizontal segments
        // let c = self.color.to_bytes(1);
        let y0 = self.p0[1] as isize;
        let y2 = self.p2[1] as isize;
        for y in y0..=y2 {
            let xl = x_left[(y - y0) as usize];
            let xr = x_right[(y - y0) as usize];

            let h_segment = interpolate(
                xl,
                xr,
                h_left[(y - y0) as usize],
                h_right[(y - y0) as usize],
            );

            for x in xl as isize..=xr as isize {
                let c = self.color * h_segment[(x - xl as isize) as usize];
                set_pixel(pixels, x, y, c, width, height);
            }
        }
    }
}

#[allow(non_snake_case)]
pub struct WireframeCube {
    // 4 "front" vertices
    vAf: Vec3,
    vBf: Vec3,
    vCf: Vec3,
    vDf: Vec3,
    // 4 "back" vertices
    vAb: Vec3,
    vBb: Vec3,
    vCb: Vec3,
    vDb: Vec3,
}

impl Default for WireframeCube {
    fn default() -> Self {
        Self {
            vAf: Vec3::new(-2., -0.5, 5.),
            vBf: Vec3::new(-2., 0.5, 5.),
            vCf: Vec3::new(-1., 0.5, 5.),
            vDf: Vec3::new(-1., -0.5, 5.),
            vAb: Vec3::new(-2., -0.5, 6.),
            vBb: Vec3::new(-2., 0.5, 6.),
            vCb: Vec3::new(-1., 0.5, 6.),
            vDb: Vec3::new(-1., -0.5, 6.),
        }
    }
}

fn viewport_to_canvas(
    x: f32,
    y: f32,
    canvas_width: f32,
    canvas_height: f32,
    viewport_size: f32,
) -> Vec3 {
    Vec3::new(
        x * canvas_width / viewport_size,
        y * canvas_height / viewport_size,
        0.0,
    )
}

fn project_vertex(
    v: &Vec3,
    canvas_width: usize,
    canvas_height: usize,
    viewport_size: usize,
    d: f32,
) -> Vec3 {
    viewport_to_canvas(
        v[0] * d / v[2],
        v[1] * d / v[2],
        canvas_width as f32,
        canvas_height as f32,
        viewport_size as f32,
    )
}

impl Draw for WireframeCube {
    fn draw(
        &self,
        pixels: &mut [u8],
        width: usize,
        height: usize,
        viewport_size: usize,
        projection_plane_z: f32,
    ) {
        // draw front face
        Line::new(
            project_vertex(&self.vAf, width, height, viewport_size, projection_plane_z),
            project_vertex(&self.vBf, width, height, viewport_size, projection_plane_z),
            Color::blue(),
        )
        .draw(pixels, width, height, viewport_size, projection_plane_z);
        Line::new(
            project_vertex(&self.vBf, width, height, viewport_size, projection_plane_z),
            project_vertex(&self.vCf, width, height, viewport_size, projection_plane_z),
            Color::blue(),
        )
        .draw(pixels, width, height, viewport_size, projection_plane_z);
        Line::new(
            project_vertex(&self.vCf, width, height, viewport_size, projection_plane_z),
            project_vertex(&self.vDf, width, height, viewport_size, projection_plane_z),
            Color::blue(),
        )
        .draw(pixels, width, height, viewport_size, projection_plane_z);
        Line::new(
            project_vertex(&self.vDf, width, height, viewport_size, projection_plane_z),
            project_vertex(&self.vAf, width, height, viewport_size, projection_plane_z),
            Color::blue(),
        )
        .draw(pixels, width, height, viewport_size, projection_plane_z);

        // draw back face
        Line::new(
            project_vertex(&self.vAb, width, height, viewport_size, projection_plane_z),
            project_vertex(&self.vBb, width, height, viewport_size, projection_plane_z),
            Color::red(),
        )
        .draw(pixels, width, height, viewport_size, projection_plane_z);
        Line::new(
            project_vertex(&self.vBb, width, height, viewport_size, projection_plane_z),
            project_vertex(&self.vCb, width, height, viewport_size, projection_plane_z),
            Color::red(),
        )
        .draw(pixels, width, height, viewport_size, projection_plane_z);
        Line::new(
            project_vertex(&self.vCb, width, height, viewport_size, projection_plane_z),
            project_vertex(&self.vDb, width, height, viewport_size, projection_plane_z),
            Color::red(),
        )
        .draw(pixels, width, height, viewport_size, projection_plane_z);
        Line::new(
            project_vertex(&self.vDb, width, height, viewport_size, projection_plane_z),
            project_vertex(&self.vAb, width, height, viewport_size, projection_plane_z),
            Color::red(),
        )
        .draw(pixels, width, height, viewport_size, projection_plane_z);

        // draw front-to-back edges
        Line::new(
            project_vertex(&self.vAf, width, height, viewport_size, projection_plane_z),
            project_vertex(&self.vAb, width, height, viewport_size, projection_plane_z),
            Color::green(),
        )
        .draw(pixels, width, height, viewport_size, projection_plane_z);
        Line::new(
            project_vertex(&self.vBf, width, height, viewport_size, projection_plane_z),
            project_vertex(&self.vBb, width, height, viewport_size, projection_plane_z),
            Color::green(),
        )
        .draw(pixels, width, height, viewport_size, projection_plane_z);
        Line::new(
            project_vertex(&self.vCf, width, height, viewport_size, projection_plane_z),
            project_vertex(&self.vCb, width, height, viewport_size, projection_plane_z),
            Color::green(),
        )
        .draw(pixels, width, height, viewport_size, projection_plane_z);
        Line::new(
            project_vertex(&self.vDf, width, height, viewport_size, projection_plane_z),
            project_vertex(&self.vDb, width, height, viewport_size, projection_plane_z),
            Color::green(),
        )
        .draw(pixels, width, height, viewport_size, projection_plane_z);
    }
}

pub struct WireframeObject {
    vertices: Vec<Vec3>,
    triangles: Vec<(usize, usize, usize, Color)>,
}

impl WireframeObject {
    pub fn new_cube(position: Vec3) -> Self {
        let mut vertices = vec![
            Vec3::new(1., 1., 1.),
            Vec3::new(-1., 1., 1.),
            Vec3::new(-1., -1., 1.),
            Vec3::new(1., -1., 1.),
            Vec3::new(1., 1., -1.),
            Vec3::new(-1., 1., -1.),
            Vec3::new(-1., -1., -1.),
            Vec3::new(1., -1., -1.),
        ];

        // translate vertices
        for v in &mut vertices {
            *v += position;
        }

        Self {
            vertices,
            triangles: vec![
                (0, 1, 2, Color::red()),
                (0, 2, 3, Color::red()),
                (4, 0, 3, Color::green()),
                (4, 3, 7, Color::green()),
                (5, 4, 7, Color::blue()),
                (5, 7, 6, Color::blue()),
                (1, 5, 6, Color::yellow()),
                (1, 6, 2, Color::yellow()),
                (4, 5, 1, Color::purple()),
                (4, 1, 0, Color::purple()),
                (2, 6, 7, Color::cyan()),
                (2, 7, 3, Color::cyan()),
            ],
        }
    }
}

impl Draw for WireframeObject {
    fn draw(
        &self,
        pixels: &mut [u8],
        width: usize,
        height: usize,
        viewport_size: usize,
        projection_plane_z: f32,
    ) {
        // project vertices
        let projected: Vec<Vec3> = self
            .vertices
            .iter()
            .map(|v| project_vertex(v, width, height, viewport_size, projection_plane_z))
            .collect();

        // construct and draw projected triangles
        for (v0, v1, v2, color) in self.triangles.iter() {
            WireframeTriangle::new(projected[*v0], projected[*v1], projected[*v2], *color).draw(
                pixels,
                width,
                height,
                viewport_size,
                projection_plane_z,
            );
        }
    }
}
