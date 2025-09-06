use nalgebra_glm as glm;
use crate::core::{intersect::Intersect, material::Material};
use super::RayIntersect;

pub struct Sphere { pub center: glm::Vec3, pub radius: f32, pub material: Material }
impl Sphere { pub fn new(center: glm::Vec3, radius: f32, material: Material) -> Self { Self { center, radius, material } } }

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ro: &glm::Vec3, rd: &glm::Vec3) -> Intersect {
        let oc = ro - self.center;
        let a = glm::dot(rd, rd);
        let b = 2.0 * glm::dot(&oc, rd);
        let c = glm::dot(&oc, &oc) - self.radius * self.radius;
        let disc = b * b - 4.0 * a * c;
        if disc < 0.0 { return Intersect::miss(); }

        let sqrt_disc = disc.sqrt();
        let mut t1 = (-b - sqrt_disc) / (2.0 * a);
        let mut t2 = (-b + sqrt_disc) / (2.0 * a);
        if t1 > t2 { core::mem::swap(&mut t1, &mut t2); }

        let t = if t1 > 0.001 { t1 } else if t2 > 0.001 { t2 } else { return Intersect::miss(); };
        let point = ro + rd * t;
        let normal = glm::normalize(&(point - self.center));

        // UV esférico (opcional)
        let u = 0.5 + normal.z.atan2(normal.x) / (2.0 * std::f32::consts::PI);
        let v = 0.5 - normal.y.asin() / std::f32::consts::PI;

        Intersect::hit(t, point, normal, (u, v), None, self.material.clone())
    }
}