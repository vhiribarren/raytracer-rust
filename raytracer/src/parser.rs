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
SOFTWARE
*/


use crate::lights::{LightPoint, AnyLightObject};
use log::info;
use toml::value::Table;
use crate::scene::{Scene, AnySceneObject, SceneConfiguration, RayEmitter};
use crate::result::RaytracerError;
use toml::Value;
use crate::colors::Color;
use crate::vector::Vec3;
use crate::cameras::PerspectiveCamera;
use crate::result::Result;
use serde::{Deserialize};
use crate::UnitInterval;


#[derive(Debug,Deserialize)]
struct Root {
    description: Option<String>,
    config: Config,
    camera: Camera,
    object: Vec<Object>,
    light: Vec<Light>,
}

#[derive(Debug,Deserialize)]
pub struct Config {
    world_color:  Option<ModelColor>,
    #[serde(default = "default_world_refractive_index")]
    world_refractive_index: f64,
    ambient_light: Option<ModelColor>,
    #[serde(default = "default_maximum_light_recursion")]
    maximum_light_recursion: u8,
}

fn default_world_refractive_index() -> f64 {
    1.0
}

fn default_maximum_light_recursion() -> u8 {
    4
}

#[derive(Debug,Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Light {
    Point {
        source: [f64; 3],
        color: ModelColor,
    }
}

#[derive(Debug,Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Camera {
    Perspective {
        screen_center: [f64; 3],
        look_at: [f64; 3],
        width: f64,
        height: f64,
        #[serde(default = "default_perspective_angle")]
        angle_degree: f64
    },
    Orthogonal {
        eye: [f64; 3],
        look_at: [f64; 3],
        width: f64,
        height: f64,
    },
}

#[derive(Debug,Deserialize)]
struct Object {
    texture: Texture,
    effect: Option<Effect>,
    #[serde(flatten)]
    _object_primitive: ObjectPrimitive,
}

#[derive(Debug,Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum ObjectPrimitive {
    Sphere {
        center: [f64; 3],
        radius: f64,
    },
    InfinitePlan {
        center: [f64; 3],
        normal: [f64; 3],
    },
    SquarePlan {
        center: [f64; 3],
        normal: [f64; 3],
        width: f64,
    }
}

#[derive(Debug,Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum Texture {
    CheckedPattern {
        primary_color: ModelColor,
        secondary_color: ModelColor,
        count: u32,
    },
    PlainColor {
        color: ModelColor,
    }
}

#[derive(Debug,Deserialize)]
#[serde(untagged)]
enum ModelColor {
    ByString(String),
    ByRGB([f64; 3]),
}


#[derive(Debug,Deserialize)]
#[serde(rename_all = "snake_case")]
struct Effect {
    mirror: Option<Mirror>,
    transparency: Option<Transparency>,
    phong: Option<Phong>,
}

#[derive(Debug,Deserialize)]
#[serde(rename_all = "snake_case")]
struct Mirror {
    coeff: UnitInterval,
}

#[derive(Debug,Deserialize)]
#[serde(rename_all = "snake_case")]
struct Transparency {
    refractive_index: f64,
    alpha: UnitInterval,
}

#[derive(Debug,Deserialize)]
#[serde(rename_all = "snake_case")]
struct Phong {
    size: u32,
    lum_coeff: UnitInterval,
}


fn default_perspective_angle() -> f64 {
    std::f64::consts::PI / 8.0
}


pub(crate) fn parse_scene_description(scene_str: &str) -> Result<Root>  {
    let root_document = toml::from_str::<Root>(scene_str)
        .map_err(|e| RaytracerError::ParsingError(e.to_string()))?;
    Ok(root_document)
}

#[cfg(test)]
mod tests {

    use super::*;

    const INVALID_TOML: &str = r##"invalid_toml"##;

    #[test]
    fn invalid_toml_string() {
        let result = parse_scene_description(invalid_toml);
        assert!(result.is_err());
    }


}