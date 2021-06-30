use std::{iter::FromIterator, ops::Index};

use crate::{ray::Ray, shapes::Shape, tuple::Tuple, MARGIN};

#[derive(Copy, Clone, Debug)]
pub struct Intersection<'a> {
    pub t: f32,
    object: &'a dyn Shape,
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
            && self.object.material() == other.object.material()
            && self.object.transform() == other.object.transform()
    }
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &'a dyn Shape) -> Self {
        Self { t, object }
    }

    pub fn prepare_computations(&self, ray: Ray, intersections: Intersections) -> Computations {
        let point = ray.position(self.t);
        let eyev = -ray.direction;
        let mut normalv = self.object.normal_at(point.x(), point.y(), point.z());
        let reflectv = ray.direction.reflect(normalv);

        let inside = normalv.dot(eyev) < 0.0;
        if inside {
            normalv = -normalv;
        }

        let mut n1 = 0.0;
        let mut n2 = 0.0;
        let mut containers: Vec<&dyn Shape> = vec![];
        for intersection in intersections {
            if &intersection == self {
                if containers.is_empty() {
                    n1 = 1.0
                } else {
                    n1 = containers.last().unwrap().material().refractive_index
                }
            }

            if containers.iter().any(|s| s == &intersection.object) {
                containers = containers
                    .into_iter()
                    .filter(|&s| s != intersection.object)
                    .collect();
            } else {
                containers.push(intersection.object);
            }

            if &intersection == self {
                if containers.is_empty() {
                    n2 = 1.0
                } else {
                    n2 = containers.last().unwrap().material().refractive_index
                }
                break;
            }
        }

        Computations {
            t: self.t,
            object: self.object,
            point,
            over_point: point + normalv * MARGIN.epsilon,
            under_point: point - normalv * MARGIN.epsilon,
            eyev,
            normalv,
            reflectv,
            inside,
            n1,
            n2,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Intersections<'a>(Vec<Intersection<'a>>);

impl<'a> Intersections<'a> {
    pub fn new(vec: Vec<Intersection<'a>>) -> Self {
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

    pub fn vec(self) -> Vec<Intersection<'a>> {
        self.0
    }

    pub fn push(&mut self, intersection: Intersection<'a>) {
        self.0.push(intersection)
    }
}

impl<'a> Index<usize> for Intersections<'a> {
    type Output = Intersection<'a>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<'a> FromIterator<Intersection<'a>> for Intersections<'a> {
    fn from_iter<T: IntoIterator<Item = Intersection<'a>>>(iter: T) -> Self {
        let mut result = Self(vec![]);
        for i in iter {
            result.0.push(i);
        }
        result
    }
}

impl<'a> IntoIterator for Intersections<'a> {
    type Item = Intersection<'a>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Computations<'a> {
    t: f32,
    pub object: &'a dyn Shape,
    pub point: Tuple,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub reflectv: Tuple,
    pub n1: f32,
    pub n2: f32,
    inside: bool,
}

impl<'a> Computations<'a> {
    pub fn schlick(&self) -> f32 {
        let mut cos = self.eyev.dot(self.normalv);

        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }

            let cos_t = (1.0 - sin2_t).sqrt();

            cos = cos_t
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::SQRT_2;

    use crate::{
        materials::Material,
        shapes::ShapeBuilder,
        shapes::{plane::Plane, sphere::Sphere},
        test::sqrt_n_over_n,
        transformations::Transform,
    };

    use super::*;

    use float_cmp::approx_eq;

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
        assert!(approx_eq!(f32, xs[0].t, 1.0));
        assert!(approx_eq!(f32, xs[1].t, 2.0));
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i2, i1]);

        let i = xs.hit();

        assert!(approx_eq!(f32, i.unwrap().t, i1.t));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = Intersections::new(vec![i2, i1]);

        let i = xs.hit();

        assert!(approx_eq!(f32, i.unwrap().t, i2.t));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = Intersections::new(vec![i2, i1]);

        let i = xs.hit();

        assert!(i.is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = Intersections::new(vec![i1, i2, i3, i4]);

        let i = xs.hit();

        assert!(approx_eq!(f32, i.unwrap().t, i4.t));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, &shape);

        let comps = i.prepare_computations(r, Intersections::new(vec![i]));

        assert!(approx_eq!(f32, comps.t, i.t));
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert!(!comps.inside);
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(1.0, &shape);

        let comps = i.prepare_computations(r, Intersections::new(vec![i]));

        assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

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

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::default()
            .origin(0.0, 0.0, -5.0)
            .direction(0.0, 0.0, 1.0);
        let shape = Sphere::default().with_transform(Transform::translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &shape);

        let comps = i.prepare_computations(r, Intersections::new(vec![i]));

        assert!(comps.over_point.z() < -MARGIN.epsilon / 2.0);
        assert!(comps.point.z() > comps.over_point.z());
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = Plane::default();
        let r = Ray::default()
            .origin(0.0, 1.0, -1.0)
            .direction(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0);
        let i = Intersection::new(SQRT_2, &shape);

        let comps = i.prepare_computations(r, Intersections::new(vec![i]));

        assert_eq!(
            comps.reflectv,
            Tuple::vector(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0)
        );
    }

    macro_rules! find_n1_and_n2 {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (index, n1, n2) = $value;
                let a = Sphere::glass().with_transform(Transform::scaling(2., 2., 2.)).with_material(Material::default().refractive_index(1.5));
                let b = Sphere::glass().with_transform(Transform::translation(0., 0., -0.25)).with_material(Material::default().refractive_index(2.0));
                let c = Sphere::glass().with_transform(Transform::translation(0., 0., 0.25)).with_material(Material::default().refractive_index(2.5));
                let r = Ray::default().origin(0., 0., -4.).direction(0., 0., 1.0);
                let xs = Intersections::new(vec![
                    Intersection::new(2.0, &a),
                    Intersection::new(2.75, &b),
                    Intersection::new(3.25, &c),
                    Intersection::new(4.75, &b),
                    Intersection::new(5.25, &c),
                    Intersection::new(6.0, &a),
                    ]);

                let xs_copy = xs.clone();
                let comps = xs_copy[index].prepare_computations(r, xs);

                assert!(approx_eq!(f32, comps.n1, n1));
                assert!(approx_eq!(f32, comps.n2, n2));
            }
        )*
        }
    }

    find_n1_and_n2! {
        find_ns_0: (0, 1.0, 1.5),
        find_ns_1: (1, 1.5, 2.0),
        find_ns_2: (2, 2.0, 2.5),
        find_ns_3: (3, 2.5, 2.5),
        find_ns_4: (4, 2.5, 1.5),
        find_ns_5: (5, 1.5, 1.0),
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray::default()
            .origin(0.0, 0.0, -5.0)
            .direction(0.0, 0.0, 1.0);
        let shape = Sphere::glass().with_transform(Transform::translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &shape);
        let xs = Intersections::new(vec![i]);

        let comps = i.prepare_computations(r, xs);

        assert!(comps.under_point.z() > MARGIN.epsilon / 2.0);
        assert!(comps.point.z() < comps.under_point.z());
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let shape = Sphere::glass();
        let r = Ray::default()
            .origin(0.0, 0.0, sqrt_n_over_n(2))
            .direction(0.0, 1.0, 0.0);
        let xs = Intersections::new(vec![
            Intersection::new(-sqrt_n_over_n(2), &shape),
            Intersection::new(sqrt_n_over_n(2), &shape),
        ]);
        let xs_copy = xs.clone();

        let comps = xs[1].prepare_computations(r, xs_copy);
        let reflectance = comps.schlick();

        assert!(approx_eq!(f32, reflectance, 1.0));
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = Sphere::glass();
        let r = Ray::default()
            .origin(0.0, 0.0, 0.0)
            .direction(0.0, 1.0, 0.0);
        let xs = Intersections::new(vec![
            Intersection::new(-1.0, &shape),
            Intersection::new(1.0, &shape),
        ]);
        let xs_copy = xs.clone();

        let comps = xs[1].prepare_computations(r, xs_copy);
        let reflectance = comps.schlick();

        assert!(approx_eq!(f32, reflectance, 0.04));
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_gt_n1() {
        let shape = Sphere::glass();
        let r = Ray::default()
            .origin(0.0, 0.99, -2.0)
            .direction(0.0, 0.0, 1.0);
        let xs = Intersections::new(vec![Intersection::new(1.8589, &shape)]);
        let xs_copy = xs.clone();

        let comps = xs[0].prepare_computations(r, xs_copy);
        let reflectance = comps.schlick();

        assert!(approx_eq!(f32, reflectance, 0.4887307));
    }
}
