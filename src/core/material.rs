use super::color::Color;
use super::texture::Texture;
use super::intersect::Face;

#[derive(Clone)]
pub enum AlbedoTex {
    None,
    Single(Texture),
    Cube {
        nx: Texture, px: Texture,
        ny: Texture, py: Texture,
        nz: Texture, pz: Texture,
    },
}

#[derive(Clone)]
pub struct Material {
    pub albedo: Color,
    pub specular: f32,
    pub shininess: f32,
    pub reflectivity: f32,
    pub transparency: f32,
    pub ior: f32, //indice de refraccion
    pub albedo_tex: AlbedoTex, //rexturas
}

impl Material {
    // una sola textura
    pub fn with_texture(tex: Texture, specular: f32, shininess: f32, reflectivity: f32, transparency: f32, ior: f32) -> Self {
        Self { albedo: Color::new(255,255,255), specular, shininess, reflectivity, transparency, ior, albedo_tex: AlbedoTex::Single(tex) }
    }

    // mas de una textura para cubo
    pub fn with_cube_textures(nx: Texture, px: Texture, ny: Texture, py: Texture, nz: Texture, pz: Texture,
        specular: f32, shininess: f32, reflectivity: f32, transparency: f32, ior: f32) -> Self {
        Self {
            albedo: Color::new(255,255,255),
            specular, shininess, reflectivity, transparency, ior,
            albedo_tex: AlbedoTex::Cube { nx, px, ny, py, nz, pz }
        }
    }

    // miss
    pub fn default_black() -> Self {
        Self { albedo: Color::new(0,0,0), specular: 0.0, shininess: 1.0, reflectivity: 0.0, transparency: 0.0, ior: 1.0, albedo_tex: AlbedoTex::None }
    }

    /// Devuelve el color base según UV (y cara si aplica)
    pub fn sample_albedo(&self, uv: (f32,f32), face: Option<Face>) -> Color {
        match &self.albedo_tex {
            AlbedoTex::None => self.albedo,
            AlbedoTex::Single(tex) => tex.sample(uv),
            AlbedoTex::Cube { nx, px, ny, py, nz, pz } => {
                let f = face.unwrap_or(Face::PosZ);
                let t = match f {
                    Face::NegX => nx, Face::PosX => px,
                    Face::NegY => ny, Face::PosY => py,
                    Face::NegZ => nz, Face::PosZ => pz,
                };
                t.sample(uv)
            }
        }
    }
}