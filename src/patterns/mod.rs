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
    fn pattern_at(&self, point: Tuple) -> Color;
    fn pattern_at_object(&self, object: &dyn Shape, world_point: Tuple) -> Color;
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
