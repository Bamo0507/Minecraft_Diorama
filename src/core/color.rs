use nalgebra_glm as glm;

#[derive(Clone, Copy, Debug)]
pub struct Color { pub r: u8, pub g: u8, pub b: u8 } // Manejar rgb
impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }
    pub fn to_u32(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
    pub fn to_vec3(self) -> glm::Vec3 {
        glm::vec3(self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0)
    }
    pub fn from_vec3(v: &glm::Vec3) -> Self {
        let r = (v.x.clamp(0.0, 1.0) * 255.0).round() as u8;
        let g = (v.y.clamp(0.0, 1.0) * 255.0).round() as u8;
        let b = (v.z.clamp(0.0, 1.0) * 255.0).round() as u8;
        Self { r, g, b }
    }
}