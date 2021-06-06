#![allow(dead_code)]

use bevy::math::Vec4;

#[derive(Debug, PartialEq)]
pub struct Tuple(Vec4);

impl Tuple {
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple(Vec4::new(x, y, z, w))
    }

    fn point(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, y, z, 1.0)
    }

    fn vector(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, y, z, 0.0)
    }

    fn is_point(&self) -> bool {
        self.0.w > 0.0
    }

    fn is_vector(&self) -> bool {
        self.0.w == 0.0
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use super::*;

    #[test]
    fn tuple_with_w_1_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);

        assert!(approx_eq!(f32, a.0.x, 4.3));
        assert!(approx_eq!(f32, a.0.y, -4.2));
        assert!(approx_eq!(f32, a.0.z, 3.1));
        assert!(approx_eq!(f32, a.0.w, 1.0));
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_with_w_0_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);

        assert!(approx_eq!(f32, a.0.x, 4.3));
        assert!(approx_eq!(f32, a.0.y, -4.2));
        assert!(approx_eq!(f32, a.0.z, 3.1));
        assert!(approx_eq!(f32, a.0.w, 0.0));
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
}
