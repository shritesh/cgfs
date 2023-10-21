use std::ops::{Add, Mul};

#[derive(Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    pub const BACKGROUND_COLOR: Color = Color(255, 255, 255);
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let r = self.0.saturating_add(rhs.0);
        let g = self.1.saturating_add(rhs.1);
        let b = self.2.saturating_add(rhs.2);

        Self(r, g, b)
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let r = self.0 as f64 * rhs;
        let g = self.1 as f64 * rhs;
        let b = self.2 as f64 * rhs;

        Self(
            r.clamp(0.0, 255.0) as u8,
            g.clamp(0.0, 255.0) as u8,
            b.clamp(0.0, 255.0) as u8,
        )
    }
}
