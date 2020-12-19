use float_cmp::{ApproxEq, F64Margin};
use std::ops::Mul;

use crate::tuple::Tuple;

use super::Matrix;

#[derive(Clone, Debug, PartialEq)]
pub struct Transform {
    data: Matrix,
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            data: Matrix::identity(),
        }
    }

    pub fn rotation_x(radians: f64) -> Self {
        Transform::identity().rotate_x(radians)
    }

    pub fn rotation_y(radians: f64) -> Self {
        Transform::identity().rotate_y(radians)
    }

    pub fn rotation_z(radians: f64) -> Self {
        Transform::identity().rotate_z(radians)
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        Transform::identity().scale(x, y, z)
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Transform::identity().shear(xy, xz, yx, yz, zx, zy)
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Transform::identity().translate(x, y, z)
    }

    pub fn rotate_x(&self, radians: f64) -> Self {
        let mut data = Matrix::identity();
        data[1][1] = radians.cos();
        data[1][2] = -radians.sin();
        data[2][1] = radians.sin();
        data[2][2] = radians.cos();

        Self {
            data: data * self.data.clone(),
        }
    }

    pub fn rotate_y(&self, radians: f64) -> Self {
        let mut data = Matrix::identity();
        data[0][0] = radians.cos();
        data[0][2] = radians.sin();
        data[2][0] = -radians.sin();
        data[2][2] = radians.cos();

        Self {
            data: data * self.data.clone(),
        }
    }

    pub fn rotate_z(&self, radians: f64) -> Self {
        let mut data = Matrix::identity();
        data[0][0] = radians.cos();
        data[0][1] = -radians.sin();
        data[1][0] = radians.sin();
        data[1][1] = radians.cos();

        Self {
            data: data * self.data.clone(),
        }
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Self {
        let mut data = Matrix::identity();
        data[0][0] = x;
        data[1][1] = y;
        data[2][2] = z;

        Self {
            data: data * self.data.clone(),
        }
    }

    pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let mut data = Matrix::identity();
        data[0][1] = xy;
        data[0][2] = xz;
        data[1][0] = yx;
        data[1][2] = yz;
        data[2][0] = zx;
        data[2][1] = zy;

        Self {
            data: data * self.data.clone(),
        }
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Self {
        let mut data = Matrix::identity();
        data[0][3] = x;
        data[1][3] = y;
        data[2][3] = z;

        Self {
            data: data * self.data.clone(),
        }
    }

    pub fn inverse(&self) -> Result<Transform, String> {
        Ok(Transform {
            data: self.data.inverse()?,
        })
    }

    pub fn is_invertible(&self) -> bool {
        self.data.is_invertible()
    }

    pub fn transpose(&self) -> Self {
        Self {
            data: self.data.transpose(),
        }
    }
}

impl Mul for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            data: self.data * rhs.data,
        }
    }
}

impl Mul<Tuple> for Transform {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        self.data * rhs
    }
}

impl<'a> ApproxEq for &'a Transform {
    type Margin = F64Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.data.approx_eq(&other.data, margin)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::F64Margin;

    use crate::{tuple::Tuple, MARGIN};
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

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Transform::rotation_x(PI / 2.0);
        let b = Transform::scaling(5.0, 5.0, 5.0);
        let c = Transform::translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert!(p2.approx_eq(&Tuple::point(1.0, -1.0, 0.0), MARGIN));

        let p3 = b * p2;
        assert!(p3.approx_eq(&Tuple::point(5.0, -5.0, 0.0), MARGIN));

        let p4 = c * p3;
        assert!(p4.approx_eq(&Tuple::point(15.0, 0.0, 7.0), MARGIN));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Transform::rotation_x(PI / 2.0);
        let b = Transform::scaling(5.0, 5.0, 5.0);
        let c = Transform::translation(10.0, 5.0, 7.0);

        let t = c * b * a;

        assert!((t * p).approx_eq(&Tuple::point(15.0, 0.0, 7.0), MARGIN));
    }

    #[test]
    fn fluent_api() {
        let rx = Transform::rotation_x(PI / 2.0);
        let ry = Transform::rotation_y(PI / 3.0);
        let rz = Transform::rotation_z(PI / 4.0);
        let s = Transform::scaling(5.0, 5.0, 5.0);
        let sh = Transform::shearing(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        let t = Transform::translation(10.0, 5.0, 7.0);

        let fluent_things = Transform::identity()
            .rotate_x(PI / 2.0)
            .rotate_y(PI / 3.0)
            .rotate_z(PI / 4.0)
            .scale(5.0, 5.0, 5.0)
            .shear(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)
            .translate(10.0, 5.0, 7.0);
        let individual_things = t * sh * s * rz * ry * rx;

        assert!(fluent_things.approx_eq(&individual_things, MARGIN));
    }
}
