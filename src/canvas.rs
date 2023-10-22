use crate::Color;
use minifb::{Key, Window, WindowOptions};

pub struct Canvas {
    width: usize,
    height: usize,
    window: Window,
    buffer: Vec<u32>,
}

impl Canvas {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let buffer: Vec<u32> = vec![0x00_FF_FF_FF; width * height];
        let window = Window::new(title, width, height, WindowOptions::default()).unwrap();

        Self {
            width,
            height,
            window,
            buffer,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        let width = self.width as i32;
        let height = self.height as i32;

        let sx = width / 2 + x;
        let sy = height / 2 - y;

        if sx < 0 || sx >= width || sy < 0 || sy >= height {
            return;
        }

        let c = (color.0 as u32) << 16 | (color.1 as u32) << 8 | (color.2 as u32);

        self.buffer[sy as usize * self.width + sx as usize] = c;
    }

    pub fn show(&mut self) {
        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            self.window
                .update_with_buffer(&self.buffer, self.width, self.height)
                .unwrap();
        }
    }
}
