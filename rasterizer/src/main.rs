use rasterizer::{render::render, scene::Scene};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scene = Scene::default();

    render(&scene, "test.png")
}
