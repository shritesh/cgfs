use macroquad::prelude::*;

use crate::canvas::Canvas;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Color,
}

pub struct Scene {
    viewport: Vec2,
    projection_distance: f32,
    background_color: Color,
    spheres: Vec<Sphere>,
}

impl Scene {
    pub fn new(viewport: Vec2, projection_distance: f32) -> Self {
        Self {
            viewport,
            projection_distance,
            background_color: WHITE,
            spheres: Vec::new(),
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn render(&self, canvas: &mut Canvas) {
        let origin = Vec3::ZERO;

        for x in canvas.width() as i32 / -2..canvas.width() as i32 / 2 {
            for y in canvas.height() as i32 / -2..canvas.height() as i32 / 2 {
                let direction = self.canvas_to_viewport(canvas, x, y);
                let color = self.trace_ray(origin, direction, 1.0, f32::INFINITY);
                canvas.put_pixel(x, y, color);
            }
        }
    }

    fn canvas_to_viewport(&self, canvas: &Canvas, x: i32, y: i32) -> Vec3 {
        Vec3::new(
            x as f32 * self.viewport.x / canvas.width() as f32,
            y as f32 * self.viewport.y / canvas.height() as f32,
            self.projection_distance,
        )
    }

    fn trace_ray(&self, origin: Vec3, direction: Vec3, t_min: f32, t_max: f32) -> Color {
        let mut closest_t = f32::INFINITY;
        let mut closest_sphere: Option<&Sphere> = None;

        for sphere in &self.spheres {
            for t in self.intersect_ray_sphere(origin, direction, sphere) {
                if t >= t_min && t <= t_max && t < closest_t {
                    closest_t = t;
                    closest_sphere = Some(sphere);
                }
            }
        }

        if let Some(sphere) = closest_sphere {
            sphere.color
        } else {
            self.background_color
        }
    }

    fn intersect_ray_sphere(&self, origin: Vec3, direction: Vec3, sphere: &Sphere) -> [f32; 2] {
        let r = sphere.radius;
        let origin_to_center = origin - sphere.center;

        let a = direction.dot(direction);
        let b = 2.0 * origin_to_center.dot(direction);
        let c = origin_to_center.dot(origin_to_center) - r * r;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            [f32::INFINITY, f32::INFINITY]
        } else {
            let square_root = discriminant.sqrt();

            [
                (-b + square_root) / (2.0 * a),
                (-b - square_root) / (2.0 * a),
            ]
        }
    }
}
