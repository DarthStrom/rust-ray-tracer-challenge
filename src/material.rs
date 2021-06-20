use crate::{
    color::Color,
    light::PointLight,
    pattern::Patterns,
    shape::Object,
    tuple::{dot, Tuple},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub reflective: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Patterns>,
}

impl Material {
    pub fn color(self, color: Color) -> Self {
        Self { color, ..self }
    }

    pub fn ambient(self, ambient: f64) -> Self {
        Self { ambient, ..self }
    }

    pub fn diffuse(self, diffuse: f64) -> Self {
        Self { diffuse, ..self }
    }

    pub fn reflective(self, reflective: f64) -> Self {
        Self { reflective, ..self }
    }

    pub fn specular(self, specular: f64) -> Self {
        Self { specular, ..self }
    }

    pub fn shininess(self, shininess: f64) -> Self {
        Self { shininess, ..self }
    }

    pub fn lighting(
        &self,
        object: &Object,
        light: PointLight,
        point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool,
    ) -> Color {
        let color = if let Some(pattern) = &self.pattern {
            pattern
                .pattern_at_shape(object, point)
                .unwrap_or(self.color)
        } else {
            self.color
        };
        let effective_color = color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = dot(lightv, normalv);
        let (diffuse, specular) = if in_shadow || light_behind_surface(light_dot_normal) {
            (Color::default(), Color::default())
        } else {
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = dot(reflectv, eyev);
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
            reflective: 0.0,
            specular: 0.9,
            shininess: 200.0,
            pattern: None,
        }
    }
}

fn light_behind_surface(light_dot_normal: f64) -> bool {
    light_dot_normal < 0.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        light::PointLight, pattern::striped, pattern::Patterns, shape::sphere::Sphere,
        test::sqrt_n_over_n, tuple::Tuple, MARGIN,
    };
    use float_cmp::ApproxEq;
    use striped::Striped;

    #[test]
    fn default() {
        let m = Material::default();

        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        f_assert_eq!(m.ambient, 0.1);
        f_assert_eq!(m.diffuse, 0.9);
        f_assert_eq!(m.specular, 0.9);
        f_assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let (m, position) = shared_setup();
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let object = Object::Sphere(Sphere::default());

        let result = m.lighting(&object, light, position, eyev, normalv, false);

        assert!(result.approx_eq(&Color::new(1.9, 1.9, 1.9), MARGIN));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_offset_45_deg() {
        let (m, position) = shared_setup();
        let eyev = Tuple::vector(0.0, sqrt_n_over_n(2), -sqrt_n_over_n(2));
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let object = Object::Sphere(Sphere::default());

        let result = m.lighting(&object, light, position, eyev, normalv, false);

        assert!(result.approx_eq(&Color::new(1.0, 1.0, 1.0), MARGIN));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_deg() {
        let (m, position) = shared_setup();
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let object = Object::Sphere(Sphere::default());

        let result = m.lighting(&object, light, position, eyev, normalv, false);

        assert!(result.approx_eq(&Color::new(0.7364, 0.7364, 0.7364), MARGIN));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection() {
        let (m, position) = shared_setup();
        let eyev = Tuple::vector(0.0, -sqrt_n_over_n(2), -sqrt_n_over_n(2));
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let object = Object::Sphere(Sphere::default());

        let result = m.lighting(&object, light, position, eyev, normalv, false);

        assert!(result.approx_eq(&Color::new(1.6364, 1.6364, 1.6364), MARGIN));
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let (m, position) = shared_setup();
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::new(Tuple::point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let object = Object::Sphere(Sphere::default());

        let result = m.lighting(&object, light, position, eyev, normalv, false);

        assert!(result.approx_eq(&Color::new(0.1, 0.1, 0.1), MARGIN));
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
        let object = Object::Sphere(Sphere::default());

        let result = m.lighting(&object, light, position, eyev, normalv, in_shadow);

        f_assert_eq!(result, &Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let (mut m, _) = shared_setup();
        m.pattern = Some(Patterns::Striped(Striped::new(
            Color::white(),
            Color::black(),
        )));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = PointLight::default()
            .position(0.0, 0.0, -10.0)
            .intensity(1.0, 1.0, 1.0);
        let object = Object::Sphere(Sphere::default());

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

        f_assert_eq!(c1, &Color::white());
        f_assert_eq!(c2, &Color::black());
    }

    #[test]
    fn reflectivity_for_the_default_material() {
        let (m, _) = shared_setup();

        f_assert_eq!(m.reflective, 0.0);
    }

    fn shared_setup() -> (Material, Tuple) {
        (Material::default(), Tuple::point(0.0, 0.0, 0.0))
    }
}
