use crate::{Canvas, Color};

fn interpolate(i0: i32, d0: i32, i1: i32, d1: i32) -> Vec<f64> {
    let mut values = Vec::new();

    if i0 == i1 {
        values.push(d0 as f64);
    } else {
        let a = (d1 - d0) as f64 / (i1 - i0) as f64;
        let mut d = d0 as f64;
        for _ in i0..=i1 {
            values.push(d);
            d += a;
        }
    }

    values
}

pub fn draw_line(canvas: &mut Canvas, p0: (i32, i32), p1: (i32, i32), color: Color) {
    let (mut x0, mut y0) = p0;
    let (mut x1, mut y1) = p1;

    if (x1 - x0).abs() > (y1 - y0).abs() {
        // line is horizontal-ish

        if x0 > x1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }

        let ys = interpolate(x0, y0, x1, y1);

        for x in x0..=x1 {
            canvas.put_pixel(x, ys[(x - x0) as usize] as i32, color);
        }
    } else {
        if y0 > y1 {
            std::mem::swap(&mut x0, &mut x1);
            std::mem::swap(&mut y0, &mut y1);
        }
        let xs = interpolate(y0, x0, y1, x1);

        for y in y0..=y1 {
            canvas.put_pixel(xs[(y - y0) as usize] as i32, y, color);
        }
    }
}

pub fn draw_wireframe_triangle(
    canvas: &mut Canvas,
    p0: (i32, i32),
    p1: (i32, i32),
    p2: (i32, i32),
    color: Color,
) {
    draw_line(canvas, p0, p1, color);
    draw_line(canvas, p1, p2, color);
    draw_line(canvas, p2, p0, color);
}
