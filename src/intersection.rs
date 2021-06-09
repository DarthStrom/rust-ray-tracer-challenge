use std::ops::Index;

use crate::sphere::Sphere;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Intersection {
    pub t: f32,
    pub object: Sphere,
}

impl Intersection {
    pub fn new(t: f32, object: Sphere) -> Self {
        Self { t, object }
    }
}

pub struct Intersections(Vec<Intersection>);

impl Intersections {
    pub fn new(vec: Vec<Intersection>) -> Self {
        Self(vec)
    }

    pub fn hit(&self) -> Option<&Intersection> {
        self.0
            .iter()
            .filter(|i| i.t >= 0.0)
            .min_by(|i1, i2| i1.t.partial_cmp(&i2.t).unwrap())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use float_cmp::approx_eq;

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let s = Sphere::default();

        let i = Intersection::new(3.5, s);

        assert!(approx_eq!(f32, i.t, 3.5));
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);

        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
        assert!(approx_eq!(f32, xs[0].t, 1.0));
        assert!(approx_eq!(f32, xs[1].t, 2.0));
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![i2, i1]);

        let i = xs.hit();

        assert_eq!(i, Some(&i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
        let xs = Intersections::new(vec![i2, i1]);

        let i = xs.hit();

        assert_eq!(i, Some(&i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);
        let xs = Intersections::new(vec![i2, i1]);

        let i = xs.hit();

        assert_eq!(i, None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let xs = Intersections::new(vec![i1, i2, i3, i4]);

        let i = xs.hit();

        assert_eq!(i, Some(&i4));
    }

    // #[test]
    // fn precomputing_the_state_of_an_intersection() {
    //     let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    //     let shape = Object::Sphere(Sphere::default());
    //     let i = Intersection::new(4.0, shape);

    //     let comps = i.prepare_computations(r).unwrap();

    //     f_assert_eq!(comps.t, i.t);
    //     assert_eq!(comps.object, i.object);
    //     f_assert_eq!(comps.point, &Tuple::point(0.0, 0.0, -1.0));
    //     f_assert_eq!(comps.eyev, &Tuple::vector(0.0, 0.0, -1.0));
    //     assert!(!comps.inside);
    //     f_assert_eq!(comps.normalv, &Tuple::vector(0.0, 0.0, -1.0));
    // }

    // #[test]
    // fn the_hit_when_an_intersection_occurs_on_the_inside() {
    //     let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    //     let shape = Object::Sphere(Sphere::default());
    //     let i = Intersection::new(1.0, shape);

    //     let comps = i.prepare_computations(r).unwrap();

    //     f_assert_eq!(comps.point, &Tuple::point(0.0, 0.0, 1.0));
    //     f_assert_eq!(comps.eyev, &Tuple::vector(0.0, 0.0, -1.0));
    //     assert!(comps.inside);
    //     f_assert_eq!(comps.normalv, &Tuple::vector(0.0, 0.0, -1.0));
    // }

    // #[test]
    // fn shading_an_intersection() {
    //     let w = World::default();
    //     let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    //     let i = Intersection::new(4.0, w.objects[0].clone());

    //     let comps = i.prepare_computations(r).unwrap();
    //     let c = w.shade_hit(comps);

    //     f_assert_eq!(c, &Color::new(0.38066, 0.47583, 0.2855));
    // }

    // #[test]
    // fn shading_an_intersection_from_the_inside() {
    //     let w = World {
    //         light_sources: vec![PointLight::new(
    //             Tuple::point(0.0, 0.25, 0.0),
    //             Color::new(1.0, 1.0, 1.0),
    //         )],
    //         ..Default::default()
    //     };
    //     let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
    //     let shape = w.objects[1].clone();
    //     let i = Intersection::new(0.5, shape);

    //     let comps = i.prepare_computations(r).unwrap();
    //     let c = w.shade_hit(comps);

    //     f_assert_eq!(c, &Color::new(0.90498, 0.90498, 0.90498));
    // }

    // #[test]
    // fn the_hit_should_offset_the_point() {
    //     let r = Ray::default()
    //         .origin(0.0, 0.0, -5.0)
    //         .direction(0.0, 0.0, 1.0);
    //     let shape =
    //         Object::Sphere(Sphere::default().transform(Transform::translation(0.0, 0.0, 1.0)));
    //     let i = Intersection::new(5.0, shape);

    //     let comps = i.prepare_computations(r).unwrap();

    //     assert!(comps.over_point.z < -MARGIN.epsilon / 2.0);
    //     assert!(comps.point.z > comps.over_point.z);
    // }

    // #[test]
    // fn precomputing_the_reflection_vector() {
    //     let shape = Object::Plane(Plane::default());
    //     let r = Ray::default()
    //         .origin(0.0, 1.0, -1.0)
    //         .direction(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0);
    //     let i = Intersection::new(SQRT_2, shape);

    //     let comps = i.prepare_computations(r).unwrap();

    //     f_assert_eq!(
    //         comps.reflectv,
    //         &Tuple::vector(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0)
    //     );
    // }
}
