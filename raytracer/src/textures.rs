/*
MIT License

Copyright (c) 2019, 2020 Vincent Hiribarren

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
use crate::utils::{f64_gt, f64_lt};
use crate::UnitInterval;
use serde::Deserialize;

pub trait Texture: Sync + Send {
    fn color_at(&self, u: f64, v: f64) -> Color;
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct PlainColorTexture {
    pub color: Color,
}

impl Default for PlainColorTexture {
    fn default() -> Self {
        PlainColorTexture {
            color: Color::WHITE,
        }
    }
}

impl Texture for PlainColorTexture {
    fn color_at(&self, _: f64, _: f64) -> Color {
        self.color.clone()
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
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
        assert!(f64_gt(u, 0.0) && f64_lt(u, 1.0));
        assert!(f64_gt(v, 0.0) && f64_lt(v, 1.0));
        let selection = ((u * self.count).floor() + (v * self.count).floor()) as u64 % 2;
        match selection {
            0 => self.primary_color.clone(),
            1 => self.secondary_color.clone(),
            _ => unreachable!(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct TextureEffects {
    #[serde(default)]
    pub phong: Option<Phong>,
    #[serde(default)]
    pub transparency: Option<Transparency>,
    #[serde(default)]
    pub mirror: Option<Mirror>,
}

impl Default for TextureEffects {
    fn default() -> Self {
        TextureEffects {
            phong: None,
            transparency: None,
            mirror: None,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Mirror {
    pub coeff: UnitInterval,
}

impl Default for Mirror {
    fn default() -> Self {
        Mirror { coeff: 0.8 }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Transparency {
    pub refractive_index: f64,
    pub alpha: UnitInterval,
}

impl Default for Transparency {
    fn default() -> Self {
        Transparency {
            refractive_index: 1.0,
            alpha: 0.5,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Phong {
    pub size: u32,
    pub lum_coeff: UnitInterval,
}

impl Default for Phong {
    fn default() -> Self {
        Phong {
            size: 50,
            lum_coeff: 0.5,
        }
    }
}
