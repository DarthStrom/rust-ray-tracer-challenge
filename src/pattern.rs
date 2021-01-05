use crate::{color::Color, tuple::Tuple};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StripePattern {
    a: Color,
    b: Color,
}

impl StripePattern {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn stripe_at(&self, point: Tuple) -> Color {
        if ((point.x % 2.0) + 2.0) % 2.0 < 1.0 {
            Color::white()
        } else {
            Color::black()
        }
    }
}

#[cfg(test)]
mod tests {
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
}
