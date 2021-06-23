pub mod checkered;
pub mod gradient;
pub mod ring;
pub mod striped;

use std::{any::Any, fmt::Debug};

use crate::{color::Color, shapes::Shape, transformations::Transform, tuple::Tuple};

pub trait PatternBuilder {
    fn with_transform(self, transform: Transform) -> Self;
}

pub trait Pattern: Any + Debug {
    fn box_clone(&self) -> BoxPattern;
    fn box_eq(&self, other: &dyn Any) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn transform(&self) -> &Transform;
    fn pattern_at(&self, point: Tuple) -> Color;
    fn pattern_at_shape(&self, object: &dyn Shape, world_point: Tuple) -> Color {
        let object_point = object.transform().inverse() * world_point;
        let pattern_point = self.transform().inverse() * object_point;

        self.pattern_at(pattern_point)
    }
}

pub type BoxPattern = Box<dyn Pattern>;

impl Clone for BoxPattern {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

impl PartialEq for BoxPattern {
    fn eq(&self, other: &Self) -> bool {
        self.box_eq(other.as_any())
    }
}

#[cfg(test)]
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct TestPattern {
    transform: Transform,
}

#[cfg(test)]
impl Pattern for TestPattern {
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
        Color::new(point.x(), point.y(), point.z())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shapes::{sphere::Sphere, ShapeBuilder},
        transformations::IDENTITY,
    };

    use super::*;

    #[test]
    fn default_test_pattern() {
        let pattern = TestPattern::default();

        assert_eq!(pattern.transform(), &IDENTITY);
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let shape = Sphere::default().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let pattern = TestPattern::default();

        let c = pattern.pattern_at_shape(&shape, Tuple::point(2.0, 3.0, 4.0));

        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let shape = Sphere::default();
        let pattern = TestPattern {
            transform: Transform::scaling(2.0, 2.0, 2.0),
        };

        let c = pattern.pattern_at_shape(&shape, Tuple::point(2.0, 3.0, 4.0));

        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let shape = Sphere::default().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let pattern = TestPattern {
            transform: Transform::translation(0.5, 1.0, 1.5),
        };

        let c = pattern.pattern_at_shape(&shape, Tuple::point(2.5, 3.0, 3.5));

        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
