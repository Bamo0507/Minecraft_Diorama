pub mod sphere;
pub mod aabb; // <-- agrega esto

use nalgebra_glm as glm;
use crate::core::intersect::Intersect;

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &glm::Vec3, ray_dir: &glm::Vec3) -> Intersect;
}