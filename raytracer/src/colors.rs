/*
MIT License

Copyright (c) 2019 Vincent Hiribarren

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::utils::unit_interval_clamp;
use crate::UnitInterval;

#[derive(Clone, Debug, Default)]
pub struct Color {
    red: UnitInterval,
    green: UnitInterval,
    blue: UnitInterval,
}

impl Color {
    pub fn new(red: UnitInterval, green: UnitInterval, blue: UnitInterval) -> Self {
        Color {
            red: unit_interval_clamp(red),
            green: unit_interval_clamp(green),
            blue: unit_interval_clamp(blue),
        }
    }

    pub fn red(&self) -> UnitInterval {
        self.red
    }

    pub fn blue(&self) -> UnitInterval {
        self.blue
    }

    pub fn green(&self) -> UnitInterval {
        self.green
    }

    pub const WHITE: Self = Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };
    pub const BLACK: Self = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };
    pub const RED: Self = Color {
        red: 1.0,
        green: 0.0,
        blue: 0.0,
    };
    pub const GREEN: Self = Color {
        red: 0.0,
        green: 1.0,
        blue: 0.0,
    };
    pub const BLUE: Self = Color {
        red: 0.0,
        green: 0.0,
        blue: 1.0,
    };
    pub const YELLOW: Self = Color {
        red: 1.0,
        green: 1.0,
        blue: 0.0,
    };
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(
            unit_interval_clamp(self.red + rhs.red),
            unit_interval_clamp(self.green + rhs.green),
            unit_interval_clamp(self.blue + rhs.blue),
        )
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.red = unit_interval_clamp(self.red + rhs.red);
        self.green = unit_interval_clamp(self.green + rhs.green);
        self.blue = unit_interval_clamp(self.blue + rhs.blue);
    }
}

impl std::ops::Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

impl std::ops::Mul<UnitInterval> for &Color {
    type Output = Color;

    fn mul(self, rhs: UnitInterval) -> Self::Output {
        Color {
            red: unit_interval_clamp(rhs * self.red),
            green: unit_interval_clamp(rhs * self.green),
            blue: unit_interval_clamp(rhs * self.blue),
        }
    }
}

impl std::ops::Mul<UnitInterval> for Color {
    type Output = Color;

    fn mul(self, rhs: UnitInterval) -> Self::Output {
        Color {
            red: unit_interval_clamp(rhs * self.red),
            green: unit_interval_clamp(rhs * self.green),
            blue: unit_interval_clamp(rhs * self.blue),
        }
    }
}

impl std::ops::Mul<Color> for UnitInterval {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            red: unit_interval_clamp(self * rhs.red),
            green: unit_interval_clamp(self * rhs.green),
            blue: unit_interval_clamp(self * rhs.blue),
        }
    }
}

impl std::ops::Mul<&Color> for UnitInterval {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        Color {
            red: unit_interval_clamp(self * rhs.red),
            green: unit_interval_clamp(self * rhs.green),
            blue: unit_interval_clamp(self * rhs.blue),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::EPSILON;

    #[test]
    fn new_color_with_high_value_is_clamped() {
        let color = Color::new(10.0, 10.0, 10.0);
        assert!(color.red <= 1.0+EPSILON);
        assert!(color.green <= 1.0+EPSILON);
        assert!(color.blue <= 1.0+EPSILON);
    }

    #[test]
    fn new_color_with_low_value_is_clamped() {
        let color = Color::new(-10.0, -10.0, -10.0);
        assert!(color.red >= -EPSILON);
        assert!(color.green >= -EPSILON);
        assert!(color.blue >= -EPSILON);
    }

    #[test]
    fn add_with_high_color_is_clamped() {
        let color_1 = Color::new(1.0, 1.0, 1.0);
        let color_2 = Color::new(1.0, 1.0, 1.0);
        let result = color_1 + color_2;
        assert!(result.red <= 1.0+EPSILON);
        assert!(result.green <= 1.0+EPSILON);
        assert!(result.blue <= 1.0+EPSILON);
    }

    #[test]
    fn mul_with_high_constant_is_clamped() {
        let color = Color::new(1.0, 1.0, 1.0);
        let result =  10.0 * color;
        assert!(result.red <= 1.0+EPSILON);
        assert!(result.green <= 1.0+EPSILON);
        assert!(result.blue <= 1.0+EPSILON);
    }

}