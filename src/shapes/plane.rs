use crate::{
    intersection::{Intersection, Intersections},
    materials::Material,
    ray::Ray,
    shapes::{Shape, ShapeBuilder},
    transformations::Transform,
    tuple::Tuple,
    MARGIN,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Plane {
    material: Material,
    transform: Transform,
}

impl ShapeBuilder for Plane {
    fn with_transform(self, transform: Transform) -> Self {
        Self { transform, ..self }
    }

    fn with_material(self, material: Material) -> Self {
        Self { material, ..self }
    }
}

impl Shape for Plane {
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

    fn normal_at(&self, _x: f32, _y: f32, _z: f32) -> Tuple {
        Tuple::vector(0.0, 1.0, 0.0)
    }

    fn intersect(&self, ray: Ray) -> Intersections {
        if ray.direction.y().abs() < MARGIN.epsilon {
            Intersections::new(vec![])
        } else {
            let t = -ray.origin.y() / ray.direction.y();
            Intersections::new(vec![Intersection::new(t, self)])
        }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::default();

        // Note: the book uses `local_normal_at` but I didn't
        let n1 = p.normal_at(0.0, 0.0, 0.0);
        let n2 = p.normal_at(10.0, 0.0, -10.0);
        let n3 = p.normal_at(-5.0, 0.0, 150.0);

        assert_eq!(n1, Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(n2, Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(n3, Tuple::vector(0.0, 1.0, 0.0));
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
        assert!(approx_eq!(f32, xs[0].t, 1.0));
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::default();
        let r = Ray::default()
            .origin(0.0, -1.0, 0.0)
            .direction(0.0, 1.0, 0.0);

        let xs = p.intersect(r);

        assert_eq!(xs.len(), 1);
        assert!(approx_eq!(f32, xs[0].t, 1.0));
    }
}
