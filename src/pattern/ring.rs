use crate::{color::Color, matrix::transform::Transform, tuple::Tuple};

use super::PatternTrait;

#[derive(Clone, Debug, PartialEq)]
pub struct Ring {
    a: Color,
    b: Color,
    transform: Transform,
}

impl Ring {
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

impl PatternTrait for Ring {
    fn pattern_at(&self, point: Tuple) -> Color {
        if (point.x * point.x + point.z * point.z).sqrt().floor() as u32 % 2 == 0 {
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
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = Patterns::Ring(Ring::new(Color::white(), Color::black()));

        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)),
            &Color::white()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(1.0, 0.0, 0.0)),
            &Color::black()
        );
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 1.0)),
            &Color::black()
        );
        // 0.708 = slightly more than sqrt(2) / 2
        f_assert_eq!(
            pattern.pattern_at(Tuple::point(0.708, 0.0, 0.708)),
            &Color::black()
        );
    }
}
