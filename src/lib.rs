#[derive(Debug, PartialEq)]
pub struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: u8,
}

impl Tuple {
    fn new(x: f32, y: f32, z: f32, w: u8) -> Self {
        Self { x, y, z, w }
    }

    fn point(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, y, z, 1)
    }

    fn vector(x: f32, y: f32, z: f32) -> Self {
        Self::new(x, y, z, 0)
    }

    fn is_point(&self) -> bool {
        self.w == 1
    }

    fn is_vector(&self) -> bool {
        self.w == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w_1_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1);

        assert_eq!(
            a,
            Tuple {
                x: 4.3,
                y: -4.2,
                z: 3.1,
                w: 1
            }
        );
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_with_w_0_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0);

        assert_eq!(
            a,
            Tuple {
                x: 4.3,
                y: -4.2,
                z: 3.1,
                w: 0
            }
        );
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_creates_tuples_with_w_1() {
        let p = Tuple::point(4.0, -4.0, 3.0);

        assert_eq!(
            p,
            Tuple {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 1
            }
        );
    }

    #[test]
    fn vector_creates_tuples_with_w_1() {
        let p = Tuple::vector(4.0, -4.0, 3.0);

        assert_eq!(
            p,
            Tuple {
                x: 4.0,
                y: -4.0,
                z: 3.0,
                w: 0
            }
        );
    }
}
