use std::ops::Mul;

#[derive(Debug, Default)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub struct Texture {
    pub color: Color,
}

impl Mul<f64> for &Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: rhs * self.red,
            green: rhs * self.green,
            blue: rhs * self.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: rhs * self.red,
            green: rhs * self.green,
            blue: rhs * self.blue,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            red: self * rhs.red,
            green: self * rhs.green,
            blue: self * rhs.blue,
        }
    }
}

impl Mul<&Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        Color {
            red: self * rhs.red,
            green: self * rhs.green,
            blue: self * rhs.blue,
        }
    }
}
