use ::bitmap::Bitmap;
use ::color::Color;
use ::camera::Camera;

use ::objects::Sphere;

use ::ray::{Ray, IntersectRay, RayIntersection};

pub struct Scene {
    camera: Camera
}


impl Scene {
    pub fn new() -> Scene {
        Scene {
            camera: Camera::new()
        }
    }

    /// Set camera
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }


    /// Trace the scene, return the bitmap
    pub fn trace(&self, width: u32, height: u32, samples: u32) -> Bitmap {
        let mut bitmap = Bitmap::new(width, height);
        let aspect_ratio = width as f64 / height as f64;

        let spheres = vec![
            Sphere {
                x: 0.0,
                y: 0.0,
                z: 2.0,
                radius: 0.5,
            },
            Sphere {
                x: 1.0,
                y: 0.0,
                z: 4.0,
                radius: 0.5,
            }
        ];

        for x in 0..width {
            for y in 0..height {
                let mut average_color = Color::zero();
                let mut color_sampling = 0.0;

                for dx in 0..samples {
                    for dy in 0..samples {
                        let px = (dx as f64 / samples as f64 + x as f64) / width as f64;
                        let py = (dy as f64 / samples as f64 + y as f64) / height as f64;

                        let ray = self.camera.cast_ray(2.0 * px - 1.0, 1.0 - 2.0 * py, aspect_ratio);

                        let mut first: Option<RayIntersection> = None;
                        for sphere in spheres.iter() {
                            if let Some(intersection) = sphere.intersect(ray) {
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

                        if let Some(intersection) = first {
                            let color = Color::new(
                                intersection.nx.abs(),
                                intersection.ny.abs(),
                                intersection.nz.abs(),
                                1.0,
                            );

                            average_color.r += color.r as f64;
                            average_color.g += color.g as f64;
                            average_color.b += color.b as f64;
                            average_color.a += color.a as f64;

                            color_sampling += color.a;
                        }
                    }
                }

                average_color.r /= color_sampling;
                average_color.g /= color_sampling;
                average_color.b /= color_sampling;
                average_color.a /= (samples * samples) as f64;

                bitmap.set_pixel(x, y, average_color);
            }
        }


        bitmap
    }
}


