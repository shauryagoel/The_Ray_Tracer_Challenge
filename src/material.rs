use crate::Light;
use crate::{Color, Tuple};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,   // 0.0 to 1.0
    pub diffuse: f64,   // 0.0 to 1.0
    pub specular: f64,  // 0.0 to 1.0
    pub shininess: f64, // usally between 10.0 (very large highlight) to 200.0 (very small highlight)
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64) -> Self {
        Self {
            color,
            ambient,
            diffuse,
            specular,
            shininess,
        }
    }

    // Phong reflection model for shading
    pub fn lighting(
        &self,
        light: Light,
        hit_point: Tuple,
        eyev: Tuple,
        normalv: Tuple,
        in_shadow: bool, // whether the point is in the shadow
    ) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - hit_point).normalize();
        let ambient = effective_color * self.ambient;
        let mut diffuse = Color::black();
        let mut specular = Color::black();

        let light_dot_normal = lightv.dot(&normalv);
        // Don't compute diffuse and specular when the point is on shadow
        if (light_dot_normal >= 0.0) && !in_shadow {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = (-lightv).reflect(&normalv);
            let reflect_dot_eye = reflectv.dot(&eyev);
            if reflect_dot_eye > 0.0 {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new(Color::new(1.0, 1.0, 1.0), 0.1, 0.9, 0.9, 200.0)
    }
}

#[cfg(test)]
mod material_tests {
    use super::*;
    use crate::{point, vector};
    use std::f64::consts::FRAC_1_SQRT_2;

    #[test]
    fn material_default() {
        let m = Material::default();
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = false;
        let result = m.lighting(light, position, eyev, normalv, in_shadow);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_45_degrees() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = false;
        let result = m.lighting(light, position, eyev, normalv, in_shadow);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_45_degrees() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = false;
        let result = m.lighting(light, position, eyev, normalv, in_shadow);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_with_eye_in_path_of_reflection_vector() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, -FRAC_1_SQRT_2, -FRAC_1_SQRT_2);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = false;
        let result = m.lighting(light, position, eyev, normalv, in_shadow);
        assert_eq!(result, Color::new(1.636396, 1.636396, 1.636396));
    }

    #[test]
    fn lighting_with_light_behind_surface() {
        let m = Material::default();
        let position = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::new(point(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = false;
        let result = m.lighting(light, position, eyev, normalv, in_shadow);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let m = Material::default();
        let light = Light::new(point(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let hit_point = point(0.0, 0.0, 0.0);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let in_shadow = true;
        let result = m.lighting(light, hit_point, eyev, normalv, in_shadow);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
