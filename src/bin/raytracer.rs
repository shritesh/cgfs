use cgfs::{Canvas, Raytracer};
pub fn main() {
    let mut canvas = Canvas::new("Computer Graphics from Scratch - Raytracer", 800, 800);
    let mut raytracer = Raytracer::DEFAULT_SCENE;
    canvas.render(&mut raytracer);
}
