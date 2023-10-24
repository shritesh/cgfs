use cgfs::{Canvas, Raytracer};
pub fn main() {
    let mut canvas = Canvas::new("Computer Graphics from Scratch - Raytracer", 800, 800);
    Raytracer::DEFAULT_SCENE.render(&mut canvas);
    canvas.show();
}
