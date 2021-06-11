use crate::{
    color::Color,
    intersection::{Computations, Intersection, Intersections},
    lights::PointLight,
    materials::Material,
    ray::Ray,
    sphere::Sphere,
    transform::Transform,
    tuple::Tuple,
};

#[derive(Clone, Debug)]
pub struct World {
    light_sources: Vec<PointLight>,
    objects: Vec<Sphere>,
}

impl World {
    pub fn new() -> Self {
        World {
            light_sources: vec![],
            objects: vec![],
        }
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

    pub fn shade_hit(&self, comps: Computations) -> Color {
        // TODO: try multiple light sources.  It will slow things down though
        comps.object.material.lighting(
            self.light_sources[0],
            comps.point,
            comps.eyev,
            comps.normalv,
            false,
        )
    }
}

impl Default for World {
    fn default() -> Self {
        let mut sphere1 = Sphere::default();
        sphere1.set_material(
            Material::default()
                .color(Color::new(0.8, 1.0, 0.6))
                .diffuse(0.7)
                .specular(0.2),
        );
        let mut sphere2 = Sphere::default();
        sphere2.set_transform(Transform::scaling(0.5, 0.5, 0.5));
        Self {
            light_sources: vec![PointLight::new(
                Tuple::point(-10.0, 10.0, -10.0),
                Color::new(1.0, 1.0, 1.0),
            )],
            objects: vec![sphere1, sphere2],
        }
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

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
        s1.set_material(
            Material::default()
                .color(Color::new(0.8, 1.0, 0.6))
                .diffuse(0.7)
                .specular(0.2),
        );
        let mut s2 = Sphere::default();
        s2.set_transform(Transform::scaling(0.5, 0.5, 0.5));

        let w = World::default();

        assert_eq!(w.light_sources, vec![light]);
        assert_eq!(w.objects[0], s1);
        assert_eq!(w.objects[1], s2);
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
        let shape = w.objects[0];
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
        let shape = w.objects[1];
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
    fn color_whith_an_intersection_behind_ray() {
        let w = World::default();
        let mut outer = w.objects[0];
        outer.material.ambient = 1.0;
        let mut inner = w.objects[1];
        inner.material.ambient = 1.0;
        let new_world = World {
            objects: vec![outer, inner],
            ..World::default()
        };
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));

        let c = new_world.color_at(r);

        assert_eq!(c, inner.material.color);
    }
}
