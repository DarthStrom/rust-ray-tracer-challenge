use crate::{
    float_eq,
    intersection::Intersection,
    materials::Material,
    ray::Ray,
    shapes::{Shape, ShapeBuilder},
    transformations::Transform,
    tuple::Tuple,
    EPSILON,
};
use std::f32::{MAX, MIN};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Cone {
    id: Uuid,
    parent: Option<Uuid>,
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

    fn intersect_caps<'a>(&'a self, ray: Ray, xs: &[Intersection<'a>]) -> Vec<Intersection<'a>> {
        let mut result = xs.to_vec();
        if !self.closed || float_eq(ray.direction.y(), 0.0) {
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
}

impl Default for Cone {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            parent: None,
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
    fn id(&self) -> uuid::Uuid {
        self.id
    }

    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
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

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let a = ray.direction.x().powi(2) - ray.direction.y().powi(2) + ray.direction.z().powi(2);

        let b = 2.0 * ray.origin.x() * ray.direction.x() - 2.0 * ray.origin.y() * ray.direction.y()
            + 2.0 * ray.origin.z() * ray.direction.z();

        let c = ray.origin.x().powi(2) - ray.origin.y().powi(2) + ray.origin.z().powi(2);

        if float_eq(a, 0.0) && float_eq(b, 0.0) {
            return self.intersect_caps(ray, &[]);
        } else if float_eq(a, 0.0) {
            return self.intersect_caps(ray, &[Intersection::new(-c / (2.0 * b), self)]);
        }

        let disc = b.powi(2) - 4.0 * a * c + EPSILON;

        if disc < 0.0 {
            vec![]
        } else {
            let mut t = (
                (-b - disc.sqrt()) / (2.0 * a),
                (-b + disc.sqrt()) / (2.0 * a),
            );
            if t.0 > t.1 {
                t = (t.1, t.0);
            }
            let mut xs: Vec<Intersection> = vec![];

            let y0 = ray.origin.y() + t.0 * ray.direction.y();
            if self.minimum < y0 && y0 < self.maximum {
                xs.push(Intersection::new(t.0, self));
            }

            let y1 = ray.origin.y() + t.1 * ray.direction.y();
            if self.minimum < y1 && y1 < self.maximum {
                xs.push(Intersection::new(t.1, self));
            }

            self.intersect_caps(ray, &xs)
        }
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        match point.x().powi(2) + point.z().powi(2) {
            dist if dist < 1.0 && point.y() >= self.maximum - EPSILON => {
                Tuple::vector(0.0, 1.0, 0.0)
            }
            dist if dist < 1.0 && point.y() <= self.minimum + EPSILON => {
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

fn check_cap(ray: Ray, t: f32, y: f32) -> bool {
    let x = ray.origin.x() + t * ray.direction.x();
    let z = ray.origin.z() + t * ray.direction.z();

    x.powi(2) + z.powi(2) <= y.powi(2)
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
                assert!(float_eq(xs[0].t, t0));
                assert!(float_eq(xs[1].t, t1));
            }
        )*
        }
    }

    intersecting_a_cone_with_a_ray! {
        intersecting_a_cone_with_a_ray_1: (Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0), 4.995, 5.005),
        intersecting_a_cone_with_a_ray_2: (Tuple::point(0.0, 0.0, -5.0), Tuple::vector(1.0, 1.0, 1.0), 8.645543, 8.674966),
        intersecting_a_cone_with_a_ray_3: (Tuple::point(1.0, 1.0, -5.0), Tuple::vector(-0.5, -1.0, 1.0), 4.550057, 49.44995),
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let shape = Cone::default();
        let direction = Tuple::vector(0.0, 1.0, 1.0);
        let r = Ray::new(Tuple::point(0.0, 0.0, -1.0), direction.normalize());

        let xs = shape.local_intersect(r);

        assert_eq!(xs.len(), 1);
        assert!(float_eq(xs[0].t, 0.35355));
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
