use raytracer::{render::render, scene::Scene};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scene = Scene::new();

    render(&scene, "test.png")
}
