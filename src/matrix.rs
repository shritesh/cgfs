use std::ops::Mul;

use crate::Vec3;

#[derive(Clone, Copy)]
pub struct Matrix([[f64; 4]; 4]);

impl Matrix {
    pub const IDENTITY: Self = Self([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    pub fn transpose(&self) -> Self {
        let mut output = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                output[i][j] = self.0[j][i];
            }
        }

        Self(output)
    }

    pub fn translation(vec: Vec3) -> Self {
        Self([
            [1.0, 0.0, 0.0, vec.0],
            [0.0, 1.0, 0.0, vec.1],
            [0.0, 0.0, 1.0, vec.2],
            [0.0, 0.0, 0.0, 1.00],
        ])
    }

    pub fn rotation_y(degrees: f64) -> Self {
        let (sin, cos) = degrees.to_radians().sin_cos();

        Self([
            [cos, 0.0, -sin, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [sin, 0.0, cos, 0.0],
            [0.0, 0.0, 0.0, 1.00],
        ])
    }

    pub fn scaling(t: f64) -> Self {
        Self([
            [t, 0.0, 0.0, 0.0],
            [0.0, t, 0.0, 0.0],
            [0.0, 0.0, t, 0.0],
            [0.0, 0.0, 0.0, 1.00],
        ])
    }
}

impl Mul<Vec3> for Matrix {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let mut out = [0.0; 4];
        let vec = [rhs.0, rhs.1, rhs.2, 1.0];

        for i in 0..4 {
            for j in 0..4 {
                out[i] += self.0[i][j] * vec[j];
            }
        }

        Vec3(out[0], out[1], out[2])
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut output = [[0.0; 4]; 4];

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    output[i][j] += self.0[i][k] * rhs.0[k][j];
                }
            }
        }

        Self(output)
    }
}
