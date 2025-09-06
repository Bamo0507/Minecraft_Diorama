use nalgebra_glm as glm;
use super::material::Material;

#[derive(Clone, Copy, Debug)]
pub enum Face { NegX, PosX, NegY, PosY, NegZ, PosZ }

#[derive(Clone)]
pub struct Intersect {
    pub distance: f32,
    pub is_intersecting: bool,
    pub point: glm::Vec3,
    pub normal: glm::Vec3,
    pub uv: (f32, f32),
    pub face: Option<Face>,
    pub material: Material,
}

impl Intersect {
    pub fn hit(distance: f32, point: glm::Vec3, normal: glm::Vec3, uv: (f32, f32), face: Option<Face>, material: Material) -> Self {
        Self { distance, is_intersecting: true, point, normal, uv, face, material }
    }
    pub fn miss() -> Self {
        Self {
            distance: f32::INFINITY,
            is_intersecting: false,
            point: glm::vec3(0.0, 0.0, 0.0),
            normal: glm::vec3(0.0, 0.0, 0.0),
            uv: (0.0, 0.0),
            face: None,
            material: Material::default_black(),
        }
    }
}