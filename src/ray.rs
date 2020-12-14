use num_traits::{Float, FromPrimitive};
use std::ops::{Add, Mul, Sub};

use crate::tuple::{dot, Tuple};

#[derive(Debug, PartialEq)]
pub struct Ray<F> {
    origin: Tuple<F>,
    direction: Tuple<F>,
}

impl<F: Float + FromPrimitive> Ray<F> {
    pub fn new(origin: Tuple<F>, direction: Tuple<F>) -> Self {
        Self { origin, direction }
    }

    pub fn position(&self, t: F) -> Tuple<F> {
        self.origin + self.direction * t
    }
}

pub struct Sphere<F> {
    center: Tuple<F>,
    radius: F,
}

impl<
        F: Float + FromPrimitive + Add<Output = F> + Sub<Output = F> + Copy + Mul<Output = F> + Mul,
    > Sphere<F>
{
    pub fn intersect(&self, ray: &Ray<F>) -> Vec<F> {
        let sphere_to_ray = ray.origin - self.center;

        let zero = F::from_f64(0.0).unwrap();
        let one = F::from_f64(1.0).unwrap();
        let two = F::from_f64(2.0).unwrap();
        let four = F::from_f64(4.0).unwrap();
        let a = dot(&ray.direction, &ray.direction);
        let b = two * dot(&ray.direction, &sphere_to_ray);
        let c = dot(&sphere_to_ray, &sphere_to_ray) - one;

        let discriminant = b.powf(two) - four * a * c;

        if discriminant < zero {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (two * a);
        let t2 = (-b + discriminant.sqrt()) / (two * a);

        vec![t1, t2]
    }
}

impl Default for Sphere<f64> {
    fn default() -> Self {
        Self {
            center: Tuple::point(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;

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
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originating_inside_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(&r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }
}
