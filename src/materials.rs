use crate::{
    color::{self, Color},
    lights::PointLight,
    patterns::BoxPattern,
    shapes::Shape,
    tuple::Tuple,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f32,
    pub diffuse: f32,
    pub reflective: f32,
    pub specular: f32,
    pub shininess: f32,
    pub transparency: f32,
    pub refractive_index: f32,
    pub pattern: Option<BoxPattern>,
}

impl Material {
    pub fn color(self, color: Color) -> Self {
        Self { color, ..self }
    }

    pub fn ambient(self, ambient: f32) -> Self {
        Self { ambient, ..self }
    }

    pub fn diffuse(self, diffuse: f32) -> Self {
        Self { diffuse, ..self }
    }

    pub fn reflective(self, reflective: f32) -> Self {
        Self { reflective, ..self }
    }

    pub fn specular(self, specular: f32) -> Self {
        Self { specular, ..self }
    }

    pub fn shininess(self, shininess: f32) -> Self {
        Self { shininess, ..self }
    }

    pub fn transparency(self, transparency: f32) -> Self {
        Self {
            transparency,
            ..self
        }
    }

    pub fn refractive_index(self, refractive_index: f32) -> Self {
        Self {
            refractive_index,
            ..self
        }
    }

    pub fn pattern(self, pattern: BoxPattern) -> Self {
        Self {
            pattern: Some(pattern),
            ..self
        }
    }

    pub fn lighting(
        &self,
        object: &dyn Shape,
        light: PointLight,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        let color = if let Some(pattern) = &self.pattern {
            pattern.pattern_at_shape(object, point)
        } else {
            self.color
        };
        let effective_color = color * light.intensity;
        let lightv = (light.position - point).normalize();

        let ambient = effective_color * self.ambient;
        if in_shadow {
            return ambient;
        }

        let light_dot_normal = lightv.dot(normalv);
        let (diffuse, specular) = if light_behind_surface(light_dot_normal) {
            (color::BLACK, color::BLACK)
        } else {
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);
            (
                effective_color * self.diffuse * light_dot_normal,
                if reflect_dot_eye <= 0.0 {
                    color::BLACK
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
            reflective: 0.0,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }
}

fn light_behind_surface(light_dot_normal: f32) -> bool {
    light_dot_normal < 0.0
}

#[cfg(test)]
mod tests {
    use crate::{patterns::striped::Striped, shapes::sphere::Sphere, test::*};

    use super::*;

    use float_cmp::approx_eq;

    #[test]
    fn default() {
        let m = Material::default();

        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert!(approx_eq!(f32, m.ambient, 0.1));
        assert!(approx_eq!(f32, m.diffuse, 0.9));
        assert!(approx_eq!(f32, m.specular, 0.9));
        assert!(approx_eq!(f32, m.shininess, 200.0));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let (m, position) = shared_setup();
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let object = Sphere::default();

        let result = m.lighting(&object, light, position, eyev, normalv, false);

        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_offset_45_deg() {
        let (m, position) = shared_setup();
        let eyev = Tuple::vector(0.0, sqrt_n_over_n(2), -sqrt_n_over_n(2));
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let object = Sphere::default();

        let result = m.lighting(&object, light, position, eyev, normalv, false);

        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_deg() {
        let (m, position) = shared_setup();
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let object = Sphere::default();

        let result = m.lighting(&object, light, position, eyev, normalv, false);

        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection() {
        let (m, position) = shared_setup();
        let eyev = Tuple::vector(0.0, -sqrt_n_over_n(2), -sqrt_n_over_n(2));
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let object = Sphere::default();

        let result = m.lighting(&object, light, position, eyev, normalv, false);

        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let (m, position) = shared_setup();
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let object = Sphere::default();

        let result = m.lighting(&object, light, position, eyev, normalv, false);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let (m, position) = shared_setup();
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::default()
            .position(0.0, 0.0, -10.0)
            .intensity(1.0, 1.0, 1.0);
        let in_shadow = true;
        let object = Sphere::default();

        let result = m.lighting(&object, light, position, eyev, normalv, in_shadow);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let (mut m, _) = shared_setup();
        m.pattern = Some(Box::new(Striped::new(color::WHITE, color::BLACK)));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::default()
            .position(0.0, 0.0, -10.0)
            .intensity(1.0, 1.0, 1.0);
        let object = Sphere::default();

        let c1 = m.lighting(
            &object,
            light,
            Tuple::point(0.9, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );
        let c2 = m.lighting(
            &object,
            light,
            Tuple::point(1.1, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );

        assert_eq!(c1, color::WHITE);
        assert_eq!(c2, color::BLACK);
    }

    #[test]
    fn reflectivity_for_the_default_material() {
        let (m, _) = shared_setup();

        assert!(approx_eq!(f32, m.reflective, 0.0));
    }

    fn shared_setup() -> (Material, Tuple) {
        (Material::default(), Tuple::point(0.0, 0.0, 0.0))
    }

    #[test]
    fn transparency_and_refractive_index_for_the_default_material() {
        let m = Material::default();

        assert!(approx_eq!(f32, m.transparency, 0.0));
        assert!(approx_eq!(f32, m.refractive_index, 1.0));
    }
}
