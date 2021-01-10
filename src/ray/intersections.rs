use std::{iter::FromIterator, ops::Index};

use crate::{
    shape::{Object, Shape},
    tuple::{dot, Tuple},
    MARGIN,
};

use super::Ray;

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Object,
}

impl Intersection {
    pub fn new(t: f64, object: Object) -> Self {
        Self { t, object }
    }

    pub fn prepare_computations(&self, ray: Ray) -> Result<Computations, String> {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        let mut normalv = self.object.normal_at(point.x, point.y, point.z)?;
        let inside = dot(normalv, eyev) < 0.0;
        if inside {
            normalv = -normalv;
        }
        Ok(Computations {
            t: self.t,
            object: self.object.clone(),
            point,
            over_point: point + normalv * MARGIN.epsilon,
            eyev,
            normalv,
            inside,
        })
    }
}

#[derive(Debug, Default)]
pub struct Intersections {
    pub data: Vec<Intersection>,
}

impl Intersections {
    pub fn new(intersections: Vec<Intersection>) -> Self {
        Self {
            data: intersections,
        }
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.data
            .iter()
            .filter(|i| i.t >= 0.0)
            .min_by(|&i1, &i2| i1.t.partial_cmp(&i2.t).unwrap())
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn append(&mut self, other: &mut Intersections) {
        self.data.append(&mut other.data);
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl FromIterator<Intersection> for Intersections {
    fn from_iter<T: IntoIterator<Item = Intersection>>(iter: T) -> Self {
        let mut result = Self { data: vec![] };
        for i in iter {
            result.data.push(i);
        }
        result
    }
}

pub struct Computations {
    t: f64,
    pub object: Object,
    pub point: Tuple,
    pub over_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    inside: bool,
}

#[cfg(test)]
mod tests {
    use float_cmp::ApproxEq;
    use light::PointLight;

    use crate::{
        color::Color, light, matrix::transform::Transform, ray::Ray, shape::sphere::Sphere,
        tuple::Tuple, world::World, MARGIN,
    };

    use super::*;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Object::Sphere(Sphere::default());

        let i = Intersection::new(3.5, s.clone());

        f_assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Object::Sphere(Sphere::default());
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s);

        let xs = Intersections::new(vec![i1, i2]);

        assert_eq!(xs.len(), 2);
        f_assert_eq!(xs[0].t, 1.0);
        f_assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Object::Sphere(Sphere::default());
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![i2, i1.clone()]);

        let i = xs.hit();

        assert_eq!(i, Some(&i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Object::Sphere(Sphere::default());
        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(1.0, s);
        let xs = Intersections::new(vec![i2.clone(), i1]);

        let i = xs.hit();

        assert_eq!(i, Some(&i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Object::Sphere(Sphere::default());
        let i1 = Intersection::new(-2.0, s.clone());
        let i2 = Intersection::new(-1.0, s);
        let xs = Intersections::new(vec![i2, i1]);

        let i = xs.hit();

        assert_eq!(i, None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Object::Sphere(Sphere::default());
        let i1 = Intersection::new(5.0, s.clone());
        let i2 = Intersection::new(7.0, s.clone());
        let i3 = Intersection::new(-3.0, s.clone());
        let i4 = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![i1, i2, i3, i4.clone()]);

        let i = xs.hit();

        assert_eq!(i, Some(&i4));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Object::Sphere(Sphere::default());
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(r).unwrap();

        f_assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        f_assert_eq!(comps.point, &Tuple::point(0.0, 0.0, -1.0));
        f_assert_eq!(comps.eyev, &Tuple::vector(0.0, 0.0, -1.0));
        assert!(!comps.inside);
        f_assert_eq!(comps.normalv, &Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Object::Sphere(Sphere::default());
        let i = Intersection::new(1.0, shape);

        let comps = i.prepare_computations(r).unwrap();

        f_assert_eq!(comps.point, &Tuple::point(0.0, 0.0, 1.0));
        f_assert_eq!(comps.eyev, &Tuple::vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
        f_assert_eq!(comps.normalv, &Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, w.objects[0].clone());

        let comps = i.prepare_computations(r).unwrap();
        let c = w.shade_hit(comps);

        f_assert_eq!(c, &Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let w = World {
            light_sources: vec![PointLight::new(
                Tuple::point(0.0, 0.25, 0.0),
                Color::new(1.0, 1.0, 1.0),
            )],
            ..Default::default()
        };
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.objects[1].clone();
        let i = Intersection::new(0.5, shape);

        let comps = i.prepare_computations(r).unwrap();
        let c = w.shade_hit(comps);

        f_assert_eq!(c, &Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::default()
            .origin(0.0, 0.0, -5.0)
            .direction(0.0, 0.0, 1.0);
        let shape =
            Object::Sphere(Sphere::default().transform(Transform::translation(0.0, 0.0, 1.0)));
        let i = Intersection::new(5.0, shape);

        let comps = i.prepare_computations(r).unwrap();

        assert!(comps.over_point.z < -MARGIN.epsilon / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}
