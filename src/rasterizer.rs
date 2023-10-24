use crate::{Canvas, Color, Matrix, Renderer, Vec3};

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

pub fn draw_filled_triangle(
    canvas: &mut Canvas,
    mut p0: Point,
    mut p1: Point,
    mut p2: Point,
    color: Color,
) {
    // sort according to y
    if p1.y() < p0.y() {
        std::mem::swap(&mut p1, &mut p0);
    }
    if p2.y() < p0.y() {
        std::mem::swap(&mut p2, &mut p0);
    }
    if p2.y() < p1.y() {
        std::mem::swap(&mut p2, &mut p1);
    }

    let mut x01 = interpolate(p0.y(), p0.x(), p1.y(), p1.x());
    let x12 = interpolate(p1.y(), p1.x(), p2.y(), p2.x());
    let x02 = interpolate(p0.y(), p0.x(), p2.y(), p2.x());

    _ = x01.pop();

    let x012 = [x01, x12].concat();

    let m = x02.len() / 2;
    let (x_left, x_right) = if x02[m] < x012[m] {
        (x02, x012)
    } else {
        (x012, x02)
    };

    for ((left_y, left_x), (right_y, right_x)) in x_left.into_iter().zip(x_right) {
        assert_eq!(left_y, right_y);
        for x in (left_x as i32)..=(right_x as i32) {
            canvas.put_pixel(x, left_y, color)
        }
    }
}
struct Triangle(pub usize, pub usize, pub usize, pub Color);
fn render_triangle(canvas: &mut Canvas, triangle: &Triangle, projected: &[Point]) {
    draw_filled_triangle(
        canvas,
        projected[triangle.0],
        projected[triangle.1],
        projected[triangle.2],
        triangle.3,
    );

    draw_wireframe_triangle(
        canvas,
        projected[triangle.0],
        projected[triangle.1],
        projected[triangle.2],
        Color::BLACK,
    )
}

struct Model<'a> {
    vertices: &'a [Vec3],
    triangles: &'a [Triangle],
}

struct Transform {
    scale: f64,
    rotation: f64,
    position: Vec3,
}

impl Transform {
    fn matrix(&self) -> Matrix {
        Matrix::translation(self.position)
            * (Matrix::rotation_y(self.rotation) * Matrix::scaling(self.scale))
    }
}

struct Instance<'a> {
    model: &'a Model<'a>,
    transform: Transform,
}

struct Camera {
    position: Vec3,
    rotation: f64,
}

pub struct Rasterizer<'a> {
    camera: Camera,
    instances: &'a [Instance<'a>],
}

impl<'a> Rasterizer<'a> {
    const CUBE_MODEL: Model<'a> = Model {
        vertices: &[
            Vec3(1.0, 1.0, 1.0),
            Vec3(-1.0, 1.0, 1.0),
            Vec3(-1.0, -1.0, 1.0),
            Vec3(1.0, -1.0, 1.0),
            Vec3(1.0, 1.0, -1.0),
            Vec3(-1.0, 1.0, -1.0),
            Vec3(-1.0, -1.0, -1.0),
            Vec3(1.0, -1.0, -1.0),
        ],
        triangles: &[
            Triangle(0, 1, 2, Color::RED),
            Triangle(0, 2, 3, Color::RED),
            Triangle(4, 0, 3, Color::GREEN),
            Triangle(4, 3, 7, Color::GREEN),
            Triangle(5, 4, 7, Color::BLUE),
            Triangle(5, 7, 6, Color::BLUE),
            Triangle(1, 5, 6, Color::YELLOW),
            Triangle(1, 6, 2, Color::YELLOW),
            Triangle(4, 5, 1, Color::PURPLE),
            Triangle(4, 1, 0, Color::PURPLE),
            Triangle(2, 6, 7, Color::CYAN),
            Triangle(2, 7, 3, Color::CYAN),
        ],
    };
    pub const DEFAULT_SCENE: Self = Self {
        camera: Camera {
            position: Vec3(-3.0, 1.0, 2.0),
            rotation: -30.0,
        },
        instances: &[
            Instance {
                model: &Self::CUBE_MODEL,
                transform: Transform {
                    scale: 0.75,
                    rotation: 0.0,
                    position: Vec3(-1.5, 0.0, 7.0),
                },
            },
            Instance {
                model: &Self::CUBE_MODEL,
                transform: Transform {
                    scale: 1.0,
                    rotation: 195.0,
                    position: Vec3(1.25, 2.0, 7.5),
                },
            },
        ],
    };
}

impl<'a> Renderer for Rasterizer<'a> {
    fn render(&self, canvas: &mut Canvas) {
        let camera_matrix = Matrix::rotation_y(self.camera.rotation).transpose()
            * Matrix::translation(-1.0 * self.camera.position);

        for instance in self.instances {
            let transform_matrix = camera_matrix * instance.transform.matrix();
            render_model(canvas, instance.model, transform_matrix);
        }
    }

    fn move_up(&mut self) {
        self.camera.position.1 += 0.5;
    }

    fn move_down(&mut self) {
        self.camera.position.1 -= 0.5;
    }

    fn move_left(&mut self) {
        self.camera.position.0 -= 0.05;
    }

    fn move_right(&mut self) {
        self.camera.position.0 += 0.05;
    }

    fn move_front(&mut self) {
        self.camera.position.2 += 0.05;
    }

    fn move_back(&mut self) {
        self.camera.position.2 -= 0.05;
    }

    fn rotate_left(&mut self) {
        self.camera.rotation += 5.0;
    }

    fn rotate_right(&mut self) {
        self.camera.rotation -= 5.0;
    }
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
