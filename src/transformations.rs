use std::ops::Mul;

use bevy::math::{Mat4, Vec3, Vec4};

use crate::{float_eq, tuple::Tuple};

#[derive(Clone, Copy, Debug, Default)]
pub struct Transform(Mat4);

pub const IDENTITY: Transform = Transform(Mat4::IDENTITY);

impl Transform {
    pub fn inverse(&self) -> Self {
        Self(self.0.inverse())
    }

    pub fn rotation_x(radians: f32) -> Self {
        Self(Mat4::from_rotation_x(radians))
    }

    pub fn rotation_y(radians: f32) -> Self {
        Self(Mat4::from_rotation_y(radians))
    }

    pub fn rotation_z(radians: f32) -> Self {
        Self(Mat4::from_rotation_z(radians))
    }

    pub fn scaling(x: f32, y: f32, z: f32) -> Self {
        Self(Mat4::from_scale(Vec3::new(x, y, z)))
    }

    pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        Self(Mat4::from_cols(
            Vec4::new(1.0, yx, zx, 0.0),
            Vec4::new(xy, 1.0, zy, 0.0),
            Vec4::new(xz, yz, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        ))
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        Self(Mat4::from_translation(Vec3::new(x, y, z)))
    }

    pub fn mat(&self) -> Mat4 {
        self.0
    }

    pub fn transpose(&self) -> Self {
        Self(self.0.transpose())
    }

    pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Self {
        let forward = (to - from).normalize();
        let left = forward.cross(up.normalize());
        let true_up = left.cross(forward);
        let orientation = Mat4::from_cols(
            Vec4::new(left.x(), true_up.x(), -forward.x(), 0.0),
            Vec4::new(left.y(), true_up.y(), -forward.y(), 0.0),
            Vec4::new(left.z(), true_up.z(), -forward.z(), 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );
        Transform(orientation) * Transform::translation(-from.x(), -from.y(), -from.z())
    }
}

impl Mul for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl Mul<Tuple> for Transform {
    type Output = Tuple;

    fn mul(self, rhs: Tuple) -> Self::Output {
        let vec = self.0 * rhs.vec();
        Tuple::new(vec.x, vec.y, vec.z, rhs.vec().w)
    }
}

impl PartialEq for Transform {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if !float_eq(self.0.row(i)[j], other.0.row(i)[j]) {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

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
        let inv = transform.inverse();
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
        let inv = transform.inverse();
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

        let sqrt2over2 = 2_f32.sqrt() / 2.0;
        assert_eq!(half_quarter * p, Tuple::point(0.0, sqrt2over2, sqrt2over2));
        assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = Transform::rotation_y(PI / 4.0);
        let full_quarter = Transform::rotation_y(PI / 2.0);

        let sqrt2over2 = 2_f32.sqrt() / 2.0;
        assert_eq!(half_quarter * p, Tuple::point(sqrt2over2, 0.0, sqrt2over2));
        assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Transform::rotation_z(PI / 4.0);
        let full_quarter = Transform::rotation_z(PI / 2.0);

        let sqrt2over2 = 2_f32.sqrt() / 2.0;
        assert_eq!(half_quarter * p, Tuple::point(-sqrt2over2, sqrt2over2, 0.0));
        assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
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
        assert_eq!(p2, Tuple::point(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert_eq!(p3, Tuple::point(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert_eq!(p4, Tuple::point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let point = Tuple::point(1.0, 0.0, 1.0);
        let a = Transform::rotation_x(PI / 2.0);
        let b = Transform::scaling(5.0, 5.0, 5.0);
        let c = Transform::translation(10.0, 5.0, 7.0);

        let transform = c * b * a;

        assert_eq!(transform * point, Tuple::point(15.0, 0.0, 7.0));
    }

    // TODO
    // #[test]
    // fn fluent_api() {
    //     let rx = Transform::rotation_x(PI / 2.0);
    //     let ry = Transform::rotation_y(PI / 3.0);
    //     let rz = Transform::rotation_z(PI / 4.0);
    //     let s = Transform::scaling(5.0, 5.0, 5.0);
    //     let sh = Transform::shearing(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    //     let t = Transform::translation(10.0, 5.0, 7.0);

    //     let fluent_things = IDENTITY
    //         .rotate_x(PI / 2.0)
    //         .rotate_y(PI / 3.0)
    //         .rotate_z(PI / 4.0)
    //         .scale(5.0, 5.0, 5.0)
    //         .shear(1.0, 2.0, 3.0, 4.0, 5.0, 6.0)
    //         .translate(10.0, 5.0, 7.0);
    //     let individual_things = t * sh * s * rz * ry * rx;

    //     assert_eq!(fluent_things, individual_things);
    // }

    #[test]
    fn transformation_matrix_for_the_default_orientation() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, -1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = Transform::view_transform(from, to, up);

        assert_eq!(t, IDENTITY);
    }

    #[test]
    fn view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, 1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = Transform::view_transform(from, to, up);

        assert_eq!(t, Transform::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn view_transformation_moves_the_world() {
        let from = Tuple::point(0.0, 0.0, 8.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);

        let t = Transform::view_transform(from, to, up);

        assert_eq!(t, Transform::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = Tuple::point(1.0, 3.0, 2.0);
        let to = Tuple::point(4.0, -2.0, 8.0);
        let up = Tuple::vector(1.0, 1.0, 0.0);

        let t = Transform::view_transform(from, to, up);

        assert_eq!(
            t,
            Transform(Mat4::from_cols(
                Vec4::new(-0.50709, 0.76772, -0.35857, 0.0),
                Vec4::new(0.50709, 0.60609, 0.59761, 0.0),
                Vec4::new(0.67612, 0.12122, -0.71714, 0.0),
                Vec4::new(-2.36643, -2.82843, 0.0, 1.0)
            ))
        )
    }
}
