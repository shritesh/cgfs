use macroquad::prelude::*;

mod canvas;
use canvas::Canvas;

const WINDOW_SIZE: u16 = 800;

fn window_conf() -> Conf {
    Conf {
        window_title: "Computer Graphics from Scratch".to_string(),
        window_width: WINDOW_SIZE as i32,
        window_height: WINDOW_SIZE as i32,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut canvas = Canvas::new(WINDOW_SIZE, WINDOW_SIZE);

    for x in -100..100 {
        for y in -100..100 {
            canvas.put_pixel(x, y, RED);
        }
    }

    let texture = canvas.render();

    loop {
        draw_texture(texture, 0.0, 0.0, WHITE);
        next_frame().await
    }
}
