use raylib::prelude::*;

pub struct FrameBuffer {
    pub image: Image,
}

impl FrameBuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let image = Image::gen_image_color(width, height, Color::BLACK);
        Self { image }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.image.width && y >= 0 && y < self.image.height {
            self.image.draw_pixel(x, y, color);
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: Color) {
        self.image.draw_line(x0, y0, x1, y1, color);
    }

    pub fn save(&self, path: &str) {
        self.image.export_image(path);
    }
}
