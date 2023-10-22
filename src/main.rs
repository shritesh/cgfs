mod canvas;
mod color;
mod rasterizer;
mod raytracer;
mod vec3;

use std::env;

use canvas::Canvas;
use color::Color;
use rasterizer::Point;
use raytracer::Raytracer;
use vec3::Vec3;

fn main() {
    let mut canvas = Canvas::new("Computer Graphics from Scratch", 800, 800);

    match env::args().skip(1).next().as_deref() {
        Some("raytrace") => {
            Raytracer::DEFAULT_SCENE.render(&mut canvas);
        }
        Some("rasterize") => {
            let (p0, p1, p2) = (
                Point(-200, -250, 0.3),
                Point(200, 50, 0.1),
                Point(20, 250, 1.0),
            );

            rasterizer::draw_filled_triangle(&mut canvas, p0, p1, p2, Color(0, 255, 0));

            rasterizer::draw_wireframe_triangle(&mut canvas, p0, p1, p2, Color(0, 0, 0));
        }
        _ => return eprintln!("specify 'raytrace' or 'rasterize'"),
    }

    canvas.show();
}
