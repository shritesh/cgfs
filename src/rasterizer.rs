use crate::{Canvas, Color, Vec3};

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

pub fn draw_example_cube(canvas: &mut Canvas) {
    let a_front = Vec3(-2.0, -0.5, 5.0);
    let b_front = Vec3(-2.0, 0.5, 5.0);
    let c_front = Vec3(-1.0, 0.5, 5.0);
    let d_front = Vec3(-1.0, -0.5, 5.0);

    let a_back = Vec3(-2.0, -0.5, 6.0);
    let b_back = Vec3(-2.0, 0.5, 6.0);
    let c_back = Vec3(-1.0, 0.5, 6.0);
    let d_back = Vec3(-1.0, -0.5, 6.0);

    // front face
    draw_line(
        canvas,
        project_vertex(canvas, a_front),
        project_vertex(canvas, b_front),
        Color(0, 0, 255),
    );
    draw_line(
        canvas,
        project_vertex(canvas, b_front),
        project_vertex(canvas, c_front),
        Color(0, 0, 255),
    );
    draw_line(
        canvas,
        project_vertex(canvas, c_front),
        project_vertex(canvas, d_front),
        Color(0, 0, 255),
    );
    draw_line(
        canvas,
        project_vertex(canvas, d_front),
        project_vertex(canvas, a_front),
        Color(0, 0, 255),
    );

    // back face
    draw_line(
        canvas,
        project_vertex(canvas, a_back),
        project_vertex(canvas, b_back),
        Color(255, 0, 0),
    );
    draw_line(
        canvas,
        project_vertex(canvas, b_back),
        project_vertex(canvas, c_back),
        Color(255, 0, 0),
    );
    draw_line(
        canvas,
        project_vertex(canvas, c_back),
        project_vertex(canvas, d_back),
        Color(255, 0, 0),
    );
    draw_line(
        canvas,
        project_vertex(canvas, d_back),
        project_vertex(canvas, a_back),
        Color(255, 0, 0),
    );

    // front to back
    draw_line(
        canvas,
        project_vertex(canvas, a_front),
        project_vertex(canvas, a_back),
        Color(0, 255, 0),
    );
    draw_line(
        canvas,
        project_vertex(canvas, b_front),
        project_vertex(canvas, b_back),
        Color(0, 255, 0),
    );
    draw_line(
        canvas,
        project_vertex(canvas, c_front),
        project_vertex(canvas, c_back),
        Color(0, 255, 0),
    );
    draw_line(
        canvas,
        project_vertex(canvas, d_front),
        project_vertex(canvas, d_back),
        Color(0, 255, 0),
    );
}
