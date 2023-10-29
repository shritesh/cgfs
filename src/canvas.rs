use crate::Color;
use minifb::{Key, Window, WindowOptions};

pub struct Canvas {
    width: usize,
    height: usize,
    window: Window,
    buffer: Vec<u32>,
    depth_buffer: Vec<f64>,
}

pub trait Renderer {
    fn render(&self, canvas: &mut Canvas);

    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn move_front(&mut self);
    fn move_back(&mut self);

    fn rotate_left(&mut self);
    fn rotate_right(&mut self);
}

const BACKGROUND_COLOR: u32 = 0x00_FF_FF_FF;

impl Canvas {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        let buffer = vec![BACKGROUND_COLOR; width * height];
        let depth_buffer = vec![f64::INFINITY; width * height];
        let mut window = Window::new(title, width, height, WindowOptions::default()).unwrap();
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Self {
            width,
            height,
            window,
            buffer,
            depth_buffer,
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
        let sy = height / 2 - y - 1;

        if sx < 0 || sx >= width || sy < 0 || sy >= height {
            return;
        }

        let c = (color.0 as u32) << 16 | (color.1 as u32) << 8 | (color.2 as u32);

        self.buffer[sy as usize * self.width + sx as usize] = c;
    }

    pub fn update_depth_buffer(&mut self, x: i32, y: i32, z: f64) -> bool {
        let width = self.width as i32;
        let height = self.height as i32;

        let sx = width / 2 + x;
        let sy = height / 2 - y - 1;

        if sx < 0 || sx >= width || sy < 0 || sy >= height {
            return false;
        }

        let offset = sy as usize * self.width + sx as usize;
        if z < self.depth_buffer[offset] {
            self.depth_buffer[offset] = z;
            true
        } else {
            false
        }
    }

    fn reset(&mut self) {
        self.buffer.fill(BACKGROUND_COLOR);
        self.depth_buffer.fill(f64::INFINITY);
    }

    pub fn render(&mut self, renderer: &mut impl Renderer) {
        renderer.render(self);

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            if self.window.is_key_down(Key::W) {
                renderer.move_front();
                self.reset();
                renderer.render(self);
            }
            if self.window.is_key_down(Key::S) {
                renderer.move_back();
                self.reset();
                renderer.render(self);
            }
            if self.window.is_key_down(Key::A) {
                if self.window.is_key_down(Key::LeftShift) {
                    renderer.rotate_left();
                } else {
                    renderer.move_left();
                }
                self.reset();
                renderer.render(self);
            }
            if self.window.is_key_down(Key::D) {
                if self.window.is_key_down(Key::LeftShift) {
                    renderer.rotate_right();
                } else {
                    renderer.move_right();
                }
                self.reset();
                renderer.render(self);
            }

            if self.window.is_key_down(Key::Up) {
                renderer.move_up();
                self.reset();
                renderer.render(self);
            }

            if self.window.is_key_down(Key::Down) {
                renderer.move_down();
                self.reset();
                renderer.render(self);
            }
            self.window
                .update_with_buffer(&self.buffer, self.width, self.height)
                .unwrap();
        }
    }
}
