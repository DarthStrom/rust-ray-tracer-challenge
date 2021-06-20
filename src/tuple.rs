#![allow(dead_code)]

use std::ops::{Add, Div, Mul, Neg, Sub};

use bevy::math::{Vec3A, Vec4};
use float_cmp::approx_eq;

#[derive(Clone, Copy, Debug, Default)]
pub struct Tuple(Vec4);

impl Tuple {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(Vec4::new(x, y, z, w))
    }

    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, y, z, 1.0)
    }

    pub fn vector(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, y, z, 0.0)
    }

    pub fn is_point(self) -> bool {
        self.0.w > 0.0
    }

    pub fn is_vector(self) -> bool {
        self.0.w == 0.0
    }

    pub fn to_point(self) -> Self {
        let mut vector = self.0;
        vector.w = 1.0;
        Self(vector)
    }

    pub fn to_vector(self) -> Self {
        let mut vector = self.0;
        vector.w = 0.0;
        Self(vector)
    }

    pub fn magnitude(self) -> f32 {
        self.0.length()
    }

    pub fn x(self) -> f32 {
        self.0.x
    }

    pub fn y(self) -> f32 {
        self.0.y
    }

    pub fn z(self) -> f32 {
        self.0.z
    }

    pub fn reflect(self, normal: Tuple) -> Self {
        self - normal * 2.0 * self.dot(normal)
    }

    pub fn normalize(self) -> Self {
        Self(self.0.normalize())
    }

    pub fn dot(self, other: Self) -> f32 {
        self.0.dot(other.0)
    }

    pub fn cross(self, other: Self) -> Self {
        let vec1 = Vec3A::from(self.0);
        let vec2 = Vec3A::from(other.0);
        let cross = vec1.cross(vec2);
        Self(Vec4::new(cross.x, cross.y, cross.z, self.0.w))
    }

    pub fn vec(self) -> Vec4 {
        self.0
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        let epsilon = 0.00001;
        approx_eq!(f32, self.0.x, other.0.x, epsilon = epsilon)
            && approx_eq!(f32, self.0.y, other.0.y, epsilon = epsilon)
            && approx_eq!(f32, self.0.z, other.0.z, epsilon = epsilon)
            && approx_eq!(f32, self.0.w, other.0.w, epsilon = epsilon)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Tuple(self.0 + rhs.0)
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Tuple(self.0 - rhs.0)
    }
}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple(-self.0)
    }
}

impl Mul<f32> for Tuple {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Tuple(self.0 * rhs)
    }
}

impl Div<f32> for Tuple {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Tuple(self.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test::*;

    #[test]
    fn tuple_with_w_1_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

        approx_eq!(f32, a.0.x, 4.3);
        approx_eq!(f32, a.0.y, -4.2);
        approx_eq!(f32, a.0.z, 3.1);
        approx_eq!(f32, a.0.w, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_with_w_0_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

        approx_eq!(f32, a.0.x, 4.3);
        approx_eq!(f32, a.0.y, -4.2);
        approx_eq!(f32, a.0.z, 3.1);
        approx_eq!(f32, a.0.w, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_creates_tuples_with_w_1() {
        let p = Tuple::point(4.0, -4.0, 3.0);

        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn vector_creates_tuples_with_w_1() {
        let p = Tuple::vector(4.0, -4.0, 3.0);

        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);

        assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);

        assert_eq!(zero - v, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn computing_the_magnitude_of_vector_1_0_0() {
        let v = Tuple::vector(1.0, 0.0, 0.0);

        approx_eq!(f32, v.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_vector_0_1_0() {
        let v = Tuple::vector(0.0, 1.0, 0.0);

        approx_eq!(f32, v.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_vector_0_0_1() {
        let v = Tuple::vector(0.0, 0.0, 1.0);

        approx_eq!(f32, v.magnitude(), 1.0);
    }

    #[test]
    fn computing_the_magnitude_of_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        approx_eq!(f32, v.magnitude(), 14_f32.sqrt());
    }

    #[test]
    fn computing_the_magnitude_of_vector_neg1_neg2_neg3() {
        let v = Tuple::vector(-1.0, -2.0, -3.0);

        approx_eq!(f32, v.magnitude(), 14_f32.sqrt());
    }

    #[test]
    fn normalizing_vector_4_0_0_gives_1_0_0() {
        let v = Tuple::vector(4.0, 0.0, 0.0);

        assert_eq!(v.normalize(), Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        assert_eq!(
            v.normalize(),
            Tuple::vector(
                1.0 / 14_f32.sqrt(),
                2.0 / 14_f32.sqrt(),
                3.0 / 14_f32.sqrt()
            )
        );
    }

    #[test]
    fn the_magnitude_of_a_normalized_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);

        let norm = v.normalize();

        approx_eq!(f32, norm.magnitude(), 1.0);
    }

    #[test]
    fn the_dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        approx_eq!(f32, a.dot(b), 20.0);
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);

        assert_eq!(a.cross(b), Tuple::vector(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(a), Tuple::vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn reflecting_a_vector_approacing_at_45_deg() {
        let v = Tuple::vector(1.0, -1.0, 0.0);
        let n = Tuple::vector(0.0, 1.0, 0.0);

        let r = v.reflect(n);

        assert_eq!(r, Tuple::vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = Tuple::vector(0.0, -1.0, 0.0);
        let n = Tuple::vector(sqrt_n_over_n(2), sqrt_n_over_n(2), 0.0);

        let r = v.reflect(n);

        assert_eq!(r, Tuple::vector(1.0, 0.0, 0.0));
    }
}
