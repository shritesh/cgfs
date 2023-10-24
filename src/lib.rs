mod canvas;
mod color;
mod matrix;
mod rasterizer;
mod raytracer;
mod vec3;

use color::Color;
use matrix::Matrix;
use vec3::Vec3;

pub use canvas::{Canvas, Renderer};
pub use rasterizer::Rasterizer;
pub use raytracer::Raytracer;
