use raylib::prelude::*;

pub struct Window {
    rl: RaylibHandle,
    thread: RaylibThread,
    tex: Texture2D,
    width: i32,
    height: i32,
}

impl Window {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let (mut rl, thread) = raylib::init()
            .size(width as i32, height as i32)
            .title(title)
            .build();
        rl.set_target_fps(40);

        // Creamos una textura vacía para volcar el framebuffer cada frame
        let img = Image::gen_image_color(width as i32, height as i32, Color::BLACK);
        let tex = rl
            .load_texture_from_image(&thread, &img)
            .expect("No se pudo crear la textura inicial");

        Self { rl, thread, tex, width: width as i32, height: height as i32 }
    }

    pub fn is_open(&self) -> bool {
        !self.rl.window_should_close()
    }

    // --- Inputs para cámara orbital ---
    pub fn mouse_delta(&self) -> (f32, f32) {
        let v = self.rl.get_mouse_delta();
        (v.x, v.y)
    }
    pub fn mouse_wheel(&self) -> f32 { self.rl.get_mouse_wheel_move() }
    pub fn is_mouse_down(&self, btn: MouseButton) -> bool { self.rl.is_mouse_button_down(btn) }

    pub fn present(&mut self, pixels_rgbx: &[u32]) {
        // Convertimos de 0x00RRGGBB (nuestro framebuffer) a RGBA8 que espera raylib
        let mut rgba = Vec::with_capacity((self.width * self.height * 4) as usize);
        for &p in pixels_rgbx {
            let r = ((p >> 16) & 0xFF) as u8;
            let g = ((p >> 8) & 0xFF) as u8;
            let b = (p & 0xFF) as u8;
            rgba.extend_from_slice(&[r, g, b, 255]);
        }

        // Actualiza la textura del frame actual (FFI estable entre versiones)
        use std::ffi::c_void;
        unsafe {
            raylib::ffi::UpdateTexture(*self.tex.as_ref(), rgba.as_ptr() as *const c_void);
        }

        let mut d = self.rl.begin_drawing(&self.thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&self.tex, 0, 0, Color::WHITE);
    }
}