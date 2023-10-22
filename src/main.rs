mod canvas;
mod color;
mod rasterizer;
mod vec3;

use canvas::Canvas;
use color::Color;
use rasterizer::Point;

fn main() {
    let mut canvas = Canvas::new("Computer Graphics from Scratch", 800, 800);

    rasterizer::draw_filled_triangle(
        &mut canvas,
        Point(-200, -250),
        Point(200, 50),
        Point(20, 250),
        Color(0, 255, 0),
    );

    rasterizer::draw_wireframe_triangle(
        &mut canvas,
        Point(-200, -250),
        Point(200, 50),
        Point(20, 250),
        Color(255, 255, 255),
    );

    canvas.show();
}
