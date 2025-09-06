use nalgebra_glm as glm;

pub struct OrbitCamera {
    pub center: glm::Vec3,
    pub radius: f32,
    pub yaw: f32,   // θ (alrededor del eje Y)
    pub pitch: f32, // φ (vertical)
    pub up_world: glm::Vec3,
}

impl OrbitCamera {
    pub fn new(center: glm::Vec3, radius: f32, yaw: f32, pitch: f32) -> Self {
        Self { center, radius, yaw, pitch, up_world: glm::vec3(0.0, 1.0, 0.0) }
    }

    /// Posición de la cámara (eye) usando coordenadas esféricas
    pub fn eye(&self) -> glm::Vec3 {
        let x = self.radius * self.pitch.cos() * self.yaw.cos();
        let y = self.radius * self.pitch.sin();
        let z = self.radius * self.pitch.cos() * self.yaw.sin();
        self.center + glm::vec3(x, y, z)
    }

    /// Ejes de la cámara (Right, Up, Forward)
    pub fn basis(&self) -> (glm::Vec3, glm::Vec3, glm::Vec3) {
        let eye = self.eye();
        let forward = glm::normalize(&(self.center - eye)); // z_cam
        let right = glm::normalize(&glm::cross(&forward, &self.up_world)); // x_cam
        let up = glm::normalize(&glm::cross(&right, &forward)); // y_cam
        (right, up, forward)
    }

    pub fn rotate(&mut self, dyaw: f32, dpitch: f32) {
        self.yaw = wrap_angle(self.yaw + dyaw);
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01; // ~89°
        self.pitch = (self.pitch + dpitch).clamp(-max_pitch, max_pitch);
    }

    pub fn zoom(&mut self, factor: f32) {
        let new_r = (self.radius * factor).clamp(0.5, 50.0);
        self.radius = new_r;
    }
}

#[inline]
fn wrap_angle(a: f32) -> f32 {
    let two_pi = std::f32::consts::PI * 2.0;
    (a % two_pi + two_pi) % two_pi
}