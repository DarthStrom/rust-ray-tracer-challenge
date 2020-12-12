use crate::tuple::Tuple;
use float_cmp::ApproxEq;
use num_traits::{Float, FromPrimitive};

pub struct Color<F> {
    tuple: Tuple<F>,
}

impl<F: Float + FromPrimitive> Color<F> {
    pub fn new(red: F, green: F, blue: F) -> Self {
        Self {
            tuple: Tuple::point(red, green, blue),
        }
    }

    pub fn red(&self) -> F {
        self.tuple.x
    }

    pub fn green(&self) -> F {
        self.tuple.y
    }

    pub fn blue(&self) -> F {
        self.tuple.z
    }
}

impl<'a, M: Copy + Default, F: Copy + ApproxEq<Margin = M>> ApproxEq for &'a Color<F> {
    type Margin = M;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.tuple.approx_eq(&other.tuple, margin)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::F64Margin;

    use super::*;

    #[test]
    fn colors_are_rgb_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert!(c.red().approx_eq(-0.5, F64Margin::default()));
        assert!(c.green().approx_eq(0.4, F64Margin::default()));
        assert!(c.blue().approx_eq(1.7, F64Margin::default()));
    }
}
