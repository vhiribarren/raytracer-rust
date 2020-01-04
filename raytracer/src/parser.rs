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
SOFTWARE
*/

use crate::cameras::{OrthogonalCamera, PerspectiveCamera};
use crate::colors::Color;
use crate::lights::{AnyLightObject, LightPoint};
use crate::primitives::{InfinitePlan, Shape, Sphere, SquarePlan};
use crate::result::RaytracerError;
use crate::result::Result;
use crate::scene::{RayEmitter, Scene, SceneConfiguration, SceneObject};
use crate::textures::{CheckedPattern, PlainColorTexture, Texture, TextureEffects};
use crate::vector::Vec3;
use log::{info, trace};
use serde::Deserialize;
use std::str::FromStr;

pub(crate) fn parse_scene_description(scene_str: &str) -> Result<Scene> {
    let root_document = toml::from_str::<ModelRoot>(scene_str)
        .map_err(|e| RaytracerError::ParsingError(e.to_string()))?;
    trace!("Parsed scene description: {:#?}", root_document);
    if let Some(description) = root_document.description {
        info!("Generating scene for: {}", description);
    }
    let config = root_document.config;
    let camera = root_document.camera.into_ray_emitter();
    let lights = root_document
        .light
        .into_iter()
        .map(DescriptionLight::into_any_light_object)
        .collect();
    let objects = root_document
        .object
        .into_iter()
        .map(DescriptionObject::into_scene_object)
        .collect();

    Ok(Scene {
        camera,
        lights,
        objects,
        config,
    })
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModelRoot {
    description: Option<String>,
    #[serde(default)]
    config: SceneConfiguration,
    camera: DescriptionCamera,
    object: Vec<DescriptionObject>,
    light: Vec<DescriptionLight>,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub(crate) struct ModelVector([f64; 3]);

impl From<ModelVector> for Vec3 {
    fn from(model_vector: ModelVector) -> Self {
        Vec3::new(model_vector.0[0], model_vector.0[1], model_vector.0[2])
    }
}

impl From<ModelColor> for Color {
    fn from(model_color: ModelColor) -> Self {
        match model_color {
            ModelColor::ByString(value) => Color::from_str(&value).unwrap(),
            ModelColor::ByRGB(rgb) => Color::new(rgb[0], rgb[1], rgb[2]),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]

enum DescriptionLight {
    Point(LightPoint),
}

impl DescriptionLight {
    fn into_any_light_object(self) -> Box<dyn AnyLightObject> {
        match self {
            DescriptionLight::Point(val) => Box::new(val),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
enum DescriptionCamera {
    Perspective {
        screen_center: Vec3,
        look_at: Vec3,
        width: f64,
        height: f64,
        #[serde(default = "default_perspective_angle")]
        angle_degree: f64,
    },
    Orthogonal {
        eye: Vec3,
        look_at: Vec3,
        width: f64,
        height: f64,
    },
}

impl DescriptionCamera {
    fn into_ray_emitter(self) -> Box<dyn RayEmitter> {
        match self {
            DescriptionCamera::Perspective {
                screen_center,
                look_at,
                width,
                height,
                angle_degree,
            } => Box::new(PerspectiveCamera::new(
                screen_center,
                look_at,
                width,
                height,
                angle_degree,
            )),
            DescriptionCamera::Orthogonal {
                eye,
                look_at,
                width,
                height,
            } => Box::new(OrthogonalCamera::new(eye, look_at, width, height)),
        }
    }
}

#[derive(Debug, Deserialize)]
struct DescriptionObject {
    description: String,
    texture: ModelTexture,
    #[serde(default)]
    effect: Option<TextureEffects>,
    #[serde(flatten)]
    shape: ModelShape,
}

impl DescriptionObject {
    fn into_scene_object(self) -> SceneObject {
        let shape: Box<dyn Shape> = match self.shape {
            ModelShape::Sphere(val) => Box::new(val),
            ModelShape::InfinitePlan { center, normal } => {
                Box::new(InfinitePlan::new(center, normal))
            }
            ModelShape::SquarePlan {
                center,
                normal,
                width,
            } => Box::new(SquarePlan::new(center, normal, width)),
        };
        let texture: Box<dyn Texture> = match self.texture {
            ModelTexture::CheckedPattern(val) => Box::new(val),
            ModelTexture::PlainColor(val) => Box::new(val),
        };
        let effects = self.effect.unwrap_or_default();
        SceneObject {
            texture,
            shape,
            effects,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum ModelShape {
    Sphere(Sphere),
    InfinitePlan {
        center: Vec3,
        normal: Vec3,
    },
    SquarePlan {
        center: Vec3,
        normal: Vec3,
        width: f64,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum ModelTexture {
    CheckedPattern(CheckedPattern),
    PlainColor(PlainColorTexture),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum ModelColor {
    ByString(String),
    ByRGB([f64; 3]),
}

fn default_perspective_angle() -> f64 {
    std::f64::consts::PI / 8.0
}

#[cfg(test)]
mod tests {

    use super::*;

    const INVALID_TOML: &str = r##"invalid_toml"##;

    #[test]
    fn invalid_toml_string() {
        let result = parse_scene_description(INVALID_TOML);
        assert!(result.is_err());
    }
}
