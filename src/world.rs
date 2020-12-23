use color::Color;
use material::Material;

use crate::{
    color, light::PointLight, material, matrix::transform::Transform, sphere::Sphere, tuple::Tuple,
};

pub struct World {
    pub objects: Vec<Sphere>,
    pub light_sources: Vec<PointLight>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            light_sources: vec![],
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let mut s1 = Sphere::default();
        s1.material = Material::default()
            .color(Color::new(0.8, 1.0, 0.6))
            .diffuse(0.7)
            .specular(0.2);
        let mut s2 = Sphere::default();
        s2.set_transform(Transform::scaling(0.5, 0.5, 0.5));
        let objects = vec![s1, s2];

        let light_sources = vec![PointLight::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        )];

        Self {
            objects,
            light_sources,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, material::Material, matrix::transform::Transform, tuple::Tuple};

    use super::*;

    #[test]
    fn creating_a_world() {
        let w = World::new();

        assert!(w.objects.is_empty());
        assert!(w.light_sources.is_empty());
    }

    #[test]
    fn default_world() {
        let light = PointLight::new(Tuple::point(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let mut s1 = Sphere::default();
        s1.material = Material::default()
            .color(Color::new(0.8, 1.0, 0.6))
            .diffuse(0.7)
            .specular(0.2);
        let mut s2 = Sphere::default();
        s2.set_transform(Transform::scaling(0.5, 0.5, 0.5));

        let w = World::default();

        assert_eq!(w.light_sources, vec![light]);
        assert_eq!(w.objects, vec![s1, s2]);
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        // coming soon...
    }
}
