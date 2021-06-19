use crate::{
    color::Color,
    intersection::{Computations, Intersection, Intersections},
    lights::PointLight,
    materials::Material,
    ray::Ray,
    shapes::{sphere::Sphere, Shape, ShapeBuilder},
    transformations::Transform,
    tuple::Tuple,
};

#[derive(Debug)]
pub struct World {
    light_sources: Vec<PointLight>,
    objects: Vec<Box<dyn Shape>>,
}

impl World {
    pub fn new() -> Self {
        World {
            light_sources: vec![],
            objects: vec![],
        }
    }

    pub fn light_source(self, light_source: PointLight) -> Self {
        Self {
            light_sources: vec![light_source],
            ..self
        }
    }

    pub fn object(self, object: Box<dyn Shape>) -> Self {
        let mut objects = self.objects;
        objects.push(object);

        Self { objects, ..self }
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        if let Some(hit) = self.intersect(ray).hit() {
            let comps = hit.prepare_computations(ray);
            self.shade_hit(comps)
        } else {
            Color::default()
        }
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let mut vec = self
            .objects
            .iter()
            .flat_map(|o| o.intersect(ray).vec())
            .collect::<Vec<Intersection>>();

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
        // TODO: try multiple light sources.  It will slow things down though
        comps.object.material().lighting(
            self.light_sources[0],
            comps.point,
            comps.eyev,
            comps.normalv,
            self.is_shadowed(comps.over_point),
        )
    }
}

impl Default for World {
    fn default() -> Self {
        let sphere1 = Sphere::default().with_material(
            Material::default()
                .color(Color::new(0.8, 1.0, 0.6))
                .diffuse(0.7)
                .specular(0.2),
        );
        let sphere2 = Sphere::default().with_transform(Transform::scaling(0.5, 0.5, 0.5));
        Self {
            light_sources: vec![PointLight::new(
                Tuple::point(-10.0, 10.0, -10.0),
                Color::new(1.0, 1.0, 1.0),
            )],
            objects: vec![Box::new(sphere1), Box::new(sphere2)],
        }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::shapes::ShapeBuilder;

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
        let s1 = Sphere::default().with_material(
            Material::default()
                .color(Color::new(0.8, 1.0, 0.6))
                .diffuse(0.7)
                .specular(0.2),
        );
        let s2 = Sphere::default().with_transform(Transform::scaling(0.5, 0.5, 0.5));

        let w = World::default();

        assert_eq!(w.light_sources, vec![light]);
        assert_eq!(w.objects[0].material(), s1.material());
        assert_eq!(w.objects[1].material(), s2.material());
        assert_eq!(w.objects[0].transform(), s1.transform());
        assert_eq!(w.objects[1].transform(), s2.transform());
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let xs = w.intersect(r);

        assert_eq!(xs.len(), 4);
        assert!(approx_eq!(f32, xs[0].t, 4.0));
        assert!(approx_eq!(f32, xs[1].t, 4.5));
        assert!(approx_eq!(f32, xs[2].t, 5.5));
        assert!(approx_eq!(f32, xs[3].t, 6.0));
    }

    #[test]
    fn shading_an_intersection() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.objects[0].as_ref();
        let i = Intersection::new(4.0, shape);

        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let w = World {
            light_sources: vec![PointLight::new(
                Tuple::point(0.0, 0.25, 0.0),
                Color::new(1.0, 1.0, 1.0),
            )],
            ..World::default()
        };
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.objects[1].as_ref();
        let i = Intersection::new(0.5, shape);

        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));

        let c = w.color_at(r);

        assert_eq!(c, Color::default());
    }

    #[test]
    fn color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let c = w.color_at(r);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn color_with_an_intersection_behind_ray() {
        let outer = Sphere::default().with_material(
            Material::default()
                .color(Color::new(0.8, 1.0, 0.6))
                .diffuse(0.7)
                .specular(0.2)
                .ambient(1.0),
        );
        let inner = Sphere::default()
            .with_transform(Transform::scaling(0.5, 0.5, 0.5))
            .with_material(Material::default().ambient(1.0));
        let w = World::new()
            .light_source(PointLight::new(
                Tuple::point(-10.0, 10.0, -10.0),
                Color::new(1.0, 1.0, 1.0),
            ))
            .object(Box::new(outer))
            .object(Box::new(inner.clone()));

        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));

        let c = w.color_at(r);

        assert_eq!(c, inner.material.color);
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

    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let w = World::new()
            .light_source(
                PointLight::default()
                    .position(0.0, 0.0, -10.0)
                    .intensity(1.0, 1.0, 1.0),
            )
            .object(Box::new(Sphere::default()))
            .object(Box::new(
                Sphere::default().with_transform(Transform::translation(0.0, 0.0, 10.0)),
            ));
        let r = Ray::default()
            .origin(0.0, 0.0, 5.0)
            .direction(0.0, 0.0, 1.0);
        let i = Intersection::new(4.0, w.objects[1].as_ref());

        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
}
