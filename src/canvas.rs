use float_cmp::ApproxEq;
use std::vec;

use crate::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color<f64>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = (0..(width * height) - 1)
            .map(|_| Color::default())
            .collect::<Vec<_>>();
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color<f64> {
        self.pixels[x + y * self.width]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color<f64>) {
        if x >= self.width || y >= self.height {
            panic!("trying to write outside of canvas");
        }

        self.pixels[x + y * self.width] = color
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::F64Margin;

    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for pixel in c.pixels {
            assert!(pixel.approx_eq(&Color::new(0.0, 0.0, 0.0), F64Margin::default()));
        }
    }

    #[test]
    fn writing_pixels() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);

        assert!(c.pixel_at(2, 3).approx_eq(&red, F64Margin::default()));
    }
}
