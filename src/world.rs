use color::Color;
use material::Material;

use crate::{
    color,
    light::PointLight,
    material,
    matrix::transform::Transform,
    ray::{
        intersections::{Computations, Intersections},
        Ray,
    },
    sphere::Sphere,
    tuple::Tuple,
};

pub struct World {
    pub light_sources: Vec<PointLight>,
    pub objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: vec![],
            light_sources: vec![],
        }
    }

    pub fn light_sources(self, light_sources: &[PointLight]) -> Self {
        Self {
            light_sources: light_sources.to_vec(),
            ..self
        }
    }

    pub fn object(self, object: Sphere) -> Self {
        let mut objects = self.objects;
        objects.push(object);

        Self { objects, ..self }
    }

    pub fn objects(self, objects: &[Sphere]) -> Self {
        let mut existing_objects = self.objects;
        existing_objects.append(&mut objects.to_vec());
        Self {
            objects: existing_objects,
            ..self
        }
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        if let Some(hit) = self.intersect(ray).hit() {
            if let Ok(comps) = hit.prepare_computations(ray) {
                self.shade_hit(comps)
            } else {
                Color::default()
            }
        } else {
            Color::default()
        }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let mut vec = self
            .objects
            .iter()
            .flat_map(|o| o.intersect(ray).data)
            .collect::<Vec<_>>();

        vec.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        Intersections::new(vec)
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        self.light_sources
            .iter()
            .map(|light| light.position - point)
            .any(|v| {
                let distance = v.magnitude();
                let direction = v.normalize();

                let r = Ray::new(point, direction);
                let intersections = self.intersect(r);

                if let Some(h) = intersections.hit() {
                    h.t < distance
                } else {
                    false
                }
            })
    }

    pub fn shade_hit(&self, comps: Computations) -> Color {
        self.light_sources
            .iter()
            .map(|&light| {
                comps
                    .object
                    .material
                    .lighting(light, comps.point, comps.eyev, comps.normalv, false)
            })
            .fold(Color::default(), |acc, c| acc + c)
    }
}

impl Default for World {
    fn default() -> Self {
        let s1 = Sphere::default().material(
            Material::default()
                .color(Color::new(0.8, 1.0, 0.6))
                .diffuse(0.7)
                .specular(0.2),
        );
        let s2 = Sphere::default().transform(Transform::scaling(0.5, 0.5, 0.5));
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
    use crate::{
        color::Color, material::Material, matrix::transform::Transform, ray::Ray, tuple::Tuple,
    };
    use float_cmp::ApproxEq;

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
        let s1 = Sphere::default().material(
            Material::default()
                .color(Color::new(0.8, 1.0, 0.6))
                .diffuse(0.7)
                .specular(0.2),
        );
        let s2 = Sphere::default().transform(Transform::scaling(0.5, 0.5, 0.5));

        let w = World::default();

        assert_eq!(w.light_sources, vec![light]);
        assert_eq!(w.objects, vec![s1, s2]);
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let xs = w.intersect(r);

        assert_eq!(xs.len(), 4);
        f_assert_eq!(xs[0].t, 4.0);
        f_assert_eq!(xs[1].t, 4.5);
        f_assert_eq!(xs[2].t, 5.5);
        f_assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));

        let c = w.color_at(r);

        f_assert_eq!(c, &Color::default());
    }

    #[test]
    fn color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let c = w.color_at(r);

        f_assert_eq!(c, &Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_whith_an_intersection_behind_ray() {
        let mut w = World::default();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;
        let inner = &w.objects[1];
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));

        let c = w.color_at(r);

        f_assert_eq!(c, &inner.material.color);
    }

    #[test]
    fn no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::default();
        let p = Tuple::point(0.0, 10.0, 0.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn shadow_when_object_between_point_and_light() {
        let w = World::default();
        let p = Tuple::point(10.0, -10.0, 10.0);

        assert!(w.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_object_behind_light() {
        let w = World::default();
        let p = Tuple::point(-20.0, 20.0, -20.0);

        assert!(!w.is_shadowed(p));
    }

    #[test]
    fn no_shadow_when_object_behind_point() {
        let w = World::default();
        let p = Tuple::point(-2.0, 2.0, -2.0);

        assert!(!w.is_shadowed(p));
    }
}
