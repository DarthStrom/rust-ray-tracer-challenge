use uuid::Uuid;

use crate::{
    float_eq,
    intersection::Intersection,
    materials::Material,
    ray::Ray,
    shapes::{Shape, ShapeBuilder},
    transformations::{Transform, IDENTITY},
    tuple::Tuple,
    EPSILON,
};
use std::{cmp::Ordering::Equal, f32::MAX};

#[derive(Clone, Debug, PartialEq)]
pub struct Cube {
    id: Uuid,
    parent: Option<Uuid>,
    material: Material,
    transform: Transform,
}

impl Default for Cube {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            parent: None,
            material: Material::default(),
            transform: IDENTITY,
        }
    }
}

fn sorted(nums: &[f32]) -> Vec<f32> {
    let mut result = nums.to_vec();
    result.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    result
}

fn min(nums: &[f32]) -> f32 {
    *sorted(nums).first().unwrap()
}

fn max(nums: &[f32]) -> f32 {
    *sorted(nums).last().unwrap()
}

fn check_axis(origin: f32, direction: f32) -> (f32, f32) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let (tmin, tmax) = if direction.abs() >= EPSILON {
        (tmin_numerator / direction, tmax_numerator / direction)
    } else {
        (tmin_numerator * MAX, tmax_numerator * MAX)
    };

    if tmin > tmax {
        (tmax, tmin)
    } else {
        (tmin, tmax)
    }
}

impl ShapeBuilder for Cube {
    fn with_transform(self, transform: Transform) -> Self {
        Self { transform, ..self }
    }

    fn with_material(self, material: Material) -> Self {
        Self { material, ..self }
    }
}

impl Shape for Cube {
    fn id(&self) -> uuid::Uuid {
        self.id
    }

    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn parent(&self) -> Option<Uuid> {
        self.parent
    }

    fn set_parent(&mut self, parent: Uuid) {
        self.parent = Some(parent);
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let (xtmin, xtmax) = check_axis(ray.origin.x(), ray.direction.x());
        let (ytmin, ytmax) = check_axis(ray.origin.y(), ray.direction.y());
        let (ztmin, ztmax) = check_axis(ray.origin.z(), ray.direction.z());

        let tmin = max(&[xtmin, ytmin, ztmin]);
        let tmax = min(&[xtmax, ytmax, ztmax]);

        if tmin > tmax {
            return vec![];
        }

        vec![Intersection::new(tmin, self), Intersection::new(tmax, self)]
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        let abs_x = point.x().abs();
        let abs_y = point.y().abs();
        let abs_z = point.z().abs();

        let maxc = max(&[abs_x, abs_y, abs_z]);

        match maxc {
            _ if float_eq(maxc, abs_x) => Tuple::vector(point.x(), 0.0, 0.0),
            _ if float_eq(maxc, abs_y) => Tuple::vector(0.0, point.y(), 0.0),
            _ if float_eq(maxc, abs_z) => Tuple::vector(0.0, 0.0, point.z()),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::float_eq;

    use super::*;

    #[test]
    fn a_ray_intersects_a_cube() {
        let c = Cube::default();
        let r = Ray::default()
            .origin(5.0, 0.5, 0.0)
            .direction(-1.0, 0.0, 0.0);
        let xs = c.local_intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(float_eq(xs[0].t, 4.0));
        assert!(float_eq(xs[1].t, 6.0));
    }

    macro_rules! a_ray_intersects_a_cube {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (origin, direction, t1, t2) = $value;
                let c = Cube::default();
                let r = Ray::new(origin, direction);
                let xs = c.local_intersect(r);

                assert_eq!(xs.len(), 2);
                assert!(float_eq(xs[0].t, t1));
                assert!(float_eq(xs[1].t, t2));
            }
        )*
        }
    }

    a_ray_intersects_a_cube! {
        a_ray_intersects_a_cube_plus_x: (Tuple::point(5.0, 0.5, 0.0), Tuple::vector(-1.0, 0.0, 0.0), 4.0, 6.0),
        a_ray_intersects_a_cube_minus_x: (Tuple::point(-5.0, 0.5, 0.0), Tuple::vector(1.0, 0.0, 0.0), 4.0, 6.0),
        a_ray_intersects_a_cube_plus_y: (Tuple::point(0.5, 5.0, 0.0), Tuple::vector(0.0, -1.0, 0.0), 4.0, 6.0),
        a_ray_intersects_a_cube_minus_y: (Tuple::point(0.5, -5.0, 0.0), Tuple::vector(0.0, 1.0, 0.0), 4.0, 6.0),
        a_ray_intersects_a_cube_plus_z: (Tuple::point(0.5, 0.0, 5.0), Tuple::vector(0.0, 0.0, -1.0), 4.0, 6.0),
        a_ray_intersects_a_cube_minus_z: (Tuple::point(0.5, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0), 4.0, 6.0),
        a_ray_intersects_a_cube_inside: (Tuple::point(0.0, 0.5, 0.0), Tuple::vector(0.0, 0.0, 1.0), -1.0, 1.0),
    }

    macro_rules! a_ray_misses_a_cube {
        ($($name:ident: $value: expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (origin, direction) = $value;
                let c = Cube::default();
                let r = Ray::new(origin, direction);
                let xs = c.local_intersect(r);

                assert_eq!(xs.len(), 0);
            }
        )*
        }
    }

    a_ray_misses_a_cube! {
        a_ray_misses_a_cube_1: (Tuple::point(-2.0, 0.0, 0.0), Tuple::vector(0.2673, 0.5345, 0.8018)),
        a_ray_misses_a_cube_2: (Tuple::point(0.0, -2.0, 0.0), Tuple::vector(0.8018, 0.2673, 0.5345)),
        a_ray_misses_a_cube_3: (Tuple::point(0.0, 0.0, -2.0), Tuple::vector(0.5345, 0.8018, 0.2673)),
        a_ray_misses_a_cube_4: (Tuple::point(2.0, 0.0, 2.0), Tuple::vector(0.0, 0.0, -1.0)),
        a_ray_misses_a_cube_5: (Tuple::point(0.0, 2.0, 2.0), Tuple::vector(0.0, -1.0, 0.0)),
        a_ray_misses_a_cube_6: (Tuple::point(2.0, 2.0, 0.0), Tuple::vector(-1.0, 0.0, 0.0)),
    }

    macro_rules! the_normal_on_the_surface_of_a_cube {
        ($($name:ident: $value: expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (point, expected_normal) = $value;
                let c = Cube::default();

                let actual_normal = c.local_normal_at(point);

                assert_eq!(actual_normal, expected_normal);
            }
        )*
        }
    }

    the_normal_on_the_surface_of_a_cube! {
        the_normal_on_the_surface_of_a_cube_1: (Tuple::point(1.0, 0.5, -0.8), Tuple::vector(1.0, 0.0, 0.0)),
        the_normal_on_the_surface_of_a_cube_2: (Tuple::point(-1.0, -0.2, 0.9), Tuple::vector(-1.0, 0.0, 0.0)),
        the_normal_on_the_surface_of_a_cube_3: (Tuple::point(-0.4, 1.0, -0.1), Tuple::vector(0.0, 1.0, 0.0)),
        the_normal_on_the_surface_of_a_cube_4: (Tuple::point(0.3, -1.0, -0.7), Tuple::vector(0.0, -1.0, 0.0)),
        the_normal_on_the_surface_of_a_cube_5: (Tuple::point(-0.6, 0.3, 1.0), Tuple::vector(0.0, 0.0, 1.0)),
        the_normal_on_the_surface_of_a_cube_6: (Tuple::point(0.4, 0.4, -1.0), Tuple::vector(0.0, 0.0, -1.0)),
        the_normal_on_the_surface_of_a_cube_7: (Tuple::point(1.0, 1.0, 1.0), Tuple::vector(1.0, 0.0, 0.0)),
        the_normal_on_the_surface_of_a_cube_8: (Tuple::point(-1.0, -1.0, -1.0), Tuple::vector(-1.0, 0.0, 0.0)),
    }
}
