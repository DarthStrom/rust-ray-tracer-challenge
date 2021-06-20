use std::any::Any;

use crate::{
    color::{self, Color},
    transformations::Transform,
    tuple::Tuple,
};

use super::{BoxPattern, Pattern, PatternBuilder};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Striped {
    pub a: Color,
    pub b: Color,
    pub transform: Transform,
}

impl Striped {
    pub fn new(a: Color, b: Color) -> Self {
        Self {
            a,
            b,
            transform: Transform::default(),
        }
    }
}

impl PatternBuilder for Striped {
    fn with_transform(self, transform: Transform) -> Self {
        Self { transform, ..self }
    }
}

impl Default for Striped {
    fn default() -> Self {
        Self {
            a: color::WHITE,
            b: color::BLACK,
            transform: Transform::default(),
        }
    }
}

impl Pattern for Striped {
    fn box_clone(&self) -> BoxPattern {
        Box::new(*self)
    }

    fn box_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn transform(&self) -> &Transform {
        &self.transform
    }

    fn pattern_at(&self, point: Tuple) -> Color {
        if ((point.x() % 2.0) + 2.0) % 2.0 < 1.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color,
        shapes::{sphere::Sphere, ShapeBuilder},
        transformations::Transform,
    };

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

    #[test]
    fn stripes_with_an_object_transformation() {
        let object = Sphere::default().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let pattern = Striped::new(color::WHITE, color::BLACK);

        let c = pattern.pattern_at_shape(&object, Tuple::point(1.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Sphere::default();
        let pattern = Striped::new(color::WHITE, color::BLACK)
            .with_transform(Transform::scaling(2.0, 2.0, 2.0));

        let c = pattern.pattern_at_shape(&object, Tuple::point(1.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let object = Sphere::default().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let pattern = Striped::new(color::WHITE, color::BLACK)
            .with_transform(Transform::translation(0.5, 0.0, 0.0));

        let c = pattern.pattern_at_shape(&object, Tuple::point(2.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }
}
