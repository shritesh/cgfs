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
            rasterizer::draw_example_cube(&mut canvas);
        }
        _ => return eprintln!("specify 'raytrace' or 'rasterize'"),
    }

    canvas.show();
}
