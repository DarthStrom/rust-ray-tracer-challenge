use crate::{
    material::Material,
    matrix::transform::Transform,
    ray::{Intersection, Intersections, Ray},
    tuple::{dot, Tuple},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    pub center: Tuple,
    pub radius: f64,
    pub transform: Transform,
    pub material: Material,
}

impl Sphere {
    pub fn intersect(&self, ray: Ray) -> Intersections {
        let ray = if self.transform.is_invertible() {
            ray.transform(self.transform.inverse().unwrap())
        } else {
            ray
        };

        let sphere_to_ray = ray.origin - self.center;

        let a = dot(ray.direction, ray.direction);
        let b = 2.0 * dot(ray.direction, sphere_to_ray);
        let c = dot(sphere_to_ray, sphere_to_ray) - 1.0;

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

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    pub fn normal_at(&self, world_point: Tuple) -> Result<Tuple, String> {
        let object_point = self.transform.inverse()? * world_point;
        let object_normal = object_point - Tuple::point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse()?.transpose() * object_normal;
        world_normal.w = 0.0;
        Ok(world_normal.normalize())
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Tuple::point(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Transform::identity(),
            material: Material::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test;
    use crate::{material::Material, MARGIN};

    use float_cmp::ApproxEq;
    use std::f64::consts::{FRAC_1_SQRT_2, PI};
    use test::sqrt_n_over_n;

    #[test]
    fn default_material() {
        let s = Sphere::default();

        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn changing_material() {
        let mut s = Sphere::default();
        let mut m = Material::default();
        m.ambient = 1.0;

        s.material = m;
        assert_eq!(s.material, m);
    }

    #[test]
    fn default_transformation() {
        let s = Sphere::default();

        assert_eq!(s.transform, Transform::identity())
    }

    #[test]
    fn changing_sphere_transformation() {
        let mut s = Sphere::default();
        let t = Transform::translation(2.0, 3.0, 4.0);

        s.set_transform(t.clone());

        assert_eq!(s.transform, t);
    }

    #[test]
    fn normal_at_a_point_on_x_axis() {
        let s = Sphere::default();

        let n = s.normal_at(Tuple::point(1.0, 0.0, 0.0)).unwrap();

        assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_at_a_point_on_y_axis() {
        let s = Sphere::default();

        let n = s.normal_at(Tuple::point(0.0, 1.0, 0.0)).unwrap();

        assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_at_a_point_on_z_axis() {
        let s = Sphere::default();

        let n = s.normal_at(Tuple::point(0.0, 0.0, 1.0)).unwrap();

        assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_at_a_nonaxial_point() {
        let s = Sphere::default();

        let n = s
            .normal_at(Tuple::point(
                sqrt_n_over_n(3),
                sqrt_n_over_n(3),
                sqrt_n_over_n(3),
            ))
            .unwrap();

        assert_eq!(
            n,
            Tuple::vector(sqrt_n_over_n(3), sqrt_n_over_n(3), sqrt_n_over_n(3))
        );
    }

    #[test]
    fn normal_is_normalized() {
        let s = Sphere::default();

        let n = s
            .normal_at(Tuple::point(
                sqrt_n_over_n(3),
                sqrt_n_over_n(3),
                sqrt_n_over_n(3),
            ))
            .unwrap();

        assert_eq!(n, n.normalize());
    }

    #[test]
    fn normal_on_a_translated_sphere() {
        let mut s = Sphere::default();
        s.set_transform(Transform::translation(0.0, 1.0, 0.0));

        let n = s
            .normal_at(Tuple::point(0.0, 1.0 + FRAC_1_SQRT_2, -FRAC_1_SQRT_2))
            .unwrap();

        println!("n: {:?}", n);
        println!("v: {:?}", Tuple::vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
        f_assert_eq!(n, &Tuple::vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2));
    }

    #[test]
    fn normal_on_a_transformed_sphere() {
        let mut s = Sphere::default();
        let m = Transform::scaling(1.0, 0.5, 1.0) * Transform::rotation_z(PI / 5.0);
        s.set_transform(m);

        let n = s
            .normal_at(Tuple::point(0.0, sqrt_n_over_n(2), -sqrt_n_over_n(2)))
            .unwrap();

        assert!(n.approx_eq(&Tuple::vector(0.0, 0.97014, -0.24254), MARGIN));
    }
}