use crate::{color::Color, matrix::transform::Transform, tuple::Tuple};

use super::PatternTrait;

#[derive(Clone, Debug, PartialEq)]
pub struct Checkered {
    a: Color,
    b: Color,
    transform: Transform,
}

impl Checkered {
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

impl PatternTrait for Checkered {
    fn pattern_at(&self, point: Tuple) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) as u32 % 2 == 0 {
            self.a
        } else {
            self.b
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
    fn checkers_should_repeat_in_x() {
        let pattern = Patterns::Checkered(Checkered::new(Color::white(), Color::black()));

        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.99, 0.0, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(1.01, 0.0, 0.0)),
            &Color::black()
        );
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Patterns::Checkered(Checkered::new(Color::white(), Color::black()));

        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.99, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 1.01, 0.0)),
            &Color::black()
        );
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Patterns::Checkered(Checkered::new(Color::white(), Color::black()));

        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.99)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 1.01)),
            &Color::black()
        );
    }
}
