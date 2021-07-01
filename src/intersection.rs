use std::cmp::Ordering;

use crate::{float_cmp, ray::Ray, shapes::Shape, tuple::Tuple, EPSILON};

#[derive(Copy, Clone, Debug)]
pub struct Intersection<'a> {
    pub t: f32,
    object: &'a dyn Shape,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f32, object: &'a dyn Shape) -> Self {
        Self { t, object }
    }

    pub fn prepare_computations(&self, ray: Ray, intersections: &[Intersection]) -> Computations {
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
            if intersection == self {
                if containers.is_empty() {
                    n1 = 1.0
                } else {
                    n1 = containers.last().unwrap().material().refractive_index
                }
            }

            if containers.contains(&intersection.object) {
                containers = containers
                    .into_iter()
                    .filter(|&s| s != intersection.object)
                    .collect();
            } else {
                containers.push(intersection.object);
            }

            if intersection == self {
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
            over_point: point + normalv * EPSILON,
            under_point: point - normalv * EPSILON,
            eyev,
            normalv,
            reflectv,
            inside,
            n1,
            n2,
        }
    }
}

impl Intersection<'_> {
    pub fn hit<'a>(xs: &'a [Intersection]) -> Option<&'a Intersection<'a>> {
        xs.iter().filter(|x| x.t >= 0.0).min()
    }
}

impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Intersection) -> bool {
        self.t == other.t && self.object.shape_eq(other.object)
    }
}

impl PartialOrd for Intersection<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(float_cmp(self.t, other.t))
    }
}

impl Eq for Intersection<'_> {}

impl Ord for Intersection<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        float_cmp(self.t, other.t)
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
        float_eq,
        materials::Material,
        shapes::ShapeBuilder,
        shapes::{plane::Plane, sphere::Sphere},
        test::sqrt_n_over_n,
        transformations::Transform,
    };

    use super::*;

    #[test]
    fn aggregating_intersections() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);

        let xs = vec![i1, i2];

        assert_eq!(xs.len(), 2);
        assert!(float_eq(xs[0].t, 1.0));
        assert!(float_eq(xs[1].t, 2.0));
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i2, i1];

        let i = Intersection::hit(&xs);

        assert!(float_eq(i.unwrap().t, i1.t));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i2, i1];

        let i = Intersection::hit(&xs);

        assert!(float_eq(i.unwrap().t, i2.t));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i2, i1];

        let i = Intersection::hit(&xs);

        assert!(i.is_none());
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Sphere::default();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2, i3, i4];

        let i = Intersection::hit(&xs);

        assert!(float_eq(i.unwrap().t, i4.t));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Sphere::default();
        let i = Intersection::new(4.0, &shape);

        let comps = i.prepare_computations(r, &[i]);

        assert!(float_eq(comps.t, i.t));
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

        let comps = i.prepare_computations(r, &[i]);

        assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::default()
            .origin(0.0, 0.0, -5.0)
            .direction(0.0, 0.0, 1.0);
        let shape = Sphere::default().with_transform(Transform::translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &shape);

        let comps = i.prepare_computations(r, &[i]);

        assert!(comps.over_point.z() < -EPSILON / 2.0);
        assert!(comps.point.z() > comps.over_point.z());
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = Plane::default();
        let r = Ray::default()
            .origin(0.0, 1.0, -1.0)
            .direction(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0);
        let i = Intersection::new(SQRT_2, &shape);

        let comps = i.prepare_computations(r, &[i]);

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
                let a = Sphere::glass().with_transform(Transform::scaling(2.0, 2.0, 2.0)).with_material(Material::default().refractive_index(1.5));
                let b = Sphere::glass().with_transform(Transform::translation(0.0, 0.0, -0.25)).with_material(Material::default().refractive_index(2.0));
                let c = Sphere::glass().with_transform(Transform::translation(0.0, 0.0, 0.25)).with_material(Material::default().refractive_index(2.5));
                let r = Ray::default().origin(0.0, 0.0, -4.0).direction(0.0, 0.0, 1.0);
                let xs = &[
                    Intersection::new(2.0, &a),
                    Intersection::new(2.75, &b),
                    Intersection::new(3.25, &c),
                    Intersection::new(4.75, &b),
                    Intersection::new(5.35, &c),
                    Intersection::new(6.0, &a),
                    ];

                let comps = xs[index].prepare_computations(r, xs);

                assert!(float_eq(comps.n1, n1));
                assert!(float_eq(comps.n2, n2));
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

        let comps = i.prepare_computations(r, &[i]);

        assert!(comps.under_point.z() > EPSILON / 2.0);
        assert!(comps.point.z() < comps.under_point.z());
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let shape = Sphere::glass();
        let r = Ray::default()
            .origin(0.0, 0.0, sqrt_n_over_n(2))
            .direction(0.0, 1.0, 0.0);
        let xs = vec![
            Intersection::new(-sqrt_n_over_n(2), &shape),
            Intersection::new(sqrt_n_over_n(2), &shape),
        ];

        let comps = xs[1].prepare_computations(r, &xs);
        let reflectance = comps.schlick();

        assert!(float_eq(reflectance, 1.0));
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = Sphere::glass();
        let r = Ray::default()
            .origin(0.0, 0.0, 0.0)
            .direction(0.0, 1.0, 0.0);
        let xs = vec![
            Intersection::new(-1.0, &shape),
            Intersection::new(1.0, &shape),
        ];

        let comps = xs[1].prepare_computations(r, &xs);
        let reflectance = comps.schlick();

        assert!(float_eq(reflectance, 0.04));
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_gt_n1() {
        let shape = Sphere::glass();
        let r = Ray::default()
            .origin(0.0, 0.99, -2.0)
            .direction(0.0, 0.0, 1.0);
        let xs = vec![Intersection::new(1.8589, &shape)];

        let comps = xs[0].prepare_computations(r, &xs);
        let reflectance = comps.schlick();

        assert!(float_eq(reflectance, 0.4887307));
    }
}
