use rasterizer::{render::render, scene::Scene};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let scene = Scene::default();
    // let scene = Scene::new_filled_triangle();
    // let scene = Scene::new_shaded_triangle();
    // let scene = Scene::new_wireframe_cube();
    let scene = Scene::new_wireframe_object();

    render(&scene, "test.png")
}
