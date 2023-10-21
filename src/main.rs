mod canvas;
mod color;
mod raytracer;
mod vec3;

use canvas::Canvas;
use color::Color;
use raytracer::Raytracer;
use vec3::Vec3;

fn main() {
    let mut canvas = Canvas::new("Computer Graphics from Scratch", 800, 800);

    Raytracer::DEFAULT_SCENE.render(&mut canvas);
    canvas.show();
}
