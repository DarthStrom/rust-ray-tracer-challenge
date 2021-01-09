use crate::{
    color::Color,
    matrix::transform::Transform,
    shape::{Object, Shape},
    tuple::Tuple,
};

#[derive(Clone, Debug, PartialEq)]
pub struct StripePattern {
    a: Color,
    b: Color,
    transform: Transform,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Transform::default(),
        }
    }

    pub fn transform(self, transform: Transform) -> Self {
        Self { transform, ..self }
    }

    pub fn stripe_at(&self, point: Tuple) -> Color {
        if ((point.x % 2.0) + 2.0) % 2.0 < 1.0 {
            Color::white()
        } else {
            Color::black()
        }
    }

    pub fn stripe_at_object(&self, object: &Object, world_point: Tuple) -> Result<Color, String> {
        let object_point = object.transform().inverse()? * world_point;
        let pattern_point = self.transform.inverse()? * object_point;

        Ok(self.stripe_at(pattern_point))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        matrix::transform::Transform,
        shape::{sphere::Sphere, Object},
    };

    use super::*;

    use float_cmp::ApproxEq;

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = StripePattern::new(Color::white(), Color::black());

        f_assert_eq!(pattern.a, &Color::white());
        f_assert_eq!(pattern.b, &Color::black());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = StripePattern::new(Color::white(), Color::black());

        f_assert_eq!(
            pattern.stripe_at(Tuple::point(0.0, 0.0, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.stripe_at(Tuple::point(0.0, 1.0, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.stripe_at(Tuple::point(0.0, 2.0, 0.0)),
            &Color::white()
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = StripePattern::new(Color::white(), Color::black());

        f_assert_eq!(
            pattern.stripe_at(Tuple::point(0.0, 0.0, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.stripe_at(Tuple::point(0.0, 0.0, 1.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.stripe_at(Tuple::point(0.0, 0.0, 2.0)),
            &Color::white()
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = StripePattern::new(Color::white(), Color::black());

        f_assert_eq!(
            pattern.stripe_at(Tuple::point(0.0, 0.0, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.stripe_at(Tuple::point(0.9, 0.0, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.stripe_at(Tuple::point(1.0, 0.0, 0.0)),
            &Color::black()
        );
        f_assert_eq!(
            pattern.stripe_at(Tuple::point(-0.1, 0.0, 0.0)),
            &Color::black()
        );
        f_assert_eq!(
            pattern.stripe_at(Tuple::point(-1.0, 0.0, 0.0)),
            &Color::black()
        );
        f_assert_eq!(
            pattern.stripe_at(Tuple::point(-1.1, 0.0, 0.0)),
            &Color::white()
        );
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let object = Object::Sphere(Sphere::default().transform(Transform::scaling(2.0, 2.0, 2.0)));
        let pattern = StripePattern::new(Color::white(), Color::black());

        let c = pattern
            .stripe_at_object(&object, Tuple::point(1.5, 0.0, 0.0))
            .unwrap();

        f_assert_eq!(c, &Color::white());
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Object::Sphere(Sphere::default());
        let pattern = StripePattern::new(Color::white(), Color::black())
            .transform(Transform::scaling(2.0, 2.0, 2.0));

        let c = pattern
            .stripe_at_object(&object, Tuple::point(1.5, 0.0, 0.0))
            .unwrap();

        println!("{:?}", c);
        f_assert_eq!(c, &Color::white());
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let object = Object::Sphere(Sphere::default().transform(Transform::scaling(2.0, 2.0, 2.0)));
        let pattern = StripePattern::new(Color::white(), Color::black())
            .transform(Transform::translation(0.5, 0.0, 0.0));

        let c = pattern
            .stripe_at_object(&object, Tuple::point(2.5, 0.0, 0.0))
            .unwrap();

        f_assert_eq!(c, &Color::white());
    }
}
