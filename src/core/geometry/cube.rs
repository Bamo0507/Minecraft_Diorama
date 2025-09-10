use nalgebra_glm as glm;
use crate::core::{intersect::{Intersect, Face}, material::Material};
use super::RayIntersect;

#[derive(Clone)]
pub struct Cube {
    pub min: glm::Vec3, //esquina minima
    pub max: glm::Vec3, //esquina maxima
    pub material: Material,
}

impl Cube {
    pub fn new(min: glm::Vec3, max: glm::Vec3, material: Material) -> Self {
        Self { min, max, material }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ro: &glm::Vec3, rd: &glm::Vec3) -> Intersect {
        let inv = glm::vec3(1.0 / rd.x, 1.0 / rd.y, 1.0 / rd.z);

        let mut tmin = (self.min.x - ro.x) * inv.x;
        let mut tmax = (self.max.x - ro.x) * inv.x;
        if tmin > tmax { core::mem::swap(&mut tmin, &mut tmax); }

        let mut tymin = (self.min.y - ro.y) * inv.y;
        let mut tymax = (self.max.y - ro.y) * inv.y;
        if tymin > tymax { core::mem::swap(&mut tymin, &mut tymax); }

        if (tmin > tymax) || (tymin > tmax) { return Intersect::miss(); }
        if tymin > tmin { tmin = tymin; }
        if tymax < tmax { tmax = tymax; }

        let mut tzmin = (self.min.z - ro.z) * inv.z;
        let mut tzmax = (self.max.z - ro.z) * inv.z;
        if tzmin > tzmax { core::mem::swap(&mut tzmin, &mut tzmax); }

        if (tmin > tzmax) || (tzmin > tmax) { return Intersect::miss(); }
        if tzmin > tmin { tmin = tzmin; }
        if tzmax < tmax { tmax = tzmax; }

        let t = if tmin > 0.001 { tmin } else if tmax > 0.001 { tmax } else { return Intersect::miss(); };
        let p = ro + rd * t;

        let eps = 1e-4;
        let size = self.max - self.min;
        let local = (p - self.min).component_div(&size); // [0,1]^3

        let (n, uv, face) = if (p.x - self.min.x).abs() < eps {
            (glm::vec3(-1.0, 0.0, 0.0), (local.z, 1.0 - local.y), Face::NegX)
        } else if (p.x - self.max.x).abs() < eps {
            (glm::vec3( 1.0, 0.0, 0.0), (1.0 - local.z, 1.0 - local.y), Face::PosX)
        } else if (p.y - self.min.y).abs() < eps {
            (glm::vec3(0.0, -1.0, 0.0), (local.x, 1.0 - local.z), Face::NegY)
        } else if (p.y - self.max.y).abs() < eps {
            (glm::vec3(0.0,  1.0, 0.0), (local.x, local.z), Face::PosY)
        } else if (p.z - self.min.z).abs() < eps {
            (glm::vec3(0.0, 0.0, -1.0), (local.x, 1.0 - local.y), Face::NegZ)
        } else {
            (glm::vec3(0.0, 0.0,  1.0), (1.0 - local.x, 1.0 - local.y), Face::PosZ)
        };

        Intersect::hit(t, p, n, uv, Some(face), self.material.clone())
    }
}