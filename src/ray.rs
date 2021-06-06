use bevy::math::Mat4;

use crate::{transform::Transform, tuple::Tuple};

#[derive(Copy, Clone, Debug, Default, PartialEq)]

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Self {
        Self { origin, direction }
    }

    pub fn origin(self, x: f32, y: f32, z: f32) -> Self {
        Self {
            origin: Tuple::point(x, y, z),
            ..self
        }
    }

    pub fn direction(self, x: f32, y: f32, z: f32) -> Self {
        Self {
            direction: Tuple::vector(x, y, z),
            ..self
        }
    }

    pub fn position(&self, t: f32) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn transform(self, transform: Transform) -> Self {
        let origin_vec = transform.mat() * self.origin.vec();
        let direction_vec = transform.mat() * self.direction.vec();
        Self {
            origin: Tuple::new(origin_vec.x, origin_vec.y, origin_vec.z, origin_vec.w),
            direction: Tuple::new(
                direction_vec.x,
                direction_vec.y,
                direction_vec.z,
                direction_vec.w,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Tuple::point(1.0, 2.0, 3.0);
        let direction = Tuple::vector(4.0, 5.0, 6.0);

        let r = Ray::new(origin, direction);

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(Tuple::point(2.0, 3.0, 4.0), Tuple::vector(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Tuple::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Tuple::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Tuple::point(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = Transform::translation(3.0, 4.0, 5.0);

        let r2 = r.transform(m);

        assert_eq!(r2.origin, Tuple::point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(Tuple::point(1.0, 2.0, 3.0), Tuple::vector(0.0, 1.0, 0.0));
        let m = Transform::scaling(2.0, 3.0, 4.0);

        let r2 = r.transform(m);

        assert_eq!(r2.origin, Tuple::point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Tuple::vector(0.0, 3.0, 0.0));
    }

    // #[test]
    // fn intersecting_a_scaled_sphere_with_a_ray() {
    //     let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    //     let mut s = Sphere::default();

    //     s = s.transform(Transform::scaling(2.0, 2.0, 2.0));
    //     let xs = s.intersect(r);

    //     assert_eq!(xs.len(), 2);
    //     f_assert_eq!(xs[0].t, 3.0);
    //     f_assert_eq!(xs[1].t, 7.0)
    // }

    // #[test]
    // fn intersecting_a_translated_sphere_with_a_ray() {
    //     let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    //     let mut s = Sphere::default();

    //     s = s.transform(Transform::translation(5.0, 0.0, 0.0));
    //     let xs = s.intersect(r);

    //     assert_eq!(xs.len(), 0);
    // }
}
