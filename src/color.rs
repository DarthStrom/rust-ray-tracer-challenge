use crate::tuple::Tuple;
use float_cmp::ApproxEq;
use num_traits::{Float, FromPrimitive};
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug, Default)]
pub struct Color<F> {
    tuple: Tuple<F>,
}

fn hadamard_product<F: Float + FromPrimitive>(c1: &Color<F>, c2: &Color<F>) -> Color<F> {
    Color::new(
        c1.red() * c2.red(),
        c1.green() * c2.green(),
        c1.blue() * c2.blue(),
    )
}

impl<F: Float + FromPrimitive> Color<F> {
    pub fn new(red: F, green: F, blue: F) -> Self {
        Self {
            tuple: Tuple::vector(red, green, blue),
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

impl<F: Add<Output = F>> Add for Color<F> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            tuple: self.tuple + other.tuple,
        }
    }
}

impl<F: Sub<Output = F>> Sub for Color<F> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            tuple: self.tuple - other.tuple,
        }
    }
}

impl<F: Clone + Copy + Mul<Output = F>> Mul<F> for Color<F> {
    type Output = Self;

    fn mul(self, rhs: F) -> Self::Output {
        Self {
            tuple: self.tuple * rhs,
        }
    }
}

impl<F: Float + FromPrimitive> Mul for Color<F> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        hadamard_product(&self, &rhs)
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::F64Margin;

    use super::*;

    #[test]
    fn colors_are_rgb_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert!(c.approx_eq(&Color::new(-0.5, 0.4, 1.7), F64Margin::default()));
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert!((c1 + c2).approx_eq(&Color::new(1.6, 0.7, 1.0), F64Margin::default()));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert!((c1 - c2).approx_eq(&Color::new(0.2, 0.5, 0.5), F64Margin::default()));
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert!((c * 2.0).approx_eq(&Color::new(0.4, 0.6, 0.8), F64Margin::default()));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert!((c1 * c2).approx_eq(&Color::new(0.9, 0.2, 0.04), F64Margin::default()));
    }
}
