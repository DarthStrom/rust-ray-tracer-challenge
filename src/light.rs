use crate::{color::Color, tuple::Tuple};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }

    pub fn position(self, x: f64, y: f64, z: f64) -> Self {
        Self {
            position: Tuple::point(x, y, z),
            ..self
        }
    }

    pub fn intensity(self, r: f64, g: f64, b: f64) -> Self {
        Self {
            intensity: Color::new(r, g, b),
            ..self
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Tuple::point(0.0, 0.0, 0.0);

        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
