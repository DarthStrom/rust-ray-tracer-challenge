use crate::{color::Color, matrix::transform::Transform, tuple::Tuple};

use super::PatternTrait;

#[derive(Clone, Debug, PartialEq)]
pub struct Gradient {
    a: Color,
    b: Color,
    transform: Transform,
}

impl Gradient {
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

impl PatternTrait for Gradient {
    fn pattern_at(&self, point: Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x - point.x.floor();

        self.a + distance * fraction
    }

    fn get_transform(&self) -> Transform {
        self.transform.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::{color::Color, pattern::Patterns, tuple::Tuple};

    use float_cmp::ApproxEq;

    use super::*;

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = Patterns::Gradient(Gradient::new(Color::white(), Color::black()));

        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.25, 0.0, 0.0)),
            &Color::new(0.75, 0.75, 0.75)
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.5, 0.0, 0.0)),
            &Color::new(0.5, 0.5, 0.5)
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.75, 0.0, 0.0)),
            &Color::new(0.25, 0.25, 0.25)
        );
    }
}
