use crate::tuple::Tuple;
use float_cmp::{ApproxEq, F64Margin};
use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Color {
    tuple: Tuple,
}

fn hadamard_product(c1: Color, c2: Color) -> Color {
    Color::new(
        c1.red() * c2.red(),
        c1.green() * c2.green(),
        c1.blue() * c2.blue(),
    )
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self {
            tuple: Tuple::vector(red, green, blue),
        }
    }

    pub fn red(&self) -> f64 {
        self.tuple.x
    }

    pub fn green(&self) -> f64 {
        self.tuple.y
    }

    pub fn blue(&self) -> f64 {
        self.tuple.z
    }
}

impl<'a> ApproxEq for &'a Color {
    type Margin = F64Margin;

    fn approx_eq<T: Into<Self::Margin>>(self, other: Self, margin: T) -> bool {
        let margin = margin.into();
        self.tuple.approx_eq(&other.tuple, margin)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            tuple: self.tuple + other.tuple,
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            tuple: self.tuple - other.tuple,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            tuple: self.tuple * rhs,
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        hadamard_product(self, rhs)
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
