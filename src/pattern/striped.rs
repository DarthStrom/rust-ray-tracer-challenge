use crate::{color::Color, matrix::transform::Transform, tuple::Tuple};

use super::PatternTrait;

#[derive(Clone, Debug, PartialEq)]
pub struct Striped {
    a: Color,
    b: Color,
    transform: Transform,
}

impl Striped {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Transform::default(),
        }
    }

    fn transform(self, transform: Transform) -> Self {
        Self { transform, ..self }
    }
}

impl PatternTrait for Striped {
    fn pattern_at(&self, point: Tuple) -> Color {
        if ((point.x % 2.0) + 2.0) % 2.0 < 1.0 {
            Color::white()
        } else {
            Color::black()
        }
    }

    fn get_transform(&self) -> Transform {
        self.transform.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::pattern::Patterns;

    use super::*;

    use float_cmp::ApproxEq;

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = Striped::new(Color::white(), Color::black());

        f_assert_eq!(pattern.a, &Color::white());
        f_assert_eq!(pattern.b, &Color::black());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Patterns::Striped(Striped::new(Color::white(), Color::black()));

        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 1.0, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 2.0, 0.0)),
            &Color::white()
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Patterns::Striped(Striped::new(Color::white(), Color::black()));

        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 1.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 2.0)),
            &Color::white()
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Patterns::Striped(Striped::new(Color::white(), Color::black()));

        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.9, 0.0, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(1.0, 0.0, 0.0)),
            &Color::black()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(-0.1, 0.0, 0.0)),
            &Color::black()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(-1.0, 0.0, 0.0)),
            &Color::black()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(-1.1, 0.0, 0.0)),
            &Color::white()
        );
    }
}
