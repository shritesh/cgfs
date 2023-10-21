use crate::{Canvas, Color, Vec3};

pub struct Raytracer<'a> {
    camera_position: Vec3,
    viewport: Vec3, // width, height, distance to projection plane
    spheres: &'a [Sphere],
    lights: &'a [Light],
}

pub enum Light {
    Point { position: Vec3, intensity: f64 },
    Directional { direction: Vec3, intensity: f64 },
    Ambient { intensity: f64 },
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub color: Color,
    pub specular: f64,
    pub reflective: f64,
}

impl<'a> Raytracer<'a> {
    pub const DEFAULT_SCENE: Self = Self {
        camera_position: Vec3(0.0, 0.0, 0.0),
        viewport: Vec3(1.0, 1.0, 1.0),
        spheres: &[
            Sphere {
                center: Vec3(0.0, -1.0, 3.0),
                radius: 1.0,
                color: Color(255, 0, 0),
                specular: 500.0,
                reflective: 0.2,
            },
            Sphere {
                center: Vec3(2.0, 0.0, 4.0),
                radius: 1.0,
                color: Color(0, 0, 255),
                specular: 500.0,
                reflective: 0.3,
            },
            Sphere {
                center: Vec3(-2.0, 0.0, 4.0),
                radius: 1.0,
                color: Color(0, 255, 0),
                specular: 10.0,
                reflective: 0.4,
            },
            Sphere {
                center: Vec3(0.0, -5001.0, 0.0),
                radius: 5000.0,
                color: Color(255, 255, 0),
                specular: 1000.0,
                reflective: 0.5,
            },
        ],
        lights: &[
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
    };

    fn canvas_to_viewport(&self, canvas: &Canvas, x: i32, y: i32) -> Vec3 {
        Vec3(
            x as f64 * self.viewport.0 / canvas.width() as f64,
            y as f64 * self.viewport.1 / canvas.height() as f64,
            self.viewport.2,
        )
    }

    fn compute_lighting(&self, point: Vec3, normal: Vec3, view: Vec3, specular: f64) -> f64 {
        let mut i = 0.0;
        for light in self.lights {
            let (direction, intensity, t_max) = match light {
                Light::Ambient { intensity } => {
                    i += intensity;
                    continue;
                }
                Light::Point {
                    position,
                    intensity,
                } => (*position - point, *intensity, 1.0),
                Light::Directional {
                    direction,
                    intensity,
                } => (*direction, *intensity, f64::INFINITY),
            };

            // shadow_check
            if self.any_intersection(point, direction, 0.001, t_max) {
                continue;
            }

            // diffuse
            let normal_dot_direction = normal.dot(direction);
            if normal_dot_direction > 0.0 {
                i += intensity * normal_dot_direction / (normal.length() * direction.length())
            }

            //specular
            if specular != -1.0 {
                let reflection = direction.reflect(normal);
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

    fn closest_intesection(
        &self,
        origin: Vec3,
        direction: Vec3,
        t_min: f64,
        t_max: f64,
    ) -> (f64, Option<&Sphere>) {
        let mut closest_t = f64::INFINITY;
        let mut closest_sphere = None;

        for sphere in self.spheres {
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

        (closest_t, closest_sphere)
    }

    // optimized early-exit for shadow check
    fn any_intersection(&self, origin: Vec3, direction: Vec3, t_min: f64, t_max: f64) -> bool {
        self.spheres.iter().any(|sphere| {
            let (t1, t2) = Self::intersect_ray_sphere(origin, direction, sphere);
            (t1 >= t_min && t1 <= t_max) || (t2 >= t_min && t2 <= t_max)
        })
    }

    fn trace_ray(
        &self,
        origin: Vec3,
        direction: Vec3,
        t_min: f64,
        t_max: f64,
        recursion_depth: u8,
    ) -> Color {
        if let (closest_t, Some(sphere)) = self.closest_intesection(origin, direction, t_min, t_max)
        {
            let point = origin + closest_t * direction;
            let normal = (point - sphere.center).unit();
            let local_color =
                sphere.color * self.compute_lighting(point, normal, -direction, sphere.specular);

            let r = sphere.reflective;
            if recursion_depth == 0 || r <= 0.0 {
                local_color
            } else {
                let reflection = (-direction).reflect(normal);
                let reflected_color =
                    self.trace_ray(point, reflection, 0.001, f64::INFINITY, recursion_depth - 1);

                local_color * (1.0 - r) + reflected_color * r
            }
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
                let color = self.trace_ray(self.camera_position, direction, 1.0, f64::INFINITY, 3);
                canvas.put_pixel(x, y, color);
            }
        }
    }
}
