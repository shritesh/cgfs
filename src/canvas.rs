use std::convert::TryInto;

use macroquad::prelude::*;

pub struct Canvas {
    image: Image,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            image: Image::gen_image_color(width, height, WHITE),
        }
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        let screen_x = self.image.width as i32 / 2 + x;
        let screen_y = self.image.height as i32 / 2 - y;

        if let (Ok(s_x), Ok(s_y)) = (screen_x.try_into(), screen_y.try_into()) {
            self.image.set_pixel(s_x, s_y, color);
        }
    }

    pub fn render(&self) -> Texture2D {
        Texture2D::from_image(&self.image)
    }
}
