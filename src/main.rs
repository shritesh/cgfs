mod canvas;
mod color;
mod rasterizer;
mod vec3;

use canvas::Canvas;
use color::Color;

fn main() {
    let mut canvas = Canvas::new("Computer Graphics from Scratch", 800, 800);

    rasterizer::draw_wireframe_triangle(
        &mut canvas,
        (-200, -250),
        (200, 50),
        (20, 250),
        Color(255, 255, 255),
    );

    canvas.show();
}
