use std::any::Any;

use crate::{
    color::{self, Color},
    transformations::Transform,
    tuple::Tuple,
};

use super::{BoxPattern, Pattern, PatternBuilder};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ring {
    pub a: Color,
    pub b: Color,
    pub transform: Transform,
}

impl Ring {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Transform::default(),
        }
    }
}

impl PatternBuilder for Ring {
    fn with_transform(self, transform: Transform) -> Self {
        Self { transform, ..self }
    }
}

impl Default for Ring {
    fn default() -> Self {
        Self {
            a: color::WHITE,
            b: color::BLACK,
            transform: Transform::default(),
        }
    }
}

impl Pattern for Ring {
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
        if (point.x() * point.x() + point.z() * point.z())
            .sqrt()
            .floor() as u32
            % 2
            == 0
        {
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
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = Ring::new(color::WHITE, color::BLACK);

        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)),
            color::WHITE
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(1.0, 0.0, 0.0)),
            color::BLACK
        );
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.0, 0.0, 1.0)),
            color::BLACK
        );
        // 0.708 = slightly more than sqrt(2) / 2
        assert_eq!(
            pattern.pattern_at(Tuple::point(0.708, 0.0, 0.708)),
            color::BLACK
        );
    }
}
