use cgfs::{rasterizer, Canvas};
pub fn main() {
    let mut canvas = Canvas::new("Computer Graphics from Scratch - Rasterizer", 800, 800);
    rasterizer::draw_example_scene(&mut canvas);
    canvas.show();
}
