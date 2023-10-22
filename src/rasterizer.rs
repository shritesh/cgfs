use crate::{Canvas, Color};

fn interpolate(i0: i32, d0: i32, i1: i32, d1: i32) -> Vec<(i32, f64)> {
    let a = (d1 - d0) as f64 / (i1 - i0) as f64;

    let mut values = Vec::new();

    if i0 == i1 {
        values.push((i0, d0 as f64));
    } else {
        let mut d = d0 as f64;
        for i in i0..=i1 {
            values.push((i, d));
            d += a;
        }
    }

    values
}

pub fn draw_line(canvas: &mut Canvas, p0: (i32, i32), p1: (i32, i32), color: Color) {
    let (mut x0, mut y0) = p0;
    let (mut x1, mut y1) = p1;

    let dx = x1 - x0;
    let dy = y1 - y0;

    if dx.abs() > dy.abs() {
        // line is horizontal-ish

        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        for (x, y) in interpolate(x0, y0, x1, y1) {
            canvas.put_pixel(x, y as i32, color);
        }
    } else {
        if y0 > y1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        for (y, x) in interpolate(y0, x0, y1, x1) {
            canvas.put_pixel(x as i32, y, color);
        }
    }
}
