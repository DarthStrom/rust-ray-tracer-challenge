use num_traits::{Float, FromPrimitive};
use std::iter::Sum;
use std::ops::{Add, Mul};

use crate::tuple::Tuple;

use super::Matrix;

pub struct Transform<F> {
    data: Matrix<F>,
}

impl<F: Float + FromPrimitive + Sum> Transform<F> {
    pub fn scaling(x: F, y: F, z: F) -> Self {
        let mut data = Matrix::identity();
        data[0][0] = x;
        data[1][1] = y;
        data[2][2] = z;

        Self { data }
    }

    pub fn translation(x: F, y: F, z: F) -> Self {
        let mut data = Matrix::identity();
        data[0][3] = x;
        data[1][3] = y;
        data[2][3] = z;

        Self { data }
    }

    pub fn inverse(&self) -> Result<Transform<F>, String> {
        Ok(Transform {
            data: self.data.inverse()?,
        })
    }
}

impl<F: Float + FromPrimitive + Mul<Output = F> + Add<Output = F>> Mul<Tuple<F>> for Transform<F> {
    type Output = Tuple<F>;

    fn mul(self, rhs: Tuple<F>) -> Self::Output {
        self.data * rhs
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

    use super::*;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Transform::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(transform * p, Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Transform::translation(5.0, -3.0, 2.0);
        let inv = transform.inverse().unwrap();
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Transform::translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v);
    }

    #[test]
    fn scaling_matrix_applied_to_point() {
        let transform = Transform::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        assert_eq!(transform * p, Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let transform = Transform::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(transform * v, Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Transform::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().unwrap();
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let transform = Transform::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(-2.0, 3.0, 4.0));
    }
}
