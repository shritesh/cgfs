mod canvas;
mod vector;
use canvas::Canvas;
use vector::Vector;

type Color = [u8; 3];

struct Sphere {
    center: Vector<3>,
    radius: f64,
    color: Color,
}

const CANVAS_WIDTH: i32 = 1000;
const CANVAS_HEIGHT: i32 = 1000;

const VIEWPORT_WIDTH: f64 = 1.;
const VIEWPORT_HEIGHT: f64 = 1.;
const VIEWPORT_DISTANCE: f64 = 1.;

const BACKGROUND_COLOR: Color = [255, 255, 255];

const SPHERES: [Sphere; 3] = [
    Sphere {
        center: Vector([0., -1., 3.]),
        radius: 1.,
        color: [255, 0, 0],
    },
    Sphere {
        center: Vector([2., 0., 4.]),
        radius: 1.,
        color: [0, 0, 255],
    },
    Sphere {
        center: Vector([-2., 0., 4.]),
        radius: 1.,
        color: [0, 255, 0],
    },
];

fn main() {
    let mut canvas = Canvas::new(CANVAS_WIDTH, CANVAS_HEIGHT);

    let origin = Vector([0., 0., 0.]);

    for x in (-CANVAS_WIDTH / 2)..(CANVAS_WIDTH / 2) {
        for y in (-CANVAS_HEIGHT / 2)..(CANVAS_HEIGHT / 2) {
            let d = canvas_to_viewport(x, y);
            let color = trace_ray(&origin, &d, 1., f64::INFINITY);
            canvas.put_pixel(x, y, color);
        }
    }

    canvas.save("output.png");
}

fn canvas_to_viewport(x: i32, y: i32) -> Vector<3> {
    Vector([
        x as f64 * VIEWPORT_WIDTH / CANVAS_WIDTH as f64,
        y as f64 * VIEWPORT_HEIGHT / CANVAS_HEIGHT as f64,
        VIEWPORT_DISTANCE,
    ])
}

fn trace_ray(origin: &Vector<3>, d: &Vector<3>, t_min: f64, t_max: f64) -> Color {
    let mut closest_t = f64::INFINITY;
    let mut closest_sphere = None;

    for sphere in SPHERES.iter() {
        let (t1, t2) = intersect_ray_sphere(origin, d, &sphere);

        if t1 > t_min && t1 < t_max && t1 < closest_t {
            closest_t = t1;
            closest_sphere = Some(sphere);
        }

        if t2 > t_min && t2 < t_max && t2 < closest_t {
            closest_t = t2;
            closest_sphere = Some(sphere);
        }
    }

    match closest_sphere {
        Some(sphere) => sphere.color,
        None => BACKGROUND_COLOR,
    }
}

fn intersect_ray_sphere(origin: &Vector<3>, d: &Vector<3>, sphere: &Sphere) -> (f64, f64) {
    let r = sphere.radius;
    let co = origin - &sphere.center;

    let a = d.dot(d);
    let b = 2. * co.dot(d);
    let c = co.dot(&co) - r * r;

    let discriminant = b * b - 4. * a * c;

    if discriminant < 0. {
        (f64::INFINITY, f64::INFINITY)
    } else {
        let t1 = (-b + discriminant.sqrt()) / (2. * a);
        let t2 = (-b - discriminant.sqrt()) / (2. * a);
        (t1, t2)
    }
}
