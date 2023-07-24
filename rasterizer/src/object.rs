use crate::{color::Color, draw::Draw, vec3::Vec3, vec4::Vec4};

pub enum Object {
    Line(Line),
    Triangle(Triangle),
    FilledTriangle(FilledTriangle),
    ShadedTriangle(ShadedTriangle),
}

impl Draw for Object {
    fn draw(&self, pixels: &mut [u8], width: usize, height: usize) {
        match self {
            Object::Line(l) => l.draw(pixels, width, height),
            Object::Triangle(t) => t.draw(pixels, width, height),
            Object::FilledTriangle(ft) => ft.draw(pixels, width, height),
            Object::ShadedTriangle(st) => st.draw(pixels, width, height),
        }
    }
}

fn interpolate(i0: f64, i1: f64, d0: f64, d1: f64) -> Vec<f64> {
    let mut values = Vec::new();

    let a = (d1 - d0) / (i1 - i0);
    let mut d = d0;

    for _i in i0 as isize..=i1 as isize {
        values.push(d);

        d += a;
    }

    values
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
    fn draw(&self, pixels: &mut [u8], width: usize, height: usize) {
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
                // todo: check for correctness
                let x_corrected = (x + width as isize / 2) as usize;
                let y_corrected = (height as isize / 2 - ys[(x - x0) as usize] as isize) as usize;
                let pos = y_corrected * width * 3 + x_corrected * 3;

                pixels[pos] = c[0];
                pixels[pos + 1] = c[1];
                pixels[pos + 2] = c[2];
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
                // todo: check for correctness
                let x_corrected = (xs[(y - y0) as usize] as isize + width as isize / 2) as usize;
                let y_corrected = (height as isize / 2 - y) as usize;
                let pos = y_corrected * width * 3 + x_corrected * 3;

                pixels[pos] = c[0];
                pixels[pos + 1] = c[1];
                pixels[pos + 2] = c[2];
            }
        }
    }
}

pub struct Triangle {
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    color: Color,
}

impl Triangle {
    pub fn new(p0: Vec3, p1: Vec3, p2: Vec3, color: Color) -> Self {
        Self { p0, p1, p2, color }
    }
}

impl Draw for Triangle {
    fn draw(&self, pixels: &mut [u8], width: usize, height: usize) {
        Line::new(self.p0, self.p1, self.color).draw(pixels, width, height);
        Line::new(self.p1, self.p2, self.color).draw(pixels, width, height);
        Line::new(self.p2, self.p0, self.color).draw(pixels, width, height);
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
    fn draw(&self, pixels: &mut [u8], width: usize, height: usize) {
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
            let y_corrected = (height as isize / 2 - y) as usize;

            let xl = x_left[(y - y0) as usize] as isize;
            let xr = x_right[(y - y0) as usize] as isize;
            for x in xl as isize..=xr as isize {
                let x_corrected = (x + width as isize / 2) as usize;
                let pos = (y_corrected * width + x_corrected) * 3;

                pixels[pos] = c[0];
                pixels[pos + 1] = c[1];
                pixels[pos + 2] = c[2];
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
    fn draw(&self, pixels: &mut [u8], width: usize, height: usize) {
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
            let y_corrected = (height as isize / 2 - y) as usize;

            let xl = x_left[(y - y0) as usize];
            let xr = x_right[(y - y0) as usize];

            let h_segment = interpolate(
                xl,
                xr,
                h_left[(y - y0) as usize],
                h_right[(y - y0) as usize],
            );

            for x in xl as isize..=xr as isize {
                let c = (self.color * h_segment[(x - xl as isize) as usize]).to_bytes(1);
                let x_corrected = (x + width as isize / 2) as usize;
                let pos = y_corrected * width * 3 + x_corrected * 3;

                pixels[pos] = c[0];
                pixels[pos + 1] = c[1];
                pixels[pos + 2] = c[2];
            }
        }
    }
}
