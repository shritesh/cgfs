use crate::Vec3;

pub enum Light {
    Point { position: Vec3, intensity: f64 },
    Directional { direction: Vec3, intensity: f64 },
    Ambient { intensity: f64 },
}
