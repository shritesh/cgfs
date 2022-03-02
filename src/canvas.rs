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

    pub fn width(&self) -> u16 {
        self.image.width
    }

    pub fn height(&self) -> u16 {
        self.image.height
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        let screen_x = self.image.width as i32 / 2 + x;
        let screen_y = self.image.height as i32 / 2 - y;

        if screen_x >= 0
            && screen_x < self.image.width as i32
            && screen_y >= 0
            && screen_y < self.image.height as i32
        {
            self.image
                .set_pixel(screen_x as u32, screen_y as u32, color);
        }
    }

    pub fn to_texture(&self) -> Texture2D {
        Texture2D::from_image(&self.image)
    }
}
