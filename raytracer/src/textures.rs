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

use crate::colors::Color;

pub trait Texture {
    fn color_at(&self, u: f64, v: f64) -> Color;
}

pub struct PlainColorTexture {
    pub color: Color,
}

impl Texture for PlainColorTexture {
    fn color_at(&self, _: f64, _: f64) -> Color {
        self.color.clone()
    }
}

pub struct CheckedPattern {
    pub primary_color: Color,
    pub secondary_color: Color,
    pub count: f64,
}

impl Default for CheckedPattern {
    fn default() -> Self {
        CheckedPattern {
            primary_color: Color::new(0.95, 0.95, 0.95),
            secondary_color: Color::new(0.05, 0.05, 0.05),
            count: 10.0,
        }
    }
}

impl Texture for CheckedPattern {
    fn color_at(&self, u: f64, v: f64) -> Color {
        assert!(u >= -std::f64::EPSILON && u <= 1.0 + std::f64::EPSILON);
        assert!(v >= -std::f64::EPSILON && v <= 1.0 + std::f64::EPSILON);
        let selection = ((u * self.count).floor() + (v * self.count).floor()) as u64 % 2;
        match selection {
            0 => self.primary_color.clone(),
            1 => self.secondary_color.clone(),
            _ => unreachable!(),
        }
    }
}
