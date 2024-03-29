extern crate image;
extern crate rand;

mod bitmap;
mod color;

mod scene;
mod camera;
mod ray;

mod object;

use object::Sphere;

fn main() {
    let mut scene = scene::Scene::new();

    let mut camera = camera::Camera::new();
    let (x, y, z) = (4.0, 3.0, -3.0);
    camera.look_at([x, y, z], [-x - 1.0, -y + 0.5, -z + 1.0], [0.0, 1.0, 0.0]);
    camera.set_field_of_view_deg(70.0);

    scene.set_camera(camera);


    scene.add_object(Box::new(
        Sphere {
            x: -1.0,
            y: 0.0,
            z: 0.75,
            radius: 0.5,
        }
    ));

    scene.add_object(Box::new(
        Sphere {
            x: 0.5,
            y: 0.25,
            z: 1.5,
            radius: 0.75,
        }
    ));

    scene.add_object(Box::new(
        Sphere {
            x: -5.0,
            y: 0.0,
            z: 5.0,
            radius: 0.5,
        }
    ));


    scene.add_light(3.0, 1.5, 1.5, 0.5);

    let bitmap = scene.trace(1920, 1080, 4);

    if let Err(e) = bitmap.save("bitmap.png") {
        println!("{}", e);
    }
}
