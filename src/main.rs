mod canvas;
mod color;
mod rasterizer;
mod vec3;

use canvas::Canvas;
use color::Color;

fn main() {
    let mut canvas = Canvas::new("Computer Graphics from Scratch", 800, 800);

    rasterizer::draw_line(&mut canvas, (0, 0), (50, 100), Color(255, 255, 255));
    rasterizer::draw_line(&mut canvas, (0, 0), (100, 50), Color(255, 255, 255));

    canvas.show();
}
