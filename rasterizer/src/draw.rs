pub trait Draw {
    fn draw(&self, pixels: &mut [u8], width: usize, height: usize);
}
