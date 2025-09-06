use nalgebra_glm as glm;
use crate::core::{color::Color, texture::Texture};

#[derive(Clone)]
pub struct Skybox {
    // Orden: +X, -X, +Y, -Y, +Z, -Z
    pub faces: [Texture; 6],
}

impl Skybox {
    pub fn new(px: Texture, nx: Texture, py: Texture, ny: Texture, pz: Texture, nz: Texture) -> Self {
        Self { faces: [px, nx, py, ny, pz, nz] }
    }

    /// Mapea dirección en mundo a (cara, uv) y samplea la textura correspondiente.
    pub fn sample(&self, dir: glm::Vec3) -> Color {
        let d = glm::normalize(&dir);
        let x = d.x; let y = d.y; let z = d.z;
        let ax = x.abs(); let ay = y.abs(); let az = z.abs();

        let (face, u, v) = if ax >= ay && ax >= az {
            // ±X
            if x > 0.0 { // +X
                // u = -z/|x|, v =  y/|x|
                (0usize, -z/ax,  y/ax)
            } else {      // -X
                // u =  z/|x|, v =  y/|x|
                (1usize,  z/ax,  y/ax)
            }
        } else if ay >= ax && ay >= az {
            // ±Y
            if y > 0.0 { // +Y (top)
                // u =  x/|y|, v =  z/|y|
                (2usize,  x/ay,  z/ay)
            } else {      // -Y (bottom)
                // u =  x/|y|, v = -z/|y|
                (3usize,  x/ay, -z/ay)
            }
        } else {
            // ±Z
            if z > 0.0 { // +Z (front)
                // u =  x/|z|, v =  y/|z|
                (4usize,  x/az,  y/az)
            } else {      // -Z (back)
                // u = -x/|z|, v =  y/|z|
                (5usize, -x/az,  y/az)
            }
        };

        // u,v en [-1,1] -> [0,1]; usualmente V se invierte según tu Texture::sample
        let mut s = (u + 1.0) * 0.5;
        let mut t = (v + 1.0) * 0.5;
        // Si tu sample usa t=0 arriba, invierte:
        t = 1.0 - t;

        self.faces[face].sample((s, t))
    }
}