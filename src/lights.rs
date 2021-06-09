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

    pub fn position(self, x: f32, y: f32, z: f32) -> Self {
        Self {
            position: Tuple::point(x, y, z),
            ..self
        }
    }

    pub fn intensity(self, r: f32, g: f32, b: f32) -> Self {
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
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Tuple::point(0.0, 0.0, 0.0);

        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
