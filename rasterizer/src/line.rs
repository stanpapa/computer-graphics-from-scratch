use crate::{color::Color, vec3::Vec3};

pub struct Line {
    begin: Vec3,
    end: Vec3,
    color: Color,
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
            let a = dy / dx;
            let mut y = p0[1];

            for x in p0[0] as isize..=p1[0] as isize {
                let x_corrected = (x + width as isize / 2) as usize;
                let y_corrected = (height as isize / 2 - y as isize) as usize;
                let pos = y_corrected * width * 3 + x_corrected;

                pixels[pos] = c[0];
                pixels[pos + 1] = c[1];
                pixels[pos + 2] = c[2];

                y += a;
            }
        } else {
            // line is vertical-ish
            let (p0, p1) = if self.begin[1] > self.end[1] {
                (self.end, self.begin)
            } else {
                (self.begin, self.end)
            };
            let a = dx / dy;
            let mut x = p0[0];

            for y in p0[1] as isize..=p1[1] as isize {
                let x_corrected = (x as isize + width as isize / 2) as usize;
                let y_corrected = (height as isize / 2 - y) as usize;
                let pos = y_corrected * width * 3 + x_corrected;

                pixels[pos] = c[0];
                pixels[pos + 1] = c[1];
                pixels[pos + 2] = c[2];

                x += a;
            }
        }
    }
}
