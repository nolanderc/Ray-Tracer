use ::bitmap::Bitmap;
use ::color::Color;
use ::camera::Camera;

use ::object::WorldObject;

use ::ray::{Ray, IntersectRay, RayIntersection};


pub struct Scene {
    camera: Camera,

    objects: Vec<Box<WorldObject>>,
    lights: Vec<Light>,
}



enum Light {
    Point { x: f64, y: f64, z: f64, size: f64 }
}

impl Light {
    /// Create a new point light
    pub fn point(x: f64, y: f64, z: f64, size: f64) -> Light {
        Light::Point {
            x,
            y,
            z,
            size
        }
    }
}



impl Scene {
    pub fn new() -> Scene {
        Scene {
            camera: Camera::new(),

            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    /// Set camera
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }


    /// Add an object to the scene
    pub fn add_object(&mut self, object: Box<WorldObject>) {
        self.objects.push(object);
    }


    /// Add a point light to the scene
    pub fn add_light(&mut self, x: f64, y: f64, z: f64, size: f64) {
        self.lights.push(Light::point(x, y, z, size));
    }


    /// Trace the scene, return the bitmap
    pub fn trace(self, width: u32, height: u32, samples: u32) -> Bitmap {
        let mut bitmap = Bitmap::new(width, height);
        let aspect_ratio = width as f64 / height as f64;

        let thread_count = 8;
        let mut threads = Vec::new();

        use std::sync::mpsc::{channel};
        let (sender, receiver) = channel();

        use std::sync::{Arc, Mutex};
        let columns = Arc::new(Mutex::new((0..width).collect::<Vec<u32>>()));

        let scene = Arc::new(self);

        use std::thread;
        for thread in 0..thread_count {
            let columns = columns.clone();
            let sender = sender.clone();
            let scene = scene.clone();
            threads.push(thread::spawn(move||{
                loop {
                    let x = {
                        let mut columns = columns.lock().unwrap();
                        if let Some(column) = columns.pop() {
                            column
                        } else {
                            break;
                        }
                    };

                    let mut colors = Vec::with_capacity(height as usize);

                    for y in 0..height {
                        let mut average_color = Color::zero();
                        let mut color_sampling = 0.0;

                        for dx in 0..samples {
                            for dy in 0..samples {
                                let px = (dx as f64 / samples as f64 + x as f64) / width as f64;
                                let py = (dy as f64 / samples as f64 + y as f64) / height as f64;

                                let ray = scene.camera.cast_ray(2.0 * px - 1.0, 1.0 - 2.0 * py, aspect_ratio);
                                let color = scene.cast_ray(ray);

                                average_color.r += color.r as f64;
                                average_color.g += color.g as f64;
                                average_color.b += color.b as f64;
                                average_color.a += color.a as f64;

                                color_sampling += color.a;
                            }
                        }

                        average_color.r /= color_sampling;
                        average_color.g /= color_sampling;
                        average_color.b /= color_sampling;
                        average_color.a /= (samples * samples) as f64;

                        colors.push(average_color);
                    }

                    sender.send((x, colors));
                }
            }));
        }

        /*for x in 0..width {
            for y in 0..height {
                let mut average_color = Color::zero();
                let mut color_sampling = 0.0;

                for dx in 0..samples {
                    for dy in 0..samples {
                        let px = (dx as f64 / samples as f64 + x as f64) / width as f64;
                        let py = (dy as f64 / samples as f64 + y as f64) / height as f64;

                        let ray = self.camera.cast_ray(2.0 * px - 1.0, 1.0 - 2.0 * py, aspect_ratio);

                        let color = self.cast_ray(ray);

                        average_color.r += color.r as f64;
                        average_color.g += color.g as f64;
                        average_color.b += color.b as f64;
                        average_color.a += color.a as f64;

                        color_sampling += color.a;
                    }
                }

                average_color.r /= color_sampling;
                average_color.g /= color_sampling;
                average_color.b /= color_sampling;
                average_color.a /= (samples * samples) as f64;

                bitmap.set_pixel(x, y, average_color);

            }

            println!("Progress: {:.2}%", 100.0 * (x as f64 + 1.0) / width as f64);
        }*/

        let mut received_columns = 0;

        while let Ok((column, colors)) = receiver.recv() {
            // println!("Got pixel: {}, {}", x, y);
            for y in 0..height {
                bitmap.set_pixel(column, y, colors[y as usize]);
            }

            received_columns += 1;
            println!("Progress: {:.2}% ({} / {})", 100.0 * received_columns as f64 / (width) as f64, received_columns, width);

            if received_columns == width {
                break;
            }
        }


        for thread in threads {
            thread.join();
        }


        bitmap
    }


    /// Cast a ray into the scene, returning the resulting color
    fn cast_ray(&self, ray: Ray) -> Color {
        use rand;
        use rand::Rng;

        if let Some(intersection) = self.get_ray_intersection(ray) {
            // Find any shadows
            let mut lightness = 0.05;

            let mut rng = rand::thread_rng();

            for light in self.lights.iter() {
                match light {
                    &Light::Point {ref x, ref y, ref z, ref size} => {
                        let samples = 256;

                        let dx = x - intersection.x;
                        let dy = y - intersection.y;
                        let dz = z - intersection.z;

                        let len = (dx*dx + dy*dy + dz*dz).sqrt();

                        let nx = dx / len;
                        let ny = dy / len;
                        let nz = dz / len;

                        let (ux, uy, uz) = if
                            nx.abs() >= 0.9999 &&
                                ny.abs() <= 0.0001 &&
                                nz.abs() <= 0.0001 {
                            (1.0, 0.0, 0.0)
                        } else {
                            (0.0, 1.0, 0.0)
                        };

                        // right = up x direction
                        let rx = uy*nz-uz*ny;
                        let ry = uz*nx-ux*nz;
                        let rz = ux*ny-uy*nx;

                        // vertical = right x direction
                        let vx = ny*rz-nz*ry;
                        let vy = nz*rx-nx*rz;
                        let vz = nx*ry-ny*rx;

                        for sample in 0..samples {
                            let ox = size * rng.gen_range(-1.0, 1.0);
                            let oy = size * rng.gen_range(-1.0, 1.0);

                            let dx = x - intersection.x + ox * rx + oy * vx;
                            let dy = y - intersection.y + ox * ry + oy * vy;
                            let dz = z - intersection.z + ox * rz + oy * vz;

                            let len = (dx*dx + dy*dy + dz*dz).sqrt();

                            let bounce_ray = Ray {
                                x: intersection.x,
                                y: intersection.y,
                                z: intersection.z,
                                dx: dx / len,
                                dy: dy / len,
                                dz: dz / len,
                            };

                            if self.get_ray_intersection(bounce_ray).is_none() {
                                let dx = bounce_ray.dx;
                                let dy = bounce_ray.dy;
                                let dz = bounce_ray.dz;

                                let nx = intersection.nx;
                                let ny = intersection.ny;
                                let nz = intersection.nz;

                                // Phong shading
                                let dot = (dx*nx + dy*ny + dz*nz);
                                lightness += 0.95 / samples as f64 * if dot < 0.0 {0.0} else {dot};
                            }
                        }
                    }
                }
            }

            Color::new(
                lightness * intersection.nx.abs(),
                lightness * intersection.ny.abs(),
                lightness * intersection.nz.abs(),
                1.0,
            )
        } else {
            Color::zero()
        }
    }


    /// Cast a ray into a scene, returning the first collision
    fn get_ray_intersection(&self, ray: Ray) -> Option<RayIntersection> {
        let mut first: Option<RayIntersection> = None;
        for object in self.objects.iter() {
            if let Some(intersection) = object.intersect(ray) {
                if let Some(ref mut first) = first {
                    if intersection.distance < first.distance {
                        *first = intersection;
                        continue;
                    }
                } else {
                    first = Some(intersection);
                }
            }
        }

        let y = -0.5;
        let dy = y - ray.y;
        let time = dy / ray.dy;

        if time > 0.000005 {
            let intersection = RayIntersection {
                x: ray.x + time * ray.dx,
                y: ray.y + time * ray.dy,
                z: ray.z + time * ray.dz,
                nx: 0.0,
                ny: 1.0,
                nz: 0.0,
                distance: time,
            };

            if let Some(ref mut first) = first {
                if intersection.distance < first.distance {
                    *first = intersection;
                }
            } else {
                first = Some(intersection);
            }
        }

        first
    }
}
