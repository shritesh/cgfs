mod canvas;
use canvas::Canvas;

fn main() {
    let mut canvas = Canvas::new(800, 800);
    for x in -5..5 {
        for y in -5..5 {
            canvas.put_pixel(x, y, &[255, 0, 0]);
        }
    }
    canvas.save("output.png");
}
