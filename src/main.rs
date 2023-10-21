mod canvas;
mod color;
mod light;
mod scene;
mod sphere;
mod vec3;

use canvas::Canvas;
use color::Color;
use light::Light;
use scene::Scene;
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    let mut canvas = Canvas::new("Computer Graphics from Scratch", 800, 800);

    let scene = Scene::new(
        Vec3(0.0, 0.0, 0.0),
        Vec3(1.0, 1.0, 1.0),
        vec![
            Sphere {
                center: Vec3(0.0, -1.0, 3.0),
                radius: 1.0,
                color: Color(255, 0, 0),
                specular: 500.0,
            },
            Sphere {
                center: Vec3(2.0, 0.0, 4.0),
                radius: 1.0,
                color: Color(0, 0, 255),
                specular: 500.0,
            },
            Sphere {
                center: Vec3(-2.0, 0.0, 4.0),
                radius: 1.0,
                color: Color(0, 255, 0),
                specular: 10.0,
            },
            Sphere {
                center: Vec3(0.0, -5001.0, 0.0),
                radius: 5000.0,
                color: Color(255, 255, 0),
                specular: 1000.0,
            },
        ],
        vec![
            Light::Ambient { intensity: 0.2 },
            Light::Point {
                position: Vec3(2.0, 1.0, 0.0),
                intensity: 0.6,
            },
            Light::Directional {
                direction: Vec3(1.0, 4.0, 4.0),
                intensity: 0.2,
            },
        ],
    );

    scene.render(&mut canvas);
    canvas.show();
}
