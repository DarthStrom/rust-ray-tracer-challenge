use std::any::Any;

use crate::{
    color::{self, Color},
    transformations::Transform,
    tuple::Tuple,
};

use super::{BoxPattern, Pattern, PatternBuilder};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Checkered {
    pub a: Color,
    pub b: Color,
    pub transform: Transform,
}

impl Checkered {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Transform::default(),
        }
    }
}

impl PatternBuilder for Checkered {
    fn with_transform(self, transform: Transform) -> Self {
        Self { transform, ..self }
    }
}

impl Default for Checkered {
    fn default() -> Self {
        Self {
            a: color::WHITE,
            b: color::BLACK,
            transform: Transform::default(),
        }
    }
}

impl Pattern for Checkered {
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
        if (point.x().floor() + point.y().floor() + point.z().floor()) as u32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Checkered::new(color::WHITE, color::BLACK);

        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.99, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(1.01, 0.0, 0.0)),
            color::BLACK
        );
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Checkered::new(color::WHITE, color::BLACK);

        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.99, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 1.01, 0.0)),
            color::BLACK
        );
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Checkered::new(color::WHITE, color::BLACK);

        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.99)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 1.01)),
            color::BLACK
        );
    }
}
