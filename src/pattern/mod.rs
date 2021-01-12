use checkered::Checkered;
use gradient::Gradient;
use ring::Ring;
use striped::Striped;
#[cfg(test)]
use test::TestPattern;

use crate::{color::Color, matrix::transform::Transform, shape::Shape, tuple::Tuple};

pub mod checkered;
pub mod gradient;
pub mod ring;
pub mod striped;
#[cfg(test)]
mod test;

pub trait PatternTrait {
    fn pattern_at(&self, point: Tuple) -> Color;
    fn get_transform(&self) -> Transform;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Patterns {
    Checkered(Checkered),
    Gradient(Gradient),
    Ring(Ring),
    Striped(Striped),
    #[cfg(test)]
    Test(TestPattern),
}

impl Patterns {
    pub fn pattern_at_shape(&self, shape: &dyn Shape, world_point: Tuple) -> Result<Color, String> {
        let shape_point = shape.get_transform().inverse()? * world_point;
        let pattern_point = match self {
            Patterns::Checkered(checkered) => checkered.get_transform(),
            Patterns::Gradient(gradient) => gradient.get_transform(),
            Patterns::Ring(ring) => ring.get_transform(),
            Patterns::Striped(striped) => striped.get_transform(),
            #[cfg(test)]
            Patterns::Test(test) => test.get_transform(),
        }
        .inverse()?
            * shape_point;

        Ok(self.pattern_at(pattern_point))
    }
}

impl PatternTrait for Patterns {
    fn pattern_at(&self, point: Tuple) -> Color {
        match self {
            Patterns::Checkered(checkered) => checkered.pattern_at(point),
            Patterns::Gradient(gradient) => gradient.pattern_at(point),
            Patterns::Ring(ring) => ring.pattern_at(point),
            Patterns::Striped(striped) => striped.pattern_at(point),
            #[cfg(test)]
            Patterns::Test(test) => test.pattern_at(point),
        }
    }

    fn get_transform(&self) -> Transform {
        match self {
            Patterns::Checkered(checkered) => checkered.get_transform(),
            Patterns::Gradient(gradient) => gradient.get_transform(),
            Patterns::Ring(ring) => ring.get_transform(),
            Patterns::Striped(striped) => striped.get_transform(),
            #[cfg(test)]
            Patterns::Test(test) => test.get_transform(),
        }
    }
}
