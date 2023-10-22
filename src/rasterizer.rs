use crate::{Canvas, Color};

#[derive(Clone, Copy)]
pub struct Point(pub i32, pub i32, pub f64);

impl Point {
    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }

    pub fn h(&self) -> f64 {
        self.2
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
    let mut h01 = interpolate(p0.y(), p0.h(), p1.y(), p1.h());

    let x12 = interpolate(p1.y(), p1.x(), p2.y(), p2.x());
    let h12 = interpolate(p1.y(), p1.h(), p2.y(), p2.h());

    let x02 = interpolate(p0.y(), p0.x(), p2.y(), p2.x());
    let h02 = interpolate(p0.y(), p0.h(), p2.y(), p2.h());

    _ = x01.pop();
    _ = h01.pop();

    let x012 = [x01, x12].concat();
    let h012 = [h01, h12].concat();

    let m = x02.len() / 2;
    let (x_left, h_left, x_right, h_right) = if x02[m].1 < x012[m].1 {
        (x02, h02, x012, h012)
    } else {
        (x012, h012, x02, h02)
    };

    for ((((left_y, left_x), (right_y, right_x)), (left_h_y, left_h)), (right_h_y, right_h)) in
        x_left.into_iter().zip(x_right).zip(h_left).zip(h_right)
    {
        let y = left_y;
        debug_assert_eq!(y, right_y);
        debug_assert_eq!(y, left_h_y);
        debug_assert_eq!(y, right_h_y);

        for (x, h) in interpolate(left_x as i32, left_h, right_x as i32, right_h) {
            canvas.put_pixel(x, y, h * color)
        }
    }
}
