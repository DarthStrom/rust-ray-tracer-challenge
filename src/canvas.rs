use crate::color::Color;

#[derive(Debug)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = (0..width * height)
            .map(|_| Color::default())
            .collect::<Vec<_>>();
        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[x + y * self.width]
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.pixels[x + y * self.width] = color
        }
    }

    pub fn to_ppm(&self) -> String {
        format!(
            "P3\n{} {}\n255\n{}\n",
            self.width,
            self.height,
            self.pixels
                .chunks(self.width)
                .map(|row| {
                    row.iter()
                        .map(|pixel| {
                            (
                                color_u8(pixel.red()),
                                color_u8(pixel.green()),
                                color_u8(pixel.blue()),
                            )
                        })
                        .map(|(r, g, b)| format!("{} {} {}", r, g, b))
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .map(|s| split_long_ppm_line(&s))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

fn split_long_ppm_line(line: &str) -> String {
    if line.len() > 70 {
        let i = line
            .match_indices(" ")
            .filter(|(i, _)| i < &70)
            .max()
            .unwrap()
            .0;
        let mut result = line.to_string();
        result.remove(i);
        result.insert(i, '\n');
        result
    } else {
        line.to_string()
    }
}

fn color_u8(color: f64) -> u8 {
    if color >= 256.0 {
        255
    } else if color <= 0.0 {
        0
    } else {
        (256.0 * color) as u8
    }
}

#[cfg(test)]
mod tests {
    use float_cmp::{ApproxEq, F64Margin};
    use std::vec;

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

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);

        let ppm = c.to_ppm();

        assert_eq!(
            ppm.lines().take(3).collect::<Vec<_>>(),
            vec!["P3", "5 3", "255"]
        );
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = c.to_ppm();

        assert_eq!(
            ppm.lines().collect::<Vec<_>>()[3..].to_vec(),
            vec![
                "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0",
                "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255",
            ]
        );
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);

        for pixel in c.pixels.iter_mut() {
            *pixel = Color::new(1.0, 0.8, 0.6);
        }
        let ppm = c.to_ppm();

        assert_eq!(
            ppm.lines().collect::<Vec<_>>()[3..].to_vec(),
            vec![
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153",
                "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
                "153 255 204 153 255 204 153 255 204 153 255 204 153",
            ]
        )
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline() {
        let c = Canvas::new(5, 3);

        let ppm = c.to_ppm();

        assert!(ppm.ends_with("\n"));
    }
}
