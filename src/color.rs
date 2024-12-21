use crate::Compare;

// Color class to represent a color; similar structure to Tuple
// All the components should be between 0 and 1
// No such constraint has been added because during processing of algorithms, color value can be <0 or >1,
// if we clip at every stage, then the final image would be too dark or too light
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: f64,   // red component of the color
    pub green: f64, // green component of the color
    pub blue: f64,  // blue component of the color
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn red() -> Color {
        Color::new(1.0, 0.0, 0.0)
    }

    pub fn green() -> Color {
        Color::new(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Color {
        Color::new(0.0, 0.0, 1.0)
    }
}

// For color + color
impl std::ops::Add for Color {
    type Output = Self;

    fn add(self, other: Color) -> Self {
        Color::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

// For color - color
impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(self, other: Color) -> Self {
        Color::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        self.red.eq(other.red) && self.green.eq(other.green) && self.blue.eq(other.blue)
    }
}

// For color * a
impl std::ops::Mul<f64> for Color {
    type Output = Self;

    fn mul(self, a: f64) -> Self {
        Color::new(self.red * a, self.green * a, self.blue * a)
    }
}

// For a * color
impl std::ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, color: Color) -> Color {
        Color::new(self * color.red, self * color.green, self * color.blue)
    }
}

// For color1 * color2
impl std::ops::Mul<Color> for Color {
    type Output = Self;

    fn mul(self, color: Self) -> Self {
        Color::new(
            self.red * color.red,
            self.green * color.green,
            self.blue * color.blue,
        )
    }
}

#[cfg(test)]
mod color_tests {
    use super::*;

    #[test]
    fn color_validity() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.red, -0.5);
        assert_eq!(c.green, 0.4);
        assert_eq!(c.blue, 1.7);
    }

    #[test]
    fn color_color_add() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let _c = Color::new(1.6, 0.7, 1.0);
        assert_eq!(c1 + c2, _c);
    }

    #[test]
    fn color_color_sub() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let _c = Color::new(0.2, 0.5, 0.5);
        assert_eq!(c1 - c2, _c);
    }

    #[test]
    fn scalar_color_multiply() {
        let c = Color::new(0.2, 0.3, 0.4);
        let _c = Color::new(0.4, 0.6, 0.8);
        assert_eq!(2.0 * c, _c);
    }

    #[test]
    fn color_scalar_multiply() {
        let c = Color::new(0.2, 0.3, 0.4);
        let _c = Color::new(0.4, 0.6, 0.8);
        assert_eq!(c * 2.0, _c);
    }

    #[test]
    fn color_color_multiply() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let _c = Color::new(0.9, 0.2, 0.04);
        assert_eq!(c1 * c2, _c);
    }
}
