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
pub struct Cone {
    material: Material,
    transform: Transform,
    minimum: f32,
    maximum: f32,
    closed: bool,
}

impl Cone {
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
        if check_cap(ray, t, self.minimum) {
            result.push(Intersection::new(t, self));
        }

        let t = (self.maximum - ray.origin.y()) / ray.direction.y();
        if check_cap(ray, t, self.maximum) {
            result.push(Intersection::new(t, self));
        }

        result
    }

    fn local_intersect(&self, ray: Ray) -> Intersections {
        let a =
            ray.direction.x().powf(2.0) - ray.direction.y().powf(2.0) + ray.direction.z().powf(2.0);

        let b = 2.0 * ray.origin.x() * ray.direction.x() - 2.0 * ray.origin.y() * ray.direction.y()
            + 2.0 * ray.origin.z() * ray.direction.z();

        let c = ray.origin.x().powf(2.0) - ray.origin.y().powf(2.0) + ray.origin.z().powf(2.0);

        if a.abs() <= MARGIN.epsilon && b.abs() <= MARGIN.epsilon {
            return self.intersect_caps(ray, Intersections::new(vec![]));
        } else if a.abs() <= MARGIN.epsilon {
            return self.intersect_caps(
                ray,
                Intersections::new(vec![Intersection::new(-c / (2.0 * b), self)]),
            );
        }

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
            dist => {
                let mut y = dist.sqrt();
                if point.y() > 0.0 {
                    y = -y;
                }
                Tuple::vector(point.x(), y, point.z())
            }
        }
    }
}

impl Default for Cone {
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

impl ShapeBuilder for Cone {
    fn with_transform(self, transform: Transform) -> Self {
        Self { transform, ..self }
    }

    fn with_material(self, material: Material) -> Self {
        Self { material, ..self }
    }
}

impl Shape for Cone {
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

fn check_cap(ray: Ray, t: f32, y: f32) -> bool {
    let x = ray.origin.x() + t * ray.direction.x();
    let z = ray.origin.z() + t * ray.direction.z();

    x.powf(2.0) + z.powf(2.0) <= y.powf(2.0)
}

#[cfg(test)]
mod tests {
    use std::f32::consts::SQRT_2;

    use super::*;

    macro_rules! intersecting_a_cone_with_a_ray {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (origin, direction, t0, t1) = $value;
                let shape = Cone::default();
                let r = Ray::new(origin, direction.normalize());

                let xs = shape.local_intersect(r);

                assert_eq!(xs.len(), 2);
                assert!(approx_eq!(f32, xs[0].t, t0));
                assert!(approx_eq!(f32, xs[1].t, t1));
            }
        )*
        }
    }

    intersecting_a_cone_with_a_ray! {
        intersecting_a_cone_with_a_ray_1: (Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0), 5.0, 5.0),
        intersecting_a_cone_with_a_ray_2: (Tuple::point(0.0, 0.0, -5.0), Tuple::vector(1.0, 1.0, 1.0), 8.66025, 8.66025),
        intersecting_a_cone_with_a_ray_3: (Tuple::point(1.0, 1.0, -5.0), Tuple::vector(-0.5, -1.0, 1.0), 4.550057, 49.44995),
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let shape = Cone::default();
        let direction = Tuple::vector(0.0, 1.0, 1.0);
        let r = Ray::new(Tuple::point(0.0, 0.0, -1.0), direction);

        let xs = shape.local_intersect(r);

        assert_eq!(xs.len(), 1);
        println!("{}", xs[0].t);
        assert!(approx_eq!(f32, xs[0].t, 0.35355));
    }

    macro_rules! intersecting_a_cones_end_caps {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (origin, direction, count) = $value;
                let shape = Cone::default().with_caps(-0.5, 0.5);
                let dir = direction.normalize();
                let r = Ray::new(origin, dir);

                let xs = shape.local_intersect(r);

                assert_eq!(xs.len(), count);
            }
        )*
        }
    }

    intersecting_a_cones_end_caps! {
        intersecting_a_cones_end_caps_1: (Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0), 0),
        intersecting_a_cones_end_caps_2: (Tuple::point(0.0, 0.0, -0.25), Tuple::vector(0.0, 1.0, 1.0), 2),
        intersecting_a_cones_end_caps_3: (Tuple::point(0.0, 0.0, -0.25), Tuple::vector(0.0, 1.0, 0.0), 4),
    }

    macro_rules! computing_the_normal_vector_on_a_cone {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (point, normal) = $value;
                let shape = Cone::default();
                let n = shape.local_normal_at(point);

                assert_eq!(n, normal);
            }
        )*
        }
    }

    computing_the_normal_vector_on_a_cone! {
        computing_the_normal_vector_on_a_cone_1: (Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 0.0)),
        computing_the_normal_vector_on_a_cone_2: (Tuple::point(1.0, 1.0, 1.0), Tuple::vector(1.0, -SQRT_2, 1.0)),
        computing_the_normal_vector_on_a_cone_3: (Tuple::point(-1.0, -1.0, 0.0), Tuple::vector(-1.0, 1.0, 0.0)),
    }
}
