use super::color::Color;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pixels: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, pixels: vec![0; (width * height) as usize] }
    }
    pub fn clear(&mut self, c: Color) {
        self.pixels.fill(c.to_u32());
    }
    pub fn set_pixel(&mut self, x: u32, y: u32, c: Color) {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.pixels[idx] = c.to_u32();
        }
    }
    pub fn pixels(&self) -> &[u32] { &self.pixels }
    pub fn pixels_mut(&mut self) -> &mut [u32] { &mut self.pixels }
}
