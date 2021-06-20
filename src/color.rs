use std::ops::{Add, Mul, Sub};

use bevy::{math::Vec4, render::color};
use float_cmp::approx_eq;

#[derive(Clone, Copy, Debug)]
pub struct Color(color::Color);

pub const BLACK: Color = Color(color::Color::BLACK);
pub const WHITE: Color = Color(color::Color::WHITE);

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self(color::Color::rgb(red, green, blue))
    }

    pub fn red(self) -> f32 {
        self.0.r()
    }

    pub fn green(self) -> f32 {
        self.0.g()
    }

    pub fn blue(self) -> f32 {
        self.0.b()
    }
}

impl Default for Color {
    fn default() -> Self {
        BLACK
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        let epsilon = 0.001;
        approx_eq!(f32, self.0.r(), other.0.r(), epsilon = epsilon)
            && approx_eq!(f32, self.0.g(), other.0.g(), epsilon = epsilon)
            && approx_eq!(f32, self.0.b(), other.0.b(), epsilon = epsilon)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let self_vec = Vec4::from(self.0);
        let rhs_vec = Vec4::from(rhs.0);
        let new_vec = self_vec + rhs_vec;

        Self(color::Color::rgb(new_vec.x, new_vec.y, new_vec.z))
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let self_vec = Vec4::from(self.0);
        let rhs_vec = Vec4::from(rhs.0);
        let new_vec = self_vec - rhs_vec;

        Self(color::Color::rgb(new_vec.x, new_vec.y, new_vec.z))
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        hadamard_product(self, rhs)
    }
}

fn hadamard_product(c1: Color, c2: Color) -> Color {
    Color(color::Color::rgb(
        c1.0.r() * c2.0.r(),
        c1.0.g() * c2.0.g(),
        c1.0.b() * c2.0.b(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_rgb_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(c, Color::new(-0.5, 0.4, 1.7));
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
