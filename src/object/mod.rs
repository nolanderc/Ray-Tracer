
mod sphere;
pub use self::sphere::Sphere;

use ::ray::IntersectRay;

pub trait WorldObject: IntersectRay {}

