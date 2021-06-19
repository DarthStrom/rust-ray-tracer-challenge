use std::any::Any;

use crate::{
    color::{self, Color},
    transformations::Transform,
    tuple::Tuple,
};

use super::{BoxPattern, Pattern, PatternBuilder};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Gradient {
    pub a: Color,
    pub b: Color,
    pub transform: Transform,
}

impl Gradient {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Transform::default(),
        }
    }
}

impl PatternBuilder for Gradient {
    fn with_transform(self, transform: Transform) -> Self {
        Self { transform, ..self }
    }
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            a: color::WHITE,
            b: color::BLACK,
            transform: Transform::default(),
        }
    }
}

impl Pattern for Gradient {
    fn box_clone(&self) -> BoxPattern {
        Box::new(*self)
    }

    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn pattern_at(&self, point: Tuple) -> Color {
        let distance = self.b - self.a;
        let fraction = point.x() - point.x().floor();

        self.a + distance * fraction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = Gradient::new(color::WHITE, color::BLACK);

        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
