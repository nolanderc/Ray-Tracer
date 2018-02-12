
use ::ray::{Ray, IntersectRay, RayIntersection};

pub struct Sphere {
    // Center of the sphere
    pub x: f64,
    pub y: f64,
    pub z: f64,

    // Radius of the sphere
    pub radius: f64
}

impl super::WorldObject for Sphere {

}

impl IntersectRay for Sphere {
    fn intersect(&self, ray: Ray) -> Option<RayIntersection> {
        let dot = ray.project(self.x, self.y, self.z);

        if dot < 0.0 {
            return None;
        }

        let closest_x = ray.x + dot * ray.dx;
        let closest_y = ray.y + dot * ray.dy;
        let closest_z = ray.z + dot * ray.dz;

        let delta_x = closest_x - self.x;
        let delta_y = closest_y - self.y;
        let delta_z = closest_z - self.z;
        let distance_squared = delta_x*delta_x + delta_y*delta_y + delta_z*delta_z;

        let radius_squared = self.radius * self.radius;

        if distance_squared > radius_squared {
            return None;
        }

        let deviation = (radius_squared - distance_squared).sqrt();

        // There are two solutions
        let a_x = closest_x - deviation * ray.dx;
        let a_y = closest_y - deviation * ray.dy;
        let a_z = closest_z - deviation * ray.dz;

        let b_x = closest_x + deviation * ray.dx;
        let b_y = closest_y + deviation * ray.dy;
        let b_z = closest_z + deviation * ray.dz;

        if dot - deviation >= 0.0 || deviation == 0.0 {
            Some(RayIntersection {
                x: a_x,
                y: a_y,
                z: a_z,

                nx: (a_x - self.x) / self.radius,
                ny: (a_y - self.y) / self.radius,
                nz: (a_z - self.z) / self.radius,

                distance: dot - deviation
            })
        } else {
            Some(RayIntersection {
                x: b_x,
                y: b_y,
                z: b_z,

                nx: (b_x - self.x) / self.radius,
                ny: (b_y - self.y) / self.radius,
                nz: (b_z - self.z) / self.radius,

                distance: dot - deviation
            })
        }
    }
}
