use crate::core::color::Color;

#[derive(Clone)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>, // RGBA8
}

impl Texture {
    pub fn load(path: &str) -> Self {
        let img = image::open(path).expect(&format!("No pude abrir textura: {}", path));
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();
        Self { width: w, height: h, data: rgba.into_raw() }
    }

    /// UV en [0,1] con wrap (nearest neighbor)
    pub fn sample(&self, uv: (f32, f32)) -> Color {
        let (mut u, mut v) = uv;
        // wrap
        u = u - u.floor();
        v = v - v.floor();

        // Y invertida (v=0 arriba)
        let x = (u * (self.width as f32 - 1.0)).round().clamp(0.0, self.width as f32 - 1.0) as u32;
        let y = ((1.0 - v) * (self.height as f32 - 1.0)).round().clamp(0.0, self.height as f32 - 1.0) as u32;

        let idx = ((y * self.width + x) * 4) as usize;
        Color::new(self.data[idx], self.data[idx + 1], self.data[idx + 2])
    }

    pub fn rotated_180(self) -> Self {
        use image::{imageops, RgbaImage};
        let img = RgbaImage::from_raw(self.width, self.height, self.data)
            .expect("Texture buffer inválido");
        let rot = imageops::rotate180(&img);
        let (w, h) = rot.dimensions();
        Self { width: w, height: h, data: rot.into_raw() }
    }
}