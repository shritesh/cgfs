use image::{ImageBuffer, Rgb};

pub struct Canvas {
    width: i32,
    height: i32,
    imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Self {
        Canvas {
            width,
            height,
            imgbuf: ImageBuffer::new(width as u32, height as u32),
        }
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: [u8; 3]) {
        let screen_x = self.width / 2 + x;
        let screen_y = self.height / 2 - y;

        if screen_x >= 0 && screen_x < self.width && screen_y >= 0 && screen_y < self.height {
            self.imgbuf
                .put_pixel(screen_x as u32, screen_y as u32, Rgb(color));
        }
    }

    pub fn save(&self, filename: &str) {
        self.imgbuf
            .save(filename)
            .expect("Unable to write to file.");
    }
}
