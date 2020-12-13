use float_cmp::ApproxEq;
use num_traits::{Float, FromPrimitive};
use std::iter::Sum;
use std::ops::{Add, Mul};

use crate::tuple::Tuple;

use super::Matrix;

pub struct Transform<F> {
    data: Matrix<F>,
}

impl<F: Float + FromPrimitive + Sum> Transform<F> {
    pub fn rotation_x(radians: F) -> Self {
        let mut data = Matrix::identity();
        data[1][1] = radians.cos();
        data[1][2] = -radians.sin();
        data[2][1] = radians.sin();
        data[2][2] = radians.cos();

        Self { data }
    }

    pub fn rotation_y(radians: F) -> Self {
        let mut data = Matrix::identity();
        data[0][0] = radians.cos();
        data[0][2] = radians.sin();
        data[2][0] = -radians.sin();
        data[2][2] = radians.cos();

        Self { data }
    }

    pub fn rotation_z(radians: F) -> Self {
        let mut data = Matrix::identity();
        data[0][0] = radians.cos();
        data[0][1] = -radians.sin();
        data[1][0] = radians.sin();
        data[1][1] = radians.cos();

        Self { data }
    }

    pub fn scaling(x: F, y: F, z: F) -> Self {
        let mut data = Matrix::identity();
        data[0][0] = x;
        data[1][1] = y;
        data[2][2] = z;

        Self { data }
    }

    pub fn shearing(xy: F, xz: F, yx: F, yz: F, zx: F, zy: F) -> Self {
        let mut data = Matrix::identity();
        data[0][1] = xy;
        data[0][2] = xz;
        data[1][0] = yx;
        data[1][2] = yz;
        data[2][0] = zx;
        data[2][1] = zy;

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

impl<'a, M: Copy + Default, F: Copy + ApproxEq<Margin = M>> ApproxEq for &'a Transform<F> {
    type Margin = M;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.data.approx_eq(&other.data, margin)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::F64Margin;

    use crate::tuple::Tuple;
    use std::f64::consts::PI;

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

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Transform::rotation_x(PI / 4.0);
        let full_quarter = Transform::rotation_x(PI / 2.0);

        let sqrt2over2 = 2f64.sqrt() / 2.0;
        assert!((half_quarter * p).approx_eq(
            &Tuple::point(0.0, sqrt2over2, sqrt2over2),
            F64Margin::default()
        ));
        assert!((full_quarter * p).approx_eq(&Tuple::point(0.0, 0.0, 1.0), F64Margin::default()));
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = Transform::rotation_y(PI / 4.0);
        let full_quarter = Transform::rotation_y(PI / 2.0);

        let sqrt2over2 = 2f64.sqrt() / 2.0;
        assert!((half_quarter * p).approx_eq(
            &Tuple::point(sqrt2over2, 0.0, sqrt2over2),
            F64Margin::default()
        ));
        assert!((full_quarter * p).approx_eq(&Tuple::point(1.0, 0.0, 0.0), F64Margin::default()));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Transform::rotation_z(PI / 4.0);
        let full_quarter = Transform::rotation_z(PI / 2.0);

        let sqrt2over2 = 2f64.sqrt() / 2.0;
        assert!((half_quarter * p).approx_eq(
            &Tuple::point(-sqrt2over2, sqrt2over2, 0.0),
            F64Margin::default()
        ));
        assert!((full_quarter * p).approx_eq(&Tuple::point(-1.0, 0.0, 0.0), F64Margin::default()));
    }

    #[test]
    fn shearing_x_in_proportion_to_y() {
        let transform = Transform::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_x_in_proportion_to_z() {
        let transform = Transform::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn shearing_y_in_proportion_to_x() {
        let transform = Transform::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn shearing_y_in_proportion_to_z() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn shearing_z_in_proportion_to_x() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn shearing_z_in_proportion_to_y() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 7.0));
    }
}
