use crate::{Canvas, Color, Light, Sphere, Vec3};

pub struct Scene {
    camera_position: Vec3,
    viewport: Vec3, // width, height, distance to projection plane
    spheres: Vec<Sphere>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new(
        camera_position: Vec3,
        viewport: Vec3,
        spheres: Vec<Sphere>,
        lights: Vec<Light>,
    ) -> Self {
        Self {
            camera_position,
            viewport,
            spheres,
            lights,
        }
    }

    fn canvas_to_viewport(&self, canvas: &Canvas, x: i32, y: i32) -> Vec3 {
        Vec3(
            x as f64 * self.viewport.0 / canvas.width() as f64,
            y as f64 * self.viewport.1 / canvas.height() as f64,
            self.viewport.2,
        )
    }

    fn compute_lighting(&self, point: Vec3, normal: Vec3, view: Vec3, specular: f64) -> f64 {
        let mut i = 0.0;
        for light in &self.lights {
            let (direction, intensity) = match light {
                Light::Ambient { intensity } => {
                    i += intensity;
                    continue;
                }
                Light::Point {
                    position,
                    intensity,
                } => (*position - point, *intensity),
                Light::Directional {
                    direction,
                    intensity,
                } => (*direction, *intensity),
            };

            // diffuse
            let normal_dot_direction = normal.dot(direction);
            if normal_dot_direction > 0.0 {
                i += intensity * normal_dot_direction / (normal.length() * direction.length())
            }

            //specular
            if specular != -1.0 {
                let reflection = 2.0 * normal * normal.dot(direction) - direction;
                let reflection_dot_view = reflection.dot(view);
                if reflection_dot_view > 0.0 {
                    i += intensity
                        * (reflection_dot_view / (reflection.length() * view.length()))
                            .powf(specular)
                }
            }
        }
        i
    }

    fn intersect_ray_sphere(origin: Vec3, direction: Vec3, sphere: &Sphere) -> (f64, f64) {
        let r = sphere.radius;
        let center_to_origin = origin - sphere.center;

        let a = direction.dot(direction);
        let b = 2.0 * center_to_origin.dot(direction);
        let c = center_to_origin.dot(center_to_origin) - r * r;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            (f64::INFINITY, f64::INFINITY)
        } else {
            let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b - discriminant.sqrt()) / (2.0 * a);

            (t1, t2)
        }
    }

    fn trace_ray(&self, origin: Vec3, direction: Vec3, t_min: f64, t_max: f64) -> Color {
        let mut closest_t = f64::INFINITY;
        let mut closest_sphere = None;

        for sphere in &self.spheres {
            let (t1, t2) = Self::intersect_ray_sphere(origin, direction, sphere);

            if t1 >= t_min && t1 <= t_max && t1 < closest_t {
                closest_t = t1;
                closest_sphere = Some(sphere);
            }
            if t2 >= t_min && t2 <= t_max && t2 < closest_t {
                closest_t = t2;
                closest_sphere = Some(sphere);
            }
        }

        if let Some(sphere) = closest_sphere {
            let point = origin + closest_t * direction;
            let normal = (point - sphere.center).unit();
            sphere.color * self.compute_lighting(point, normal, -direction, sphere.specular)
        } else {
            Color::BACKGROUND_COLOR
        }
    }

    pub fn render(&self, canvas: &mut Canvas) {
        let canvas_width = canvas.width() as i32;
        let canvas_height = canvas.height() as i32;

        for x in -canvas_width / 2..canvas_width / 2 {
            for y in -canvas_height / 2..canvas_height / 2 {
                let direction = self.canvas_to_viewport(canvas, x, y);
                let color = self.trace_ray(self.camera_position, direction, 1.0, f64::INFINITY);
                canvas.put_pixel(x, y, color);
            }
        }
    }
}
