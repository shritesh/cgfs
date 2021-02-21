use image::{ImageBuffer, Rgb};

pub struct Canvas {
    width: u32,
    height: u32,
    imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Canvas {
            width,
            height,
            imgbuf: ImageBuffer::new(width, height),
        }
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: &[u8; 3]) {
        let screen_x = (self.width / 2) as i32 + x;
        let screen_y = (self.height / 2) as i32 - y;

        if screen_x >= 0
            && screen_x < self.width as i32
            && screen_y >= 0
            && screen_y < self.height as i32
        {
            self.imgbuf
                .put_pixel(screen_x as u32, screen_y as u32, Rgb(*color));
        }
    }

    pub fn save(&self, filename: &str) {
        self.imgbuf
            .save(filename)
            .expect("Unable to write to file.");
    }
}
