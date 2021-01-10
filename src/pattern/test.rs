use crate::{color::Color, matrix::transform::Transform, tuple::Tuple};

use super::PatternTrait;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct TestPattern {
    pub transform: Transform,
}

impl TestPattern {
    fn transform(self, transform: Transform) -> Self {
        Self { transform }
    }
}

impl PatternTrait for TestPattern {
    fn pattern_at(&self, point: Tuple) -> Color {
        Color::new(point.x, point.y, point.z)
    }

    fn get_transform(&self) -> Transform {
        self.transform.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::{pattern::Patterns, shape::sphere::Sphere};

    use super::*;

    use float_cmp::ApproxEq;

    #[test]
    fn default_pattern_transformation() {
        let pattern = TestPattern::default();

        f_assert_eq!(pattern.transform, &Transform::default());
    }

    #[test]
    fn assigning_a_transformation() {
        let transform = Transform::translation(1.0, 2.0, 3.0);
        let pattern = TestPattern::default().transform(transform.clone());

        f_assert_eq!(pattern.transform, &transform);
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let shape = Sphere::default().transform(Transform::scaling(2.0, 2.0, 2.0));
        let pattern = Patterns::Test(TestPattern::default());

        let c = pattern
            .pattern_at_shape(&shape, Tuple::point(2.0, 3.0, 4.0))
            .unwrap();

        f_assert_eq!(c, &Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let shape = Sphere::default();
        let pattern =
            Patterns::Test(TestPattern::default().transform(Transform::scaling(2.0, 2.0, 2.0)));

        let c = pattern
            .pattern_at_shape(&shape, Tuple::point(2.0, 3.0, 4.0))
            .unwrap();

        f_assert_eq!(c, &Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let shape = Sphere::default().transform(Transform::scaling(2.0, 2.0, 2.0));
        let pattern =
            Patterns::Test(TestPattern::default().transform(Transform::translation(0.5, 1.0, 1.5)));

        let c = pattern
            .pattern_at_shape(&shape, Tuple::point(2.5, 3.0, 3.5))
            .unwrap();

        f_assert_eq!(c, &Color::new(0.75, 0.5, 0.25));
    }
}
