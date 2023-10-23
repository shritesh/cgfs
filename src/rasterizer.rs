use crate::{Canvas, Color, Matrix, Vec3};

#[derive(Clone, Copy)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }
}

fn interpolate<T: Into<f64> + Copy>(i0: i32, d0: T, i1: i32, d1: T) -> Vec<(i32, f64)> {
    let mut values = Vec::new();

    if i0 == i1 {
        values.push((i0, d0.into()));
    } else {
        let a = (d1.into() - d0.into()) / (i1 - i0) as f64;
        let mut d = d0.into();
        for i in i0..=i1 {
            values.push((i, d));
            d += a;
        }
    }

    values
}

pub fn draw_line(canvas: &mut Canvas, mut p0: Point, mut p1: Point, color: Color) {
    if (p1.x() - p0.x()).abs() > (p1.y() - p0.y()).abs() {
        // line is horizontal-ish

        if p0.x() > p1.x() {
            std::mem::swap(&mut p0, &mut p1);
        }

        for (x, y) in interpolate(p0.x(), p0.y(), p1.x(), p1.y()) {
            canvas.put_pixel(x, y as i32, color);
        }
    } else {
        if p0.y() > p1.y() {
            std::mem::swap(&mut p0, &mut p1);
        }

        for (y, x) in interpolate(p0.y(), p0.x(), p1.y(), p1.x()) {
            canvas.put_pixel(x as i32, y, color);
        }
    }
}

pub fn draw_wireframe_triangle(canvas: &mut Canvas, p0: Point, p1: Point, p2: Point, color: Color) {
    draw_line(canvas, p0, p1, color);
    draw_line(canvas, p1, p2, color);
    draw_line(canvas, p2, p0, color);
}

const VIEWPORT_WIDTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 1.0;
const DISTANCE: f64 = 1.0;

fn viewport_to_canvas(canvas: &Canvas, x: f64, y: f64) -> Point {
    Point(
        (x * canvas.width() as f64 / VIEWPORT_WIDTH) as i32,
        (y * canvas.height() as f64 / VIEWPORT_HEIGHT) as i32,
    )
}

fn project_vertex(canvas: &Canvas, v: Vec3) -> Point {
    viewport_to_canvas(canvas, v.0 * DISTANCE / v.2, v.1 * DISTANCE / v.2)
}

struct Triangle(pub usize, pub usize, pub usize, pub Color);
fn render_triangle(canvas: &mut Canvas, triangle: &Triangle, projected: &[Point]) {
    draw_wireframe_triangle(
        canvas,
        projected[triangle.0],
        projected[triangle.1],
        projected[triangle.2],
        triangle.3,
    )
}

struct Model<'a> {
    vertices: &'a [Vec3],
    triangles: &'a [Triangle],
}

struct Transform {
    scale: f64,
    rotation: Matrix,
    position: Vec3,
}

impl Transform {
    fn matrix(&self) -> Matrix {
        Matrix::translation(self.position) * (self.rotation * Matrix::scaling(self.scale))
    }
}

struct Instance<'a> {
    model: &'a Model<'a>,
    transform: Transform,
}

struct Camera {
    position: Vec3,
    orientation: Matrix,
}

fn render_model(canvas: &mut Canvas, model: &Model, transform_matrix: Matrix) {
    let projected: Vec<Point> = model
        .vertices
        .into_iter()
        .map(|v| project_vertex(canvas, transform_matrix * *v))
        .collect();

    for t in model.triangles {
        render_triangle(canvas, t, &projected);
    }
}

fn render_scene(canvas: &mut Canvas, camera: Camera, instances: &[Instance]) {
    let camera_matrix =
        camera.orientation.transpose() * Matrix::translation(-1.0 * camera.position);

    for instance in instances {
        let transform_matrix = camera_matrix * instance.transform.matrix();
        render_model(canvas, instance.model, transform_matrix);
    }
}

pub fn draw_example_scene(canvas: &mut Canvas) {
    let vertices = [
        Vec3(1.0, 1.0, 1.0),
        Vec3(-1.0, 1.0, 1.0),
        Vec3(-1.0, -1.0, 1.0),
        Vec3(1.0, -1.0, 1.0),
        Vec3(1.0, 1.0, -1.0),
        Vec3(-1.0, 1.0, -1.0),
        Vec3(-1.0, -1.0, -1.0),
        Vec3(1.0, -1.0, -1.0),
    ];

    let red = Color(255, 0, 0);
    let green = Color(0, 255, 0);
    let blue = Color(0, 0, 255);
    let yellow = Color(255, 255, 0);
    let purple = Color(255, 0, 255);
    let cyan = Color(0, 255, 255);

    let triangles = [
        Triangle(0, 1, 2, red),
        Triangle(0, 2, 3, red),
        Triangle(4, 0, 3, green),
        Triangle(4, 3, 7, green),
        Triangle(5, 4, 7, blue),
        Triangle(5, 7, 6, blue),
        Triangle(1, 5, 6, yellow),
        Triangle(1, 6, 2, yellow),
        Triangle(4, 5, 1, purple),
        Triangle(4, 1, 0, purple),
        Triangle(2, 6, 7, cyan),
        Triangle(2, 7, 3, cyan),
    ];

    let cube = Model {
        vertices: &vertices,
        triangles: &triangles,
    };

    let instances = [
        Instance {
            model: &cube,
            transform: Transform {
                scale: 0.75,
                rotation: Matrix::IDENTITY,
                position: Vec3(-1.5, 0.0, 7.0),
            },
        },
        Instance {
            model: &cube,
            transform: Transform {
                scale: 1.0,
                rotation: Matrix::rotation_y(195.0),
                position: Vec3(1.25, 2.0, 7.5),
            },
        },
    ];

    let camera = Camera {
        position: Vec3(-3.0, 1.0, 2.0),
        orientation: Matrix::rotation_y(-30.0),
    };

    render_scene(canvas, camera, &instances);
}
