use crate::{
    color::Color,
    light::PointLight,
    tuple::{dot, Tuple},
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn lighting(&self, light: PointLight, point: Tuple, eyev: Tuple, normalv: Tuple) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = dot(&lightv, &normalv);
        let (diffuse, specular) = if light_behind_surface(light_dot_normal) {
            (Color::default(), Color::default())
        } else {
            let reflectv = (-lightv).reflect(&normalv);
            let reflect_dot_eye = dot(&reflectv, &eyev);
            (
                effective_color * self.diffuse * light_dot_normal,
                if reflect_dot_eye <= 0.0 {
                    Color::default()
                } else {
                    let factor = reflect_dot_eye.powf(self.shininess);
                    light.intensity * self.specular * factor
                },
            )
        };

        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

fn light_behind_surface(light_dot_normal: f64) -> bool {
    light_dot_normal < 0.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{light::PointLight, test::sqrt_n_over_n, tuple::Tuple, MARGIN};
    use float_cmp::ApproxEq;

    #[test]
    fn default() {
        let m = Material::default();

        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let (m, position) = shared_setup();
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eyev, normalv);

        assert!(result.approx_eq(&Color::new(1.9, 1.9, 1.9), MARGIN));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_offset_45_deg() {
        let (m, position) = shared_setup();
        let eyev = Tuple::vector(0.0, sqrt_n_over_n(2), -sqrt_n_over_n(2));
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eyev, normalv);

        assert!(result.approx_eq(&Color::new(1.0, 1.0, 1.0), MARGIN));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_deg() {
        let (m, position) = shared_setup();
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eyev, normalv);

        assert!(result.approx_eq(&Color::new(0.7364, 0.7364, 0.7364), MARGIN));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection() {
        let (m, position) = shared_setup();
        let eyev = Tuple::vector(0.0, -sqrt_n_over_n(2), -sqrt_n_over_n(2));
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eyev, normalv);

        assert!(result.approx_eq(&Color::new(1.6364, 1.6364, 1.6364), MARGIN));
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let (m, position) = shared_setup();
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));

        let result = m.lighting(light, position, eyev, normalv);

        assert!(result.approx_eq(&Color::new(0.1, 0.1, 0.1), MARGIN));
    }

    fn shared_setup() -> (Material, Tuple) {
        (Material::default(), Tuple::point(0.0, 0.0, 0.0))
    }
}
