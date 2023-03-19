use crate::{color::Color, vec3::Vec3};

pub struct Line {
    begin: Vec3,
    end: Vec3,
    color: Color,
}

fn interpolate(i0: f64, i1: f64, d0: f64, d1: f64) -> Vec<f64> {
    let mut values = Vec::new();

    let a = (d1 - d0) / (i1 - i0);
    let mut d = d0;

    for i in i0 as isize..=i1 as isize {
        values.push(d);

        d += a;
    }

    values
}

impl Line {
    pub fn new(begin: Vec3, end: Vec3, color: Color) -> Self {
        Self { begin, end, color }
    }

    pub fn draw(&self, pixels: &mut [u8], width: usize, height: usize) {
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
                let pos = y_corrected * width * 3 + x_corrected;

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
                let pos = y_corrected * width * 3 + x_corrected;

                pixels[pos] = c[0];
                pixels[pos + 1] = c[1];
                pixels[pos + 2] = c[2];
            }
        }
    }
}
