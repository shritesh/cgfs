use image::{ImageBuffer, Rgb};

fn main() {
    let width = 800;
    let height = 800;

    let mut imgbuf = ImageBuffer::new(width, height);
    imgbuf.put_pixel(10, 10, Rgb([255u8, 255u8, 255u8]));
    imgbuf.save("output.png").expect("Unable to write to file.");
}
