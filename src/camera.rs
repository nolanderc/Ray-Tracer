use ::ray::Ray;

#[derive(Clone)]
pub struct Camera {
    position: [f64; 3],
    direction: [f64; 3],
    up: [f64; 3],

    // The field of view in radians
    field_of_view: f64,
}

impl Camera {
    pub fn new() -> Camera {
        use std::f64::consts::PI;

        Camera {
            position: [0.0; 3],
            direction: [0.0, 0.0, 1.0],
            up: [0.0, 1.0, 0.0],

            field_of_view: 90.0 / 180.0 * PI,
        }
    }


    pub fn look_at(
        &mut self,
        position: [f64; 3],
        direction: [f64; 3],
        up: [f64; 3]
    ) {
        self.position = position;

        let dx: f64 = direction[0];
        let dy: f64 = direction[1];
        let dz: f64 = direction[2];
        let d_len = (dx*dx + dy*dy + dz*dz).sqrt();

        self.direction = [
            dx / d_len, dy / d_len, dz / d_len
        ];


        let ux: f64 = up[0];
        let uy: f64 = up[1];
        let uz: f64 = up[2];
        let u_len = (ux*ux + uy*uy + uz*uz).sqrt();

        self.up = [
            ux / u_len, uy / u_len, uz / u_len
        ];
    }


    /// Set the horizontal field of view in degrees
    pub fn set_field_of_view_deg(&mut self, field_of_view: f64) {
        use std::f64::consts::PI;
        self.field_of_view = field_of_view / 180.0 * PI;

    }


    /// Returns a ray as cast from the camera.
    /// The input is relative to the edges of the camera.
    /// X: [-1, 1] => [left, right]
    /// Y: [-1, 1] => [bottom, top]
    pub fn cast_ray(&self, x: f64, y: f64, aspect_ratio: f64) -> Ray {
        let dx = self.direction[0] as f64;
        let dy = self.direction[1] as f64;
        let dz = self.direction[2] as f64;

        let ux = self.up[0] as f64;
        let uy = self.up[1] as f64;
        let uz = self.up[2] as f64;


        // right = up x direction
        let rx = uy*dz-uz*dy;
        let ry = uz*dx-ux*dz;
        let rz = ux*dy-uy*dx;

        // vertical = right x direction
        let vx = dy*rz-dz*ry;
        let vy = dz*rx-dx*rz;
        let vz = dx*ry-dy*rx;

        let far = 1.0;

        let half_fov = self.field_of_view * 0.5;

        let width = half_fov.tan();
        let height = width / aspect_ratio;

        // screen = x * right + y * vertical + distance * far
        let sx = width * x * rx + height * y * vx + dx;
        let sy = width * x * ry + height * y * vy + dy;
        let sz = width * x * rz + height * y * vz + dz;

        let len = (sx*sx + sy*sy + sz*sz).sqrt();

        // direction = screen - origin
        let nx = sx / len;
        let ny = sy / len;
        let nz = sz / len;


        Ray {
            x: self.position[0],
            y: self.position[1],
            z: self.position[2],
            dx: nx,
            dy: ny,
            dz: nz,
        }
    }
}