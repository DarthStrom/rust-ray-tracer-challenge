use crate::{
    matrix::transform::Transform,
    ray::{Intersection, Intersections, Ray},
    tuple::{dot, Tuple},
};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    center: Tuple,
    radius: f64,
    transform: Transform,
}

impl Sphere {
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let ray = if self.transform.is_invertible() {
            ray.transform(self.transform.inverse().unwrap())
        } else {
            ray.clone()
        };

        let sphere_to_ray = ray.origin - self.center;

        let a = dot(&ray.direction, &ray.direction);
        let b = 2.0 * dot(&ray.direction, &sphere_to_ray);
        let c = dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant < 0.0 {
            return Intersections::default();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        Intersections::new(vec![
            Intersection::new(t1, self),
            Intersection::new(t2, self),
        ])
    }

    pub fn set_transform(&mut self, transform: &Transform) {
        self.transform = transform.clone();
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Tuple::point(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Transform::identity(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_transformation() {
        let s = Sphere::default();

        assert_eq!(s.transform, Transform::identity())
    }

    #[test]
    fn changing_sphere_transformation() {
        let mut s = Sphere::default();
        let t = Transform::translation(2.0, 3.0, 4.0);

        s.set_transform(&t);

        assert_eq!(s.transform, t);
    }
}
