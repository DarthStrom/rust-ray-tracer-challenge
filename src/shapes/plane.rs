use uuid::Uuid;

use crate::{
    intersection::Intersection,
    materials::Material,
    ray::Ray,
    shapes::{Shape, ShapeBuilder},
    transformations::{Transform, IDENTITY},
    tuple::Tuple,
    EPSILON,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Plane {
    id: Uuid,
    parent: Option<Uuid>,
    material: Material,
    transform: Transform,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            parent: None,
            material: Material::default(),
            transform: IDENTITY,
        }
    }
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
    fn id(&self) -> Uuid {
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
        if ray.direction.y().abs() < EPSILON {
            vec![]
        } else {
            let t = -ray.origin.y() / ray.direction.y();
            vec![Intersection::new(t, self)]
        }
    }

    fn local_normal_at(&self, _point: Tuple) -> Tuple {
        Tuple::vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::float_eq;

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
        assert!(float_eq(xs[0].t, 1.0));
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::default();
        let r = Ray::default()
            .origin(0.0, -1.0, 0.0)
            .direction(0.0, 1.0, 0.0);

        let xs = p.intersect(r);

        assert_eq!(xs.len(), 1);
        assert!(float_eq(xs[0].t, 1.0));
    }
}
