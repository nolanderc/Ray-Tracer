use ::ray::Ray;

pub struct Camera {
    position: [f64; 3],
    direction: [f64; 3],

    // The field of view in degrees
    field_of_view: f64,

    // The aspect ratio
    aspect_ratio: f64
}

impl Camera {

    pub fn new() -> Camera {
        Camera {
            position: [0.0; 3],
            direction: [0.0, 0.0, 1.0],

            field_of_view: 70.0,
            aspect_ratio: 1.0,
        }
    }


    pub fn look_at(
        &mut self,
        position: [f64; 3],
        direction: [f64; 3]
    ) {
        self.position = position;
        self.direction = direction;
    }


    /// Set the horizontal field of view in degrees
    pub fn set_field_of_view(&mut self, field_of_view: f64) {
        self.field_of_view = field_of_view;
    }


    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) {
        self.aspect_ratio = aspect_ratio;
    }


    /// Returns a ray as cast from the camera.
    /// The input is relative to the edges of the camera.
    /// X: [-1, 1] => [left, right]
    /// Y: [-1, 1] => [bottom, top]
    pub fn cast_ray(&self, x: f64, y: f64, aspect_ratio: f64) -> Ray {
        use std::f64::consts::PI;

        let half_fov = self.field_of_view / 360.0 * PI;

        let (sin_x, cos_x) = (-x * half_fov).sin_cos();
        let (sin_y, cos_y) = (y * half_fov / aspect_ratio).sin_cos();

        let dx = self.direction[0] as f64;
        let dy = self.direction[1] as f64;
        let dz = self.direction[2] as f64;

        let nx = dx * cos_x - dz * sin_x;
        let nz = dx * sin_x + dz * cos_x;

        let s = (dx*dx + dz*dz).sqrt();
        let ns = s * cos_y - dy * sin_y;
        let ny = s * sin_y + dy * cos_y;

        Ray {
            x: self.position[0],
            y: self.position[1],
            z: self.position[2],
            dx: nx * ns,
            dy: ny,
            dz: nz * ns,
        }
    }
}