use std::ops::Sub;
pub struct Vector<const N: usize>(pub [f64; N]);

impl<const N: usize> Vector<N> {
    pub fn dot(&self, rhs: &Self) -> f64 {
        let mut result = 0.0;

        for i in 0..N {
            result += self.0[i] * rhs.0[i];
        }

        result
    }
}

impl<const N: usize> Sub for &Vector<N> {
    type Output = Vector<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result = self.0;

        for i in 0..N {
            result[i] -= rhs.0[i];
        }

        Vector(result)
    }
}
