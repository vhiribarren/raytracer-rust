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
use crate::primitives::Ray;
use crate::vector::Vec3;
use serde::Deserialize;
use std::f64::consts::PI;

pub trait AnyLightObject: Send + Sync {
    fn source(&self) -> Vec3;
    fn color_for_ray(&self, ray: Ray) -> Color;
}

#[derive(Debug, Deserialize)]
pub struct LightPoint {
    pub source: Vec3,
    pub color: Color,
}

impl LightPoint {
    pub fn new(source: Vec3) -> Self {
        LightPoint {
            source,
            color: Color::WHITE,
        }
    }

    pub fn with_color(source: Vec3, color: Color) -> Self {
        LightPoint { source, color }
    }
}

impl AnyLightObject for LightPoint {
    fn source(&self) -> Vec3 {
        self.source
    }

    fn color_for_ray(&self, _ray: Ray) -> Color {
        self.color.clone()
    }
}

pub struct AmbientLight {
    pub power: f64,
}

#[derive(Debug, Deserialize)]
pub struct SpotLight {
    pub source: Vec3,
    pub color: Color,
    pub direction: Vec3,
    pub inner_angle: f64,
    pub outer_angle: f64,
    _use_constructor: (),
}

impl SpotLight {
    pub fn new(
        source: Vec3,
        direction: Vec3,
        inner_angle_degree: f64,
        outer_angle_degree: f64,
    ) -> Self {
        SpotLight {
            source,
            direction: direction.normalize(),
            color: Color::WHITE,
            inner_angle: inner_angle_degree * 2.0 * PI / 360.0,
            outer_angle: outer_angle_degree * 2.0 * PI / 360.0,
            _use_constructor: (),
        }
    }

    pub fn with_color(
        source: Vec3,
        direction: Vec3,
        inner_angle_degree: f64,
        outer_angle_degree: f64,
        color: Color,
    ) -> Self {
        SpotLight {
            color,
            ..SpotLight::new(source, direction, inner_angle_degree, outer_angle_degree)
        }
    }
}

impl AnyLightObject for SpotLight {
    fn source(&self) -> Vec3 {
        self.source
    }

    fn color_for_ray(&self, ray: Ray) -> Color {
        let angle = self.direction.dot_product(-ray.direction).acos();
        if angle <= self.inner_angle {
            self.color.clone()
        } else if angle >= self.outer_angle {
            Color::BLACK.clone()
        } else {
            let luminosity = 1.0 - (angle - self.inner_angle) / (self.outer_angle - self.inner_angle);
            luminosity * self.color.clone()
        }
    }
}
