pub trait Draw {
    fn draw(
        &self,
        pixels: &mut [u8],
        width: usize,
        height: usize,
        viewport_size: usize,
        projection_plane_z: f64,
    );
}
