mod canvas;
mod color;
mod light;
mod raytracer;
mod sphere;
mod vec3;

use canvas::Canvas;
use color::Color;
use light::Light;
use raytracer::Raytracer;
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    let mut canvas = Canvas::new("Computer Graphics from Scratch", 800, 800);

    Raytracer::DEFAULT_SCENE.render(&mut canvas);
    canvas.show();
}
