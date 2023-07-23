use rasterizer::{render::render, scene::Scene};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let scene = Scene::default();
    let scene = Scene::new_triangle();

    render(&scene, "test.png")
}
