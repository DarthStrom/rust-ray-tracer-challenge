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

    fn local_intersect(&self, _ray: Ray) -> Vec<Intersection> {
        todo!()
    }

    fn local_normal_at(&self, _point: Tuple) -> Tuple {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::shapes::TestShape;

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
}
