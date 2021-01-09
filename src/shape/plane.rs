use crate::{
    matrix::transform::Transform,
    ray::{
        intersections::{Intersection, Intersections},
        Ray,
    },
    tuple::Tuple,
    MARGIN,
};

use super::{Object, Shape};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Plane {
    pub transform: Transform,
}

impl Plane {
    fn transform(self, transform: Transform) -> Self {
        Self { transform, ..self }
    }
}

impl Shape for Plane {
    fn intersect(&self, ray: Ray) -> Intersections {
        if ray.direction.y.abs() < MARGIN.epsilon {
            Intersections::new(vec![])
        } else {
            let t = -ray.origin.y / ray.direction.y;
            Intersections::new(vec![Intersection::new(t, Object::Plane(self.clone()))])
        }
    }

    fn normal_at(&self, x: f64, y: f64, z: f64) -> Result<Tuple, String> {
        Ok(Tuple::vector(0.0, 1.0, 0.0))
    }

    fn transform(&self) -> Transform {
        self.transform.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{ray::Ray, shape::Object, tuple::Tuple};
    use float_cmp::ApproxEq;

    #[test]
    fn normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::default();
        let expected = Tuple::vector(0.0, 1.0, 0.0);

        let n1 = p.normal_at(0.0, 0.0, 0.0).unwrap();
        let n2 = p.normal_at(10.0, 0.0, -10.0).unwrap();
        let n3 = p.normal_at(-5.0, 0.0, 150.0).unwrap();

        f_assert_eq!(n1, &expected);
        f_assert_eq!(n2, &expected);
        f_assert_eq!(n3, &expected);
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::default();
        let r = Ray::default()
            .origin(0.0, 10.0, 0.0)
            .direction(0.0, 0.0, 1.0);

        let xs = p.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::default();
        let r = Ray::default()
            .origin(0.0, 0.0, 0.0)
            .direction(0.0, 0.0, 1.0);

        let xs = p.intersect(r);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::default();
        let r = Ray::default()
            .origin(0.0, 1.0, 0.0)
            .direction(0.0, -1.0, 0.0);

        let xs = p.intersect(r);

        assert_eq!(xs.len(), 1);
        f_assert_eq!(xs[0].t, 1.0);
        if let Object::Plane(object) = &xs[0].object {
            assert_eq!(object, &p);
        } else {
            panic!("not a plane");
        }
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::default();
        let r = Ray::default()
            .origin(0.0, -1.0, 0.0)
            .direction(0.0, 1.0, 0.0);

        let xs = p.intersect(r);

        assert_eq!(xs.len(), 1);
        f_assert_eq!(xs[0].t, 1.0);
        if let Object::Plane(object) = &xs[0].object {
            assert_eq!(object, &p);
        } else {
            panic!("not a plane");
        }
    }
}
