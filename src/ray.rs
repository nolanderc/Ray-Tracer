
#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Ray {
    // Origin of ray
    pub x: f64,
    pub y: f64,
    pub z: f64,

    // Direction of ray
    pub dx: f64,
    pub dy: f64,
    pub dz: f64,
}

impl Ray {
    pub fn new(
        origin: [f64; 3],
        direction: [f64; 3],
    ) -> Ray {
        Ray {
            x: origin[0],
            y: origin[1],
            z: origin[2],

            dx: direction[0],
            dy: direction[1],
            dz: direction[2],
        }
    }


    /// The distance along the ray that a point's perpendicular projection is located
    pub fn project(&self, x: f64, y: f64, z: f64) -> f64 {
        // Translate all points, placing the ray's origin at the world's origin
        let ox = x - self.x;
        let oy = y - self.y;
        let oz = z - self.z;

        // Dot projection
        ox * self.dx + oy * self.dy + oz * self.dz
    }
}


pub trait IntersectRay {
    /// Return the intersection point with an object and a ray
    fn intersect(&self, ray: Ray) -> Option<RayIntersection>;
}


pub struct RayIntersection {
    // Point of intersection
    pub x: f64,
    pub y: f64,
    pub z: f64,

    // Normal direction of intersection
    pub nx: f64,
    pub ny: f64,
    pub nz: f64,

    // Distance from ray origin to intersection point
    pub distance: f64
}
