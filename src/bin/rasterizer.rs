use cgfs::{Canvas, Rasterizer};
pub fn main() {
    let mut canvas = Canvas::new("Computer Graphics from Scratch - Raytracer", 800, 800);
    let mut rasterizer = Rasterizer::default_scene();
    canvas.render(&mut rasterizer);
}
