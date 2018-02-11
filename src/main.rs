extern crate image;

mod bitmap;
mod color;

mod scene;
mod camera;
mod ray;

mod objects;

fn main() {
    let scene = scene::Scene::new();

    let bitmap = scene.trace(1920, 1080, 4);

    if let Err(e) = bitmap.save("bitmap.png") {
        println!("{}", e);
    }
}
