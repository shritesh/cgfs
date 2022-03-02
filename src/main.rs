mod canvas;
mod scene;

use canvas::Canvas;
use macroquad::prelude::*;
use scene::{Scene, Sphere};

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
    let mut scene = Scene::new(Vec2::ONE, 1.0);
    scene.add_sphere(Sphere {
        center: Vec3::new(0.0, -1.0, 3.0),
        radius: 1.0,
        color: Color::new(1.0, 0.0, 0.0, 1.0),
    });
    scene.add_sphere(Sphere {
        center: Vec3::new(2.0, 0.0, 4.0),
        radius: 1.0,
        color: Color::new(0.0, 0.0, 1.0, 1.0),
    });
    scene.add_sphere(Sphere {
        center: Vec3::new(-2.0, 0.0, 4.0),
        radius: 1.0,
        color: Color::new(0.0, 1.0, 0.0, 1.0),
    });
    scene.add_sphere(Sphere {
        center: Vec3::new(0.0, -5001.0, 0.0),
        radius: 5000.0,
        color: Color::new(1.0, 1.0, 0.0, 1.0),
    });

    let mut canvas = Canvas::new(WINDOW_SIZE, WINDOW_SIZE);
    scene.render(&mut canvas);

    let texture = canvas.to_texture();

    loop {
        draw_texture(texture, 0.0, 0.0, WHITE);
        next_frame().await
    }
}
