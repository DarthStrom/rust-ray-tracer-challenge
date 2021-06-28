use crate::{
    intersection::{Intersection, Intersections},
    materials::Material,
    ray::Ray,
    shapes::{Shape, ShapeBuilder},
    transformations::Transform,
    tuple::Tuple,
    MARGIN,
};
use float_cmp::approx_eq;
use std::f32::{MAX, MIN};

#[derive(Clone, Debug, PartialEq)]
pub struct Cylinder {
    material: Material,
    transform: Transform,
    minimum: f32,
    maximum: f32,
    closed: bool,
}

impl Cylinder {
    pub fn with_caps(self, bottom: f32, top: f32) -> Self {
        Self {
            closed: true,
            minimum: bottom,
            maximum: top,
            ..self
        }
    }

    fn intersect_caps<'a>(&'a self, ray: Ray, xs: Intersections<'a>) -> Intersections<'a> {
        let mut result = xs.clone();
        if !self.closed || approx_eq!(f32, ray.direction.y(), 0.0) {
            return result;
        }

        let t = (self.minimum - ray.origin.y()) / ray.direction.y();
        if check_cap(ray, t) {
            result.push(Intersection::new(t, self));
        }

        let t = (self.maximum - ray.origin.y()) / ray.direction.y();
        if check_cap(ray, t) {
            result.push(Intersection::new(t, self));
        }

        result
    }

    fn local_intersect(&self, ray: Ray) -> Intersections {
        let a = ray.direction.x().powf(2.0) + ray.direction.z().powf(2.0);
        if approx_eq!(f32, a, 0.0) {
            return self.intersect_caps(ray, Intersections::new(vec![]));
        }

        let b = 2.0 * ray.origin.x() * ray.direction.x() + 2.0 * ray.origin.z() * ray.direction.z();

        let c = ray.origin.x().powf(2.0) + ray.origin.z().powf(2.0) - 1.0;

        let disc = b.powf(2.0) - 4.0 * a * c;

        if disc < 0.0 {
            Intersections::new(vec![])
        } else {
            let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
            let mut t1 = (-b + disc.sqrt()) / (2.0 * a);
            if t0 > t1 {
                let temp = t0;
                t0 = t1;
                t1 = temp;
            }
            let mut xs = Intersections::new(vec![]);

            let y0 = ray.origin.y() + t0 * ray.direction.y();
            if self.minimum < y0 && y0 < self.maximum {
                xs.push(Intersection::new(t0, self));
            }

            let y1 = ray.origin.y() + t1 * ray.direction.y();
            if self.minimum < y1 && y1 < self.maximum {
                xs.push(Intersection::new(t1, self));
            }

            self.intersect_caps(ray, xs)
        }
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        match point.x().powf(2.0) + point.z().powf(2.0) {
            dist if dist < 1.0 && point.y() >= self.maximum - MARGIN.epsilon => {
                Tuple::vector(0.0, 1.0, 0.0)
            }
            dist if dist < 1.0 && point.y() <= self.minimum + MARGIN.epsilon => {
                Tuple::vector(0.0, -1.0, 0.0)
            }
            _ => Tuple::vector(point.x(), 0.0, point.z()),
        }
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            minimum: MIN,
            maximum: MAX,
            transform: Transform::default(),
            material: Material::default(),
            closed: false,
        }
    }
}

impl ShapeBuilder for Cylinder {
    fn with_transform(self, transform: Transform) -> Self {
        Self { transform, ..self }
    }

    fn with_material(self, material: Material) -> Self {
        Self { material, ..self }
    }
}

impl Shape for Cylinder {
    fn box_clone(&self) -> crate::shapes::BoxShape {
        Box::new((*self).clone())
    }

    fn box_eq(&self, other: &dyn std::any::Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn normal_at(&self, x: f32, y: f32, z: f32) -> Tuple {
        let world_point = Tuple::point(x, y, z);
        let obj_point = self.transform().inverse() * world_point;
        let object_normal = self.local_normal_at(obj_point);
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal = world_normal.to_vector();
        world_normal.normalize()
    }

    fn intersect(&self, ray: Ray) -> Intersections {
        let local_ray = ray.transform(self.transform().inverse());
        self.local_intersect(local_ray)
    }
}

fn check_cap(ray: Ray, t: f32) -> bool {
    let x = ray.origin.x() + t * ray.direction.x();
    let z = ray.origin.z() + t * ray.direction.z();

    x.powf(2.0) + z.powf(2.0) <= 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! a_ray_misses_a_cylinder {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (origin, direction) = $value;
                let c = Cylinder::default();
                let direction = direction.normalize();
                let r = Ray::new(origin, direction);
                let xs = c.local_intersect(r);

                assert_eq!(xs.len(), 0);
            }
        )*
        }
    }

    a_ray_misses_a_cylinder! {
        a_ray_misses_a_cylinder_1: (Tuple::point(1.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)),
        a_ray_misses_a_cylinder_2: (Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)),
        a_ray_misses_a_cylinder_3: (Tuple::point(0.0, 0.0, -5.0), Tuple::vector(1.0, 1.0, 1.0)),
    }

    macro_rules! a_ray_strikes_a_cylinder {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (origin, direction, t0, t1) = $value;
                let cyl = Cylinder::default();
                let direction = direction.normalize();
                let r = Ray::new(origin, direction);

                let xs = cyl.local_intersect(r);

                assert_eq!(xs.len(), 2);
                println!("{}, {}, {}, {}", xs[0].t, t0, xs[1].t, t1);
                assert!(approx_eq!(f32, xs[0].t, t0));
                assert!(approx_eq!(f32, xs[1].t, t1));
            }
        )*
        }
    }

    a_ray_strikes_a_cylinder! {
        a_ray_strikes_a_cylinder_1: (Tuple::point(1.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0), 5.0, 5.0),
        a_ray_strikes_a_cylinder_2: (Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0), 4.0, 6.0),
        a_ray_strikes_a_cylinder_3: (Tuple::point(0.5, 0.0, -5.0), Tuple::vector(0.1, 1.0, 1.0), 6.808006, 7.0886984),
    }

    macro_rules! normal_vector_on_a_cylinder {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (point, normal) = $value;
                let cyl = Cylinder::default();
                let n = cyl.local_normal_at(point);

                assert_eq!(n, normal);
            }
        )*
        }
    }

    normal_vector_on_a_cylinder! {
        normal_vector_on_a_cylinder_1: (Tuple::point(1.0, 0.0, 0.0), Tuple::vector(1.0, 0.0, 0.0)),
        normal_vector_on_a_cylinder_2: (Tuple::point(0.0, 5.0, -1.0), Tuple::vector(0.0, 0.0, -1.0)),
        normal_vector_on_a_cylinder_3: (Tuple::point(0.0, -2.0, 1.0), Tuple::vector(0.0, 0.0, 1.0)),
        normal_vector_on_a_cylinder_4: (Tuple::point(-1.0, 1.0, 0.0), Tuple::vector(-1.0, 0.0, 0.0)),
    }

    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder() {
        let cyl = Cylinder::default();

        assert!(approx_eq!(f32, cyl.minimum, MIN));
        assert!(approx_eq!(f32, cyl.maximum, MAX));
    }

    macro_rules! intersecting_a_contstrained_cylinder {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (point, direction, count) = $value;
                let mut cyl = Cylinder::default();
                cyl.minimum = 1.0;
                cyl.maximum = 2.0;
                let direction = direction.normalize();
                let r = Ray::new(point, direction);

                let xs = cyl.local_intersect(r);

                assert_eq!(xs.len(), count);
            }
        )*
        }
    }

    intersecting_a_contstrained_cylinder! {
        intersecting_a_contstrained_cylinder_1: (Tuple::point(0.0, 1.5, 0.0), Tuple::vector(0.1, 1.0, 0.0), 0),
        intersecting_a_contstrained_cylinder_2: (Tuple::point(0.0, 3.0, -5.0), Tuple::vector(0.0, 0.0, 1.0), 0),
        intersecting_a_contstrained_cylinder_3: (Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0), 0),
        intersecting_a_contstrained_cylinder_4: (Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0), 0),
        intersecting_a_contstrained_cylinder_5: (Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0), 0),
        intersecting_a_contstrained_cylinder_6: (Tuple::point(0.0, 1.5, -2.0), Tuple::vector(0.0, 0.0, 1.0), 2),
    }

    #[test]
    fn the_default_closed_value_for_a_cylinder() {
        let cyl = Cylinder::default();

        assert!(!cyl.closed);
    }

    macro_rules! intersecting_the_caps_of_a_closed_cylinder {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (point, direction, count) = $value;
                let mut cyl = Cylinder::default();
                cyl.minimum = 1.0;
                cyl.maximum = 2.0;
                cyl.closed = true;
                let direction = direction.normalize();
                let r = Ray::new(point, direction);

                let xs = cyl.local_intersect(r);

                assert_eq!(xs.len(), count);
            }
        )*
        }
    }

    // TODO: not sure why 3 and 5 don't pass
    intersecting_the_caps_of_a_closed_cylinder! {
        intersecting_the_caps_of_a_closed_cylinder_1: (Tuple::point(0.0, 3.0, 0.0), Tuple::vector(0.0, -1.0, 0.0), 2),
        intersecting_the_caps_of_a_closed_cylinder_2: (Tuple::point(0.0, 3.0, -2.0), Tuple::vector(0.0, -1.0, 2.0), 2),
        intersecting_the_caps_of_a_closed_cylinder_3: (Tuple::point(0.0, 4.0, -2.0), Tuple::vector(0.0, -1.0, 1.0), 2),
        intersecting_the_caps_of_a_closed_cylinder_4: (Tuple::point(0.0, 0.0, -2.0), Tuple::vector(0.0, 1.0, 2.0), 2),
        intersecting_the_caps_of_a_closed_cylinder_5: (Tuple::point(0.0, -1.0, -2.0), Tuple::vector(0.0, 1.0, 1.0), 2),
    }

    macro_rules! the_normal_vector_on_a_cylinders_end_caps {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (point, normal) = $value;
                let mut cyl = Cylinder::default();
                cyl.minimum = 1.0;
                cyl.maximum = 2.0;
                cyl.closed = true;

                let n = cyl.local_normal_at(point);

                assert_eq!(n, normal);
            }
        )*
        };
    }

    the_normal_vector_on_a_cylinders_end_caps! {
        the_normal_vector_on_a_cylinders_end_caps_1: (Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0)),
        the_normal_vector_on_a_cylinders_end_caps_2: (Tuple::point(0.5, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0)),
        the_normal_vector_on_a_cylinders_end_caps_3: (Tuple::point(0.0, 1.0, 0.5), Tuple::vector(0.0, -1.0, 0.0)),
        the_normal_vector_on_a_cylinders_end_caps_4: (Tuple::point(0.0, 2.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)),
        the_normal_vector_on_a_cylinders_end_caps_5: (Tuple::point(0.5, 2.0, 0.0), Tuple::vector(0.0, 1.0, 0.0)),
        the_normal_vector_on_a_cylinders_end_caps_6: (Tuple::point(0.0, 2.0, 0.5), Tuple::vector(0.0, 1.0, 0.0)),
    }
}
