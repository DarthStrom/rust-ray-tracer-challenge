use crate::{
    color::{self, Color},
    intersection::{Computations, Intersection, Intersections},
    lights::PointLight,
    materials::Material,
    ray::Ray,
    shapes::{sphere::Sphere, BoxShape, Shape, ShapeBuilder},
    transformations::Transform,
    tuple::Tuple,
    MARGIN,
};

#[derive(Debug)]
pub struct World {
    light_source: PointLight,
    objects: Vec<BoxShape>,
}

impl World {
    pub fn new(light: PointLight) -> Self {
        World {
            light_source: light,
            objects: vec![],
        }
    }

    pub fn light_source(self, light_source: PointLight) -> Self {
        Self {
            light_source,
            ..self
        }
    }

    pub fn object(self, object: Box<dyn Shape>) -> Self {
        let mut objects = self.objects;
        objects.push(object);

        Self { objects, ..self }
    }

    pub fn color_at(&self, ray: Ray, remaining: u32) -> Color {
        let intersections = self.intersect(ray);
        if let Some(hit) = intersections.clone().hit() {
            let comps = hit.prepare_computations(ray, intersections);
            self.shade_hit(comps, remaining)
        } else {
            color::BLACK
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
        let v = self.light_source.position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(point, direction);
        let intersections = self.intersect(r);

        if let Some(h) = intersections.hit() {
            h.t < distance
        } else {
            false
        }
    }

    pub fn shade_hit(&self, comps: Computations, remaining: u32) -> Color {
        // TODO: try multiple light sources.  It will slow things down though
        let shadowed = self.is_shadowed(comps.over_point);

        let material = comps.object.material();
        let surface = material.lighting(
            comps.object,
            self.light_source,
            comps.over_point,
            comps.eyev,
            comps.normalv,
            shadowed,
        );

        let reflected = self.reflected_color(comps, remaining);
        let refracted = self.refracted_color(comps, remaining);

        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = comps.schlick();
            surface + reflected * reflectance + refracted * (1.0 - reflectance)
        } else {
            surface + reflected + refracted
        }
    }

    pub fn reflected_color(&self, comps: Computations, remaining: u32) -> Color {
        if comps.object.material().reflective < MARGIN.epsilon || remaining == 0 {
            color::BLACK
        } else {
            let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
            let color = self.color_at(reflect_ray, remaining - 1);

            color * comps.object.material().reflective
        }
    }

    pub fn refracted_color(&self, comps: Computations, remaining: u32) -> Color {
        if comps.object.material().transparency <= MARGIN.epsilon || remaining == 0 {
            color::BLACK
        } else {
            let n_ratio = comps.n1 / comps.n2;
            let cos_i = comps.eyev.dot(comps.normalv);
            let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));

            if sin2_t > 1.0 {
                color::BLACK
            } else {
                let cos_t = (1.0 - sin2_t).sqrt();
                let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
                let refract_ray = Ray::new(comps.under_point, direction);
                self.color_at(refract_ray, remaining - 1) * comps.object.material().transparency
            }
        }
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
            light_source: PointLight::new(
                Tuple::point(-10.0, 10.0, -10.0),
                Color::new(1.0, 1.0, 1.0),
            ),
            objects: vec![Box::new(sphere1), Box::new(sphere2)],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::SQRT_2;

    use float_cmp::approx_eq;

    use crate::{
        color,
        patterns::TestPattern,
        shapes::{plane::Plane, ShapeBuilder},
        test::sqrt_n_over_n,
    };

    use super::*;

    #[test]
    fn creating_a_world() {
        let w = World::new(PointLight::default());

        assert!(w.objects.is_empty());
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

        assert_eq!(w.light_source, light);
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

        let comps = i.prepare_computations(r, Intersections::new(vec![i]));
        let c = w.shade_hit(comps, 3);

        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let w = World {
            light_source: PointLight::new(Tuple::point(0.0, 0.25, 0.0), Color::new(1.0, 1.0, 1.0)),
            ..World::default()
        };
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = w.objects[1].as_ref();
        let i = Intersection::new(0.5, shape);

        let comps = i.prepare_computations(r, Intersections::new(vec![i]));
        let c = w.shade_hit(comps, 3);

        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }

    #[test]
    fn color_when_ray_misses() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));

        let c = w.color_at(r, 3);

        assert_eq!(c, color::BLACK);
    }

    #[test]
    fn color_when_a_ray_hits() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));

        let c = w.color_at(r, 3);

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
        let w = World::new(PointLight::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ))
        .object(Box::new(outer))
        .object(Box::new(inner.clone()));

        let r = Ray::new(Tuple::point(0.0, 0.0, 0.75), Tuple::vector(0.0, 0.0, -1.0));

        let c = w.color_at(r, 3);

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
        let w = World::new(
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

        let comps = i.prepare_computations(r, Intersections::new(vec![i]));
        let c = w.shade_hit(comps, 3);

        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let sphere1 = Sphere::default().with_material(
            Material::default()
                .color(Color::new(0.8, 1.0, 0.6))
                .diffuse(0.7)
                .specular(0.2),
        );
        let sphere2 = Sphere::default()
            .with_transform(Transform::scaling(0.5, 0.5, 0.5))
            .with_material(Material::default().ambient(1.0));
        let w = World::new(PointLight::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ))
        .object(Box::new(sphere1))
        .object(Box::new(sphere2.clone()));
        let r = Ray::default()
            .origin(0.0, 0.0, 0.0)
            .direction(0.0, 0.0, 1.0);
        let i = Intersection::new(1.0, &sphere2);

        let comps = i.prepare_computations(r, Intersections::new(vec![i]));
        let color = w.reflected_color(comps, 3);

        assert_eq!(color, color::BLACK);
    }

    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let w = World::default().object(Box::new(
            Plane::default()
                .with_material(Material::default().reflective(0.5))
                .with_transform(Transform::translation(0.0, -1.0, 0.0)),
        ));
        let shape = w.objects[2].as_ref();
        let r = Ray::default().origin(0.0, 0.0, -3.0).direction(
            0.0,
            -sqrt_n_over_n(2),
            sqrt_n_over_n(2),
        );
        let i = Intersection::new(SQRT_2, shape);

        let comps = i.prepare_computations(r, Intersections::new(vec![i]));
        let color = w.reflected_color(comps, 10);

        println!("{:?}", color);
        assert_eq!(color, Color::new(0.19032, 0.2379, 0.14274));
    }

    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut w = World::default();
        let shape = Plane::default()
            .with_material(Material::default().reflective(0.5))
            .with_transform(Transform::translation(0.0, -1.0, 0.0));
        w.objects.push(shape.box_clone());
        let r = Ray::default()
            .origin(0.0, 0.0, -3.0)
            .direction(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0);
        let i = Intersection::new(SQRT_2, &shape);

        let comps = i.prepare_computations(r, Intersections::new(vec![i]));
        let color = w.shade_hit(comps, 5);

        println!("{:?}", color);
        assert_eq!(color, Color::new(0.87677, 0.92436, 0.82918));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let lower = Plane::default()
            .with_material(Material::default().reflective(1.0))
            .with_transform(Transform::translation(0.0, -1.0, 0.0));
        let upper = Plane::default()
            .with_material(Material::default().reflective(1.0))
            .with_transform(Transform::translation(0.0, 1.0, 0.0));
        let w = World::new(PointLight::new(Tuple::point(0.0, 0.0, 0.0), color::WHITE))
            .object(Box::new(lower))
            .object(Box::new(upper));
        let r = Ray::default()
            .origin(0.0, 0.0, 0.0)
            .direction(0.0, 1.0, 0.0);

        let _ = w.color_at(r, 3);
    }

    // TODO: maybe revisit infinite recursion, so far seems to be fine maybe because of compiler optimizations?

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = World::default();
        let shape = w.objects[0].as_ref();
        let r = Ray::default()
            .origin(0.0, 0.0, -5.0)
            .direction(0.0, 0.0, 1.0);
        let xs = Intersections::new(vec![
            Intersection::new(4.0, shape),
            Intersection::new(6.0, shape),
        ]);
        let xs_copy = xs.clone();

        let comps = xs[0].prepare_computations(r, xs_copy);
        let c = w.refracted_color(comps, 5);

        assert_eq!(c, color::BLACK);
    }

    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let w = World::new(PointLight::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ))
        .object(Box::new(
            Sphere::default().with_material(
                Material::default()
                    .color(Color::new(0.8, 1.0, 0.6))
                    .diffuse(0.7)
                    .specular(0.2)
                    .transparency(1.0)
                    .refractive_index(1.5),
            ),
        ))
        .object(Box::new(
            Sphere::default().with_transform(Transform::scaling(0.5, 0.5, 0.5)),
        ));
        let r = Ray::default()
            .origin(0.0, 0.0, -5.0)
            .direction(0.0, 0.0, 1.0);
        let shape = w.objects[0].as_ref();
        let xs = Intersections::new(vec![
            Intersection::new(4.0, shape),
            Intersection::new(6.0, shape),
        ]);
        let xs_copy = xs.clone();

        let comps = xs[0].prepare_computations(r, xs_copy);
        let c = w.refracted_color(comps, 0);

        assert_eq!(c, color::BLACK);
    }

    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let w = World::new(PointLight::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ))
        .object(Box::new(
            Sphere::default().with_material(
                Material::default()
                    .color(Color::new(0.8, 1.0, 0.6))
                    .diffuse(0.7)
                    .specular(0.2)
                    .transparency(1.0)
                    .refractive_index(1.5),
            ),
        ))
        .object(Box::new(
            Sphere::default().with_transform(Transform::scaling(0.5, 0.5, 0.5)),
        ));
        let r = Ray::default()
            .origin(0.0, 0.0, sqrt_n_over_n(2))
            .direction(0.0, 1.0, 0.0);
        let shape = w.objects[0].as_ref();
        let xs = Intersections::new(vec![
            Intersection::new(-sqrt_n_over_n(2), shape),
            Intersection::new(sqrt_n_over_n(2), shape),
        ]);
        let xs_copy = xs.clone();

        let comps = xs[1].prepare_computations(r, xs_copy);
        let c = w.refracted_color(comps, 5);

        assert_eq!(c, color::BLACK);
    }

    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let w = World::new(PointLight::new(
            Tuple::point(-10.0, 10.0, -10.0),
            Color::new(1.0, 1.0, 1.0),
        ))
        .object(Box::new(
            Sphere::default().with_material(
                Material::default()
                    .color(Color::new(0.8, 1.0, 0.6))
                    .diffuse(0.7)
                    .specular(0.2)
                    .ambient(1.0)
                    .pattern(Box::new(TestPattern::default())),
            ),
        ))
        .object(Box::new(Sphere::default().with_material(
            Material::default().transparency(1.0).refractive_index(1.5),
        )));
        let r = Ray::default()
            .origin(0.0, 0.0, 0.1)
            .direction(0.0, 1.0, 0.0);
        let a = w.objects[0].as_ref();
        let b = w.objects[1].as_ref();
        let xs = Intersections::new(vec![
            Intersection::new(-0.9899, a),
            Intersection::new(-0.4899, b),
            Intersection::new(0.4899, b),
            Intersection::new(0.9899, a),
        ]);
        let xs_copy = xs.clone();

        let comps = xs[2].prepare_computations(r, xs_copy);
        let color = w.refracted_color(comps, 5);

        assert_eq!(color, Color::new(0.0, 0.99888, 0.04725));
    }

    #[test]
    fn shade_hit_with_a_transparent_material() {
        let w = World::default()
            .object(Box::new(
                Plane::default()
                    .with_transform(Transform::translation(0.0, -1.0, 0.0))
                    .with_material(Material::default().transparency(0.5).refractive_index(1.5)),
            ))
            .object(Box::new(
                Sphere::default()
                    .with_material(
                        Material::default()
                            .color(Color::new(1.0, 0.0, 0.0))
                            .ambient(0.5),
                    )
                    .with_transform(Transform::translation(0.0, -3.5, -0.5)),
            ));
        let r = Ray::default().origin(0.0, 0.0, -3.0).direction(
            0.0,
            -sqrt_n_over_n(2),
            sqrt_n_over_n(2),
        );
        let floor = w.objects[0].as_ref();
        let xs = Intersections::new(vec![Intersection::new(SQRT_2, floor)]);
        let xs_copy = xs.clone();

        let comps = xs[0].prepare_computations(r, xs_copy);
        let color = w.shade_hit(comps, 5);

        assert_eq!(color, Color::new(0.93642, 0.686432, 0.68642))
    }

    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let w = World::default()
            .object(Box::new(
                Plane::default()
                    .with_transform(Transform::translation(0.0, -1.0, 0.0))
                    .with_material(
                        Material::default()
                            .reflective(0.5)
                            .transparency(0.5)
                            .refractive_index(1.5),
                    ),
            ))
            .object(Box::new(
                Sphere::default()
                    .with_material(
                        Material::default()
                            .color(Color::new(1.0, 0.0, 0.0))
                            .ambient(0.5),
                    )
                    .with_transform(Transform::translation(0.0, -3.5, -0.5)),
            ));
        let floor = w.objects[2].as_ref();
        let xs = Intersections::new(vec![Intersection::new(SQRT_2, floor)]);
        let xs_copy = xs.clone();
        let r = Ray::default().origin(0.0, 0.0, -3.0).direction(
            0.0,
            -sqrt_n_over_n(2),
            sqrt_n_over_n(2),
        );

        let comps = xs[0].prepare_computations(r, xs_copy);
        let color = w.shade_hit(comps, 5);

        assert_eq!(color, Color::new(0.93391, 0.69643, 0.69243));
    }
}
