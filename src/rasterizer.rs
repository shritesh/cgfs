use std::f64::consts::PI;

use crate::{Canvas, Color, Matrix, Renderer, Vec3};

#[derive(Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy)]
struct Triangle(pub usize, pub usize, pub usize, pub Color, pub [Vec3; 3]);

struct Plane {
    normal: Vec3,
    distance: f64,
}

impl Plane {
    fn signed_distance(&self, vertex: Vec3) -> f64 {
        self.normal.dot(vertex) + self.distance
    }

    fn clip_count(&self, triangle: &Triangle, vertices: &[Vec3]) -> usize {
        let v0 = vertices[triangle.0];
        let v1 = vertices[triangle.1];
        let v2 = vertices[triangle.2];

        [v0, v1, v2]
            .into_iter()
            .filter(|v| self.signed_distance(*v) > 0.0)
            .count()
    }
}

struct Model {
    vertices: Vec<Vec3>,
    triangles: Vec<Triangle>,
    bounds_center: Vec3,
    bounds_radius: f64,
}

impl Model {
    fn sphere(divs: u32, color: Color) -> Self {
        let delta_angle = 2.0 * PI / divs as f64;

        let mut vertices = Vec::new();
        let mut triangles = Vec::new();

        for d in 0..=divs {
            let y = (2.0 / divs as f64) * (d as f64 - divs as f64 / 2.0);
            let radius = (1.0 - y * y).sqrt();
            for i in 0..divs {
                let vertex = Vec3(
                    radius * (i as f64 * delta_angle).cos(),
                    y,
                    radius * (i as f64 * delta_angle).sin(),
                );
                vertices.push(vertex);
            }
        }

        for d in 0..divs {
            for i in 0..divs {
                let i0 = d * divs + i;
                let i1 = (d + 1) * divs + (i + 1) % divs;
                let i2 = divs * d + (i + 1) % divs;
                let tri0 = [i0 as usize, i1 as usize, i2 as usize];
                let tri1 = [i0 as usize, (i0 + divs) as usize, i1 as usize];
                triangles.push(Triangle(
                    tri0[0],
                    tri0[1],
                    tri0[2],
                    color,
                    [vertices[tri0[0]], vertices[tri0[1]], vertices[tri0[2]]],
                ));
                triangles.push(Triangle(
                    tri1[0],
                    tri1[1],
                    tri1[2],
                    color,
                    [vertices[tri1[0]], vertices[tri1[1]], vertices[tri1[2]]],
                ));
            }
        }

        Model {
            vertices,
            triangles,
            bounds_center: Vec3(0.0, 0.0, 0.0),
            bounds_radius: 1.0,
        }
    }
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

struct Instance {
    model_idx: usize,
    transform: Transform,
}

struct Camera {
    position: Vec3,
    rotation: f64,
    clipping_planes: Vec<Plane>,
}

enum Light {
    Point { position: Vec3, intensity: f64 },
    Directional { direction: Vec3, intensity: f64 },
    Ambient { intensity: f64 },
}

enum ShadingModel {
    Flat,
    Gouraud,
    Phong,
}

pub struct Rasterizer {
    camera: Camera,
    models: Vec<Model>,
    instances: Vec<Instance>,
    lights: Vec<Light>,
    shading_model: ShadingModel,
}

impl Rasterizer {
    fn render_triangle(
        &self,
        canvas: &mut Canvas,
        triangle: &Triangle,
        vertices: &[Vec3],
        projected: &[Point],
        rotation: f64,
    ) {
        let (mut v0, mut v1, mut v2) = (
            vertices[triangle.0],
            vertices[triangle.1],
            vertices[triangle.2],
        );

        let normal = triangle_normal(v0, v1, v2);
        // backface culling
        let camera_rotation_matrix = Matrix::rotation_y(self.camera.rotation).transpose();
        let vertex_to_camera =
            camera_rotation_matrix * self.camera.position - camera_rotation_matrix * v0;
        if vertex_to_camera.dot(normal) <= 0.0 {
            return;
        }

        let (mut p0, mut p1, mut p2) = (
            projected[triangle.0],
            projected[triangle.1],
            projected[triangle.2],
        );

        let transform = camera_rotation_matrix * Matrix::rotation_y(rotation);
        let (mut normal0, mut normal1, mut normal2) = (
            transform * triangle.4[0],
            transform * triangle.4[1],
            transform * triangle.4[2],
        );
        // sort according to y
        if p1.y < p0.y {
            std::mem::swap(&mut p1, &mut p0);
            std::mem::swap(&mut v1, &mut v0);
            std::mem::swap(&mut normal1, &mut normal0);
        }
        if p2.y < p0.y {
            std::mem::swap(&mut p2, &mut p0);
            std::mem::swap(&mut v2, &mut v0);
            std::mem::swap(&mut normal2, &mut normal0);
        }
        if p2.y < p1.y {
            std::mem::swap(&mut p2, &mut p1);
            std::mem::swap(&mut v2, &mut v1);
            std::mem::swap(&mut normal2, &mut normal1);
        }

        let (x02, x012) = edge_interpolate(p0.y, p0.x, p1.y, p1.x, p2.y, p2.x);
        let (z02, z012) = edge_interpolate(p0.y, 1.0 / v0.2, p1.y, 1.0 / v1.2, p2.y, 1.0 / v2.2);

        // flat
        let intesity = illumination((v0 + v1 + v2) / 3.0, normal, &self.camera, &self.lights);

        // gouraud
        let i0 = illumination(v0, normal0, &self.camera, &self.lights);
        let i1 = illumination(v1, normal1, &self.camera, &self.lights);
        let i2 = illumination(v2, normal2, &self.camera, &self.lights);
        let (i02, i012) = edge_interpolate(p0.y, i0, p1.y, i1, p2.y, i2);

        let m = x02.len() / 2;
        let (x_left, x_right, z_left, z_right, i_left, i_right) = if x02[m] < x012[m] {
            (x02, x012, z02, z012, i02, i012)
        } else {
            (x012, x02, z012, z02, i012, i02)
        };

        for ((((((y, left_x), right_x), left_z), right_z), left_i), right_i) in (p0.y..=p2.y)
            .zip(x_left)
            .zip(x_right)
            .zip(z_left)
            .zip(z_right)
            .zip(i_left)
            .zip(i_right)
        {
            let (lx, rx) = (left_x as i32, right_x as i32);

            match self.shading_model {
                ShadingModel::Flat => {
                    for (x, z) in (lx..=rx).zip(interpolate(lx, left_z, rx, right_z)) {
                        if canvas.update_depth_buffer(x, y, z) {
                            canvas.put_pixel(x, y, triangle.3 * intesity);
                        }
                    }
                }
                ShadingModel::Gouraud => {
                    for ((x, z), i) in (lx..=rx)
                        .zip(interpolate(lx, left_z, rx, right_z))
                        .zip(interpolate(lx, left_i, rx, right_i))
                    {
                        if canvas.update_depth_buffer(x, y, z) {
                            canvas.put_pixel(x, y, triangle.3 * i);
                        }
                    }
                }
                ShadingModel::Phong => todo!(),
            }
        }
    }
    fn render_model(&self, canvas: &mut Canvas, model: &Model, rotation: f64) {
        let projected: Vec<Point> = model
            .vertices
            .iter()
            .map(|v| project_vertex(canvas, *v))
            .collect();

        for t in &model.triangles {
            self.render_triangle(canvas, &t, &model.vertices, &projected, rotation);
        }
    }

    pub fn default_scene() -> Self {
        let cube = Model {
            vertices: vec![
                Vec3(1.0, 1.0, 1.0),
                Vec3(-1.0, 1.0, 1.0),
                Vec3(-1.0, -1.0, 1.0),
                Vec3(1.0, -1.0, 1.0),
                Vec3(1.0, 1.0, -1.0),
                Vec3(-1.0, 1.0, -1.0),
                Vec3(-1.0, -1.0, -1.0),
                Vec3(1.0, -1.0, -1.0),
            ],
            triangles: vec![
                Triangle(
                    0,
                    1,
                    2,
                    Color::RED,
                    [
                        Vec3(0.0, 0.0, 1.0),
                        Vec3(0.0, 0.0, 1.0),
                        Vec3(0.0, 0.0, 1.0),
                    ],
                ),
                Triangle(
                    0,
                    2,
                    3,
                    Color::RED,
                    [
                        Vec3(0.0, 0.0, 1.0),
                        Vec3(0.0, 0.0, 1.0),
                        Vec3(0.0, 0.0, 1.0),
                    ],
                ),
                Triangle(
                    4,
                    0,
                    3,
                    Color::GREEN,
                    [
                        Vec3(1.0, 0.0, 0.0),
                        Vec3(1.0, 0.0, 0.0),
                        Vec3(1.0, 0.0, 0.0),
                    ],
                ),
                Triangle(
                    4,
                    3,
                    7,
                    Color::GREEN,
                    [
                        Vec3(1.0, 0.0, 0.0),
                        Vec3(1.0, 0.0, 0.0),
                        Vec3(1.0, 0.0, 0.0),
                    ],
                ),
                Triangle(
                    5,
                    4,
                    7,
                    Color::BLUE,
                    [
                        Vec3(0.0, 0.0, -1.0),
                        Vec3(0.0, 0.0, -1.0),
                        Vec3(0.0, 0.0, -1.0),
                    ],
                ),
                Triangle(
                    5,
                    7,
                    6,
                    Color::BLUE,
                    [
                        Vec3(0.0, 0.0, -1.0),
                        Vec3(0.0, 0.0, -1.0),
                        Vec3(0.0, 0.0, -1.0),
                    ],
                ),
                Triangle(
                    1,
                    5,
                    6,
                    Color::YELLOW,
                    [
                        Vec3(-1.0, 0.0, 0.0),
                        Vec3(-1.0, 0.0, 0.0),
                        Vec3(-1.0, 0.0, 0.0),
                    ],
                ),
                Triangle(
                    1,
                    6,
                    2,
                    Color::YELLOW,
                    [
                        Vec3(-1.0, 0.0, 0.0),
                        Vec3(-1.0, 0.0, 0.0),
                        Vec3(-1.0, 0.0, 0.0),
                    ],
                ),
                Triangle(
                    1,
                    0,
                    5,
                    Color::PURPLE,
                    [
                        Vec3(0.0, 1.0, 0.0),
                        Vec3(0.0, 1.0, 0.0),
                        Vec3(0.0, 1.0, 0.0),
                    ],
                ),
                Triangle(
                    5,
                    0,
                    4,
                    Color::PURPLE,
                    [
                        Vec3(0.0, 1.0, 0.0),
                        Vec3(0.0, 1.0, 0.0),
                        Vec3(0.0, 1.0, 0.0),
                    ],
                ),
                Triangle(
                    2,
                    6,
                    7,
                    Color::CYAN,
                    [
                        Vec3(0.0, -1.0, 0.0),
                        Vec3(0.0, -1.0, 0.0),
                        Vec3(0.0, -1.0, 0.0),
                    ],
                ),
                Triangle(
                    2,
                    7,
                    3,
                    Color::CYAN,
                    [
                        Vec3(0.0, -1.0, 0.0),
                        Vec3(0.0, -1.0, 0.0),
                        Vec3(0.0, -1.0, 0.0),
                    ],
                ),
            ],
            bounds_center: Vec3(0.0, 0.0, 0.0),
            bounds_radius: 3.0f64.sqrt(),
        };

        let sphere = Model::sphere(15, Color::GREEN);

        let s2 = 1.0 / 2.0f64.sqrt();

        Self {
            models: vec![cube, sphere],
            camera: Camera {
                position: Vec3(-3.0, 1.0, 2.0),
                rotation: -30.0,
                clipping_planes: vec![
                    Plane {
                        // near
                        normal: Vec3(0.0, 0.0, 1.0),
                        distance: -1.0,
                    },
                    Plane {
                        // left
                        normal: Vec3(s2, 0.0, s2),
                        distance: 0.0,
                    },
                    Plane {
                        // right
                        normal: Vec3(-s2, 0.0, s2),
                        distance: 0.0,
                    },
                    Plane {
                        // top
                        normal: Vec3(0.0, -s2, s2),
                        distance: 0.0,
                    },
                    Plane {
                        // bottom
                        normal: Vec3(0.0, s2, s2),
                        distance: 0.0,
                    },
                ],
            },
            instances: vec![
                Instance {
                    model_idx: 0,
                    transform: Transform {
                        scale: 0.75,
                        rotation: 0.0,
                        position: Vec3(-1.5, 0.0, 7.0),
                    },
                },
                Instance {
                    model_idx: 0,
                    transform: Transform {
                        scale: 1.0,
                        rotation: 195.0,
                        position: Vec3(1.25, 2.0, 7.5),
                    },
                },
                Instance {
                    model_idx: 1,
                    transform: Transform {
                        scale: 1.75,
                        rotation: 0.0,
                        position: Vec3(1.75, -0.5, 7.0),
                    },
                },
            ],
            lights: vec![
                Light::Ambient { intensity: 0.2 },
                Light::Directional {
                    direction: Vec3(1.0, 0.0, 1.0),
                    intensity: 0.2,
                },
                Light::Point {
                    position: Vec3(-3.0, 2.0, -10.0),
                    intensity: 0.6,
                },
            ],
            shading_model: ShadingModel::Gouraud,
        }
    }
}

impl Renderer for Rasterizer {
    fn render(&self, canvas: &mut Canvas) {
        let camera_matrix = Matrix::rotation_y(self.camera.rotation).transpose()
            * Matrix::translation(-1.0 * self.camera.position);

        for instance in &self.instances {
            let transform_matrix = camera_matrix * instance.transform.matrix();

            if let Some(clipped_model) = transform_and_clip(
                &self.camera.clipping_planes,
                &self.models[instance.model_idx],
                instance.transform.scale,
                transform_matrix,
            ) {
                self.render_model(canvas, &clipped_model, instance.transform.rotation);
            }
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

fn transform_and_clip(
    clipping_planes: &[Plane],
    model: &Model,
    scale: f64,
    transform_matrix: Matrix,
) -> Option<Model> {
    let center = transform_matrix * model.bounds_center;
    let radius = model.bounds_radius * scale;

    if clipping_planes
        .iter()
        .any(|cp| cp.signed_distance(center) < -radius)
    {
        return None;
    }

    let vertices: Vec<_> = model
        .vertices
        .iter()
        .map(|v| transform_matrix * *v)
        .collect();

    // TODO: Generate new vertices for other clip counts
    let triangles = clipping_planes
        .iter()
        .fold(model.triangles.clone(), |triangles, plane| {
            triangles
                .into_iter()
                .filter(|t| plane.clip_count(t, &vertices) == 3)
                .collect()
        });

    Some(Model {
        vertices,
        triangles,
        bounds_center: model.bounds_center,
        bounds_radius: model.bounds_radius,
    })
}

fn interpolate<T: Into<f64> + Copy>(i0: i32, d0: T, i1: i32, d1: T) -> Vec<f64> {
    let mut values = Vec::new();

    if i0 == i1 {
        values.push(d0.into());
    } else {
        let a = (d1.into() - d0.into()) / (i1 - i0) as f64;
        let mut d = d0.into();
        for _ in i0..=i1 {
            values.push(d);
            d += a;
        }
    }

    values
}

fn edge_interpolate<T: Into<f64> + Copy>(
    y0: i32,
    x0: T,
    y1: i32,
    x1: T,
    y2: i32,
    x2: T,
) -> (Vec<f64>, Vec<f64>) {
    let mut x01 = interpolate(y0, x0, y1, x1);
    let x12 = interpolate(y1, x1, y2, x2);
    let x02 = interpolate(y0, x0, y2, x2);

    _ = x01.pop();
    let x012 = [x01, x12].concat();

    (x02, x012)
}

fn triangle_normal(v0: Vec3, v1: Vec3, v2: Vec3) -> Vec3 {
    (v1 - v0).cross(v2 - v0)
}

fn illumination(vertex: Vec3, normal: Vec3, camera: &Camera, lights: &[Light]) -> f64 {
    let mut il = 0.0;

    for light in lights {
        let (intensity, vector) = match light {
            Light::Ambient { intensity } => {
                il += intensity;
                continue;
            }

            Light::Directional {
                direction,
                intensity,
            } => {
                let camera_matrix = Matrix::rotation_y(camera.rotation).transpose();
                (intensity, camera_matrix * *direction)
            }

            Light::Point {
                position,
                intensity,
            } => {
                let camera_matrix = Matrix::rotation_y(camera.rotation).transpose()
                    * Matrix::translation(-camera.position);
                (intensity, camera_matrix * *position + -vertex)
            }
        };

        // diffuse
        let cos_alpha = vector.dot(normal) / (vector.length() * normal.length());
        if cos_alpha > 0.0 {
            il += cos_alpha * intensity;
        }

        //specular
        let reflected = vector.reflect(normal);
        let view = camera.position - vertex;

        let cos_beta = reflected.dot(view) / (reflected.length() * view.length());
        if cos_beta > 0.0 {
            let specular = 50.0;
            il += cos_beta.powf(specular) * intensity;
        }
    }

    il
}

const VIEWPORT_WIDTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 1.0;
const DISTANCE: f64 = 1.0;

fn viewport_to_canvas(canvas: &Canvas, x: f64, y: f64) -> Point {
    Point {
        x: (x * canvas.width() as f64 / VIEWPORT_WIDTH) as i32,
        y: (y * canvas.height() as f64 / VIEWPORT_HEIGHT) as i32,
    }
}

fn project_vertex(canvas: &Canvas, v: Vec3) -> Point {
    viewport_to_canvas(canvas, v.0 * DISTANCE / v.2, v.1 * DISTANCE / v.2)
}
