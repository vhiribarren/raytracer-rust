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
use crate::lights::AnyLightObject;
use crate::parser;
use crate::primitives::{Ray, Shape};
use crate::result::{RaytracerError, Result};
use crate::textures::{Texture, TextureEffects, PlainColorTexture};
use crate::vector::Vec3;
use crate::UnitInterval;
use serde::{Deserialize};
use std::str::FromStr;
use std::fmt::Debug;
use std::fmt::{Formatter, Error};
use crate::parser::DescriptionConfig;

#[derive(Deserialize)]
#[serde(default)]
#[serde(from="DescriptionConfig")]
pub struct SceneConfiguration {
    pub world_texture: Box<dyn Texture>,
    pub world_refractive_index: f64,
    pub ambient_light: Option<Color>,
    pub maximum_light_recursion: u8,
}

impl Debug for SceneConfiguration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        Ok(())
    }
}

impl Default for SceneConfiguration {
    fn default() -> Self {
        SceneConfiguration {
            world_texture: Box::new(PlainColorTexture::new(Color::BLACK)),
            world_refractive_index: 1.0,
            ambient_light: Some(Color::new(0.2, 0.2, 0.2)),
            maximum_light_recursion: 2,
        }
    }
}

pub struct SceneObject {
    pub texture: Box<dyn Texture>,
    pub shape: Box<dyn Shape>,
    pub effects: TextureEffects,
}

impl SceneObject {
    pub fn color_at(&self, point: Vec3) -> Color {
        let (u, v) = self.shape.surface_mapping_at(point).unwrap();
        self.texture.color_at(u, v)
    }

    pub fn check_collision(&self, ray: &Ray) -> Option<Vec3> {
        self.shape.check_collision(ray)
    }

    pub fn normal_at(&self, point: Vec3) -> Option<Vec3> {
        self.shape.normal_at(point)
    }

    pub fn effects(&self) -> &TextureEffects {
        &self.effects
    }
}

pub trait RayEmitter: Send + Sync {
    fn width(&self) -> f64;
    fn height(&self) -> f64;
    fn size_ratio(&self) -> f64 {
        self.width() / self.height()
    }
    fn generate_ray(&self, canvas_x: UnitInterval, canvas_y: UnitInterval) -> Ray;
}

pub struct Scene {
    pub camera: Box<dyn RayEmitter>,
    pub lights: Vec<Box<dyn AnyLightObject>>,
    pub objects: Vec<SceneObject>,
    pub config: SceneConfiguration,
}

impl FromStr for Scene {
    type Err = RaytracerError;

    fn from_str(scene_str: &str) -> Result<Scene> {
        parser::parse_scene_description(scene_str)
    }
}
