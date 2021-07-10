use uuid::Uuid;

use crate::{
    intersection::Intersection,
    materials::Material,
    ray::Ray,
    shapes::Shape,
    transformations::{Transform, IDENTITY},
    tuple::Tuple,
};

#[derive(Debug, PartialEq)]
pub struct Group {
    id: Uuid,
    parent: Option<Uuid>,
    pub transform: Transform,
    pub material: Material,
    pub objects: Vec<Box<dyn Shape>>,
}

impl Group {
    fn new() -> Self {
        Self::default()
    }

    fn add_child(&mut self, mut child: Box<dyn Shape>) {
        child.set_parent(self.id);
        self.objects.push(child)
    }
}

impl Default for Group {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            parent: None,
            transform: IDENTITY,
            material: Material::default(),
            objects: vec![],
        }
    }
}

impl Shape for Group {
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
        let mut result = vec![];

        for object in &self.objects {
            let intersections = object.intersect(ray);
            for intersection in intersections {
                result.push(intersection);
            }
        }

        result.sort_by(|a, b| a.partial_cmp(b).unwrap());
        result
    }

    fn local_normal_at(&self, _point: Tuple) -> Tuple {
        panic!("Don't call me bro!")
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::{sphere::Sphere, ShapeBuilder, TestShape};

    use super::*;

    #[test]
    fn creating_a_new_group() {
        let g = Group::new();

        assert!(g.objects.is_empty());
        assert_eq!(g.transform, IDENTITY);
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let mut g = Group::new();
        let s = TestShape::new();

        g.add_child(Box::new(s));

        assert!(!g.objects.is_empty());
        assert_eq!(g.objects[0].parent(), Some(g.id));
    }

    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let g = Group::new();
        let r = Ray::default()
            .origin(0.0, 0.0, 0.0)
            .direction(0.0, 0.0, 1.0);

        let xs = g.local_intersect(r);

        assert!(xs.is_empty())
    }

    #[test]
    fn intersecting_a_ray_with_a_nonempty_group() {
        let mut g = Group::new();

        let s1 = Sphere::new();
        let s1_id = s1.id();

        let s2 = Sphere::new().with_transform(Transform::translation(0.0, 0.0, -3.0));
        let s2_id = s2.id();

        let s3 = Sphere::new().with_transform(Transform::translation(5.0, 0.0, 0.0));

        g.add_child(Box::new(s1));
        g.add_child(Box::new(s2));
        g.add_child(Box::new(s3));

        let r = Ray::default()
            .origin(0.0, 0.0, -5.0)
            .direction(0.0, 0.0, 1.0);
        let xs = g.local_intersect(r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object.id(), s2_id);
        assert_eq!(xs[1].object.id(), s2_id);
        assert_eq!(xs[2].object.id(), s1_id);
        assert_eq!(xs[3].object.id(), s1_id);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let mut g = Group::new();
        g.transform = Transform::scaling(2.0, 2.0, 2.0);

        let s = Sphere::new().with_transform(Transform::translation(5.0, 0.0, 0.0));

        g.add_child(Box::new(s));

        let r = Ray::default()
            .origin(10.0, 0.0, -10.0)
            .direction(0.0, 0.0, 1.0);

        let xs = g.intersect(r);
        assert_eq!(xs.len(), 2);
    }
}
