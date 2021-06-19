use crate::{
    color::{self, Color},
    shapes::Shape,
    transformations::Transform,
    tuple::Tuple,
};

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

    pub fn with_transform(self, transform: Transform) -> Self {
        Self { transform, ..self }
    }

    pub fn pattern_at(&self, point: Tuple) -> Color {
        if ((point.x() % 2.0) + 2.0) % 2.0 < 1.0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn pattern_at_object(&self, object: &dyn Shape, world_point: Tuple) -> Color {
        let object_point = object.transform().inverse() * world_point;
        let pattern_point = self.transform.inverse() * object_point;

        self.pattern_at(pattern_point)
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

#[cfg(test)]
mod tests {
    use crate::{color, shapes::ShapeBuilder, sphere::Sphere, transformations::Transform};

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

        let c = pattern.pattern_at_object(&object, Tuple::point(1.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Sphere::default();
        let pattern = Striped::new(color::WHITE, color::BLACK)
            .with_transform(Transform::scaling(2.0, 2.0, 2.0));

        let c = pattern.pattern_at_object(&object, Tuple::point(1.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }

    #[test]
    fn stripes_with_both_an_object_and_a_pattern_transformation() {
        let object = Sphere::default().with_transform(Transform::scaling(2.0, 2.0, 2.0));
        let pattern = Striped::new(color::WHITE, color::BLACK)
            .with_transform(Transform::translation(0.5, 0.0, 0.0));

        let c = pattern.pattern_at_object(&object, Tuple::point(2.5, 0.0, 0.0));

        assert_eq!(c, color::WHITE);
    }
}
