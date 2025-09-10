use nalgebra_glm as glm;
use crate::core::color::Color;

#[derive(Clone, Copy)]
pub struct Light {
    pub position: glm::Vec3, // posición en el mundo
    pub color: Color,
    pub intensity: f32,
}

impl Light {
    pub fn point(position: glm::Vec3, color: Color, intensity: f32) -> Self {
        Self { position, color, intensity }
    }
}