use crate::{
    color::{self, Color},
    tuple::Tuple,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Striped {
    pub a: Color,
    pub b: Color,
}

impl Striped {
    pub fn new(a: Color, b: Color) -> Self {
        Self { a, b }
    }

    pub fn pattern_at(&self, point: Tuple) -> Color {
        if ((point.x() % 2.0) + 2.0) % 2.0 < 1.0 {
            self.a
        } else {
            self.b
        }
    }
}

impl Default for Striped {
    fn default() -> Self {
        Self {
            a: color::WHITE,
            b: color::BLACK,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::color;

    use super::*;

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = Striped::new(color::WHITE, color::BLACK);

        assert_eq!(pattern.a, color::WHITE);
        assert_eq!(pattern.b, color::BLACK);
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Striped::new(color::WHITE, color::BLACK);

        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 1.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 2.0, 0.0)),
            color::WHITE
        );
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Striped::new(color::WHITE, color::BLACK);

        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 1.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 2.0)),
            color::WHITE
        );
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Striped::new(color::WHITE, color::BLACK);

        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.9, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(1.0, 0.0, 0.0)),
            color::BLACK
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(-0.1, 0.0, 0.0)),
            color::BLACK
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(-1.0, 0.0, 0.0)),
            color::BLACK
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(-1.1, 0.0, 0.0)),
            color::WHITE
        );
    }
}
