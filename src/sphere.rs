use crate::{
    intersection::{Intersection, Intersections},
    ray::Ray,
    transform::Transform,
    tuple::Tuple,
};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Sphere {
    pub center: Tuple,
    pub radius: f32,
    pub transform: Transform,
}

impl Sphere {
    pub fn intersect(self, ray: Ray) -> Intersections {
        let ray = ray.transform(self.transform.inverse());
        let discriminant = discriminant(ray);

        if discriminant < 0.0 {
            Intersections::new(vec![])
        } else {
            let a = a(ray);
            let b = b(ray);
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            let i1 = Intersection::new(t1, self);
            let i2 = Intersection::new(t2, self);
            Intersections::new(vec![i1, i2])
        }
    }

    pub fn set_transform(&mut self, transform: Transform) {
        self.transform = transform
    }
}

fn a(ray: Ray) -> f32 {
    ray.direction.dot(ray.direction)
}

fn b(ray: Ray) -> f32 {
    let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);
    2.0 * ray.direction.dot(sphere_to_ray)
}

fn c(ray: Ray) -> f32 {
    let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);
    sphere_to_ray.dot(sphere_to_ray) - 1.0
}

fn discriminant(ray: Ray) -> f32 {
    b(ray).powf(2.0) - 4.0 * a(ray) * c(ray)
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::transform::IDENTITY;

    use super::*;

    #[test]
    fn ray_intersects_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_eq!(f32, xs[0].t, 4.0));
        assert!(approx_eq!(f32, xs[1].t, 6.0));
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_eq!(f32, xs[0].t, 5.0));
        assert!(approx_eq!(f32, xs[1].t, 5.0));
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originating_inside_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_eq!(f32, xs[0].t, -1.0));
        assert!(approx_eq!(f32, xs[1].t, 1.0));
    }

    #[test]
    fn sphere_behind_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::default();

        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_eq!(f32, xs[0].t, -6.0));
        assert!(approx_eq!(f32, xs[1].t, -4.0));
    }

    #[test]
    fn a_spheres_default_transformation() {
        let s = Sphere::default();

        assert_eq!(s.transform, IDENTITY);
    }

    #[test]
    fn changin_a_spheres_transformation() {
        let mut s = Sphere::default();
        let t = Transform::translation(2.0, 3.0, 4.0);

        s.set_transform(t);

        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Sphere::default();

        s.set_transform(Transform::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert!(approx_eq!(f32, xs[0].t, 3.0));
        assert!(approx_eq!(f32, xs[1].t, 7.0));
    }
}
