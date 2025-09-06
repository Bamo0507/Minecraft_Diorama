use nalgebra_glm as glm;

pub struct Ray { pub origin: glm::Vec3, pub dir: glm::Vec3 }
impl Ray {
    pub fn new(origin: glm::Vec3, dir: glm::Vec3) -> Self {
        Self { origin, dir: glm::normalize(&dir) }
    }
}
