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
use crate::lights::AnyLightObject;
use crate::primitives::{Ray, Shape};
use crate::textures::{Texture, TextureEffects};
use crate::vector::Vec3;
use crate::UnitInterval;

pub struct Scene {
    pub camera: Box<dyn RayEmitter>,
    pub lights: Vec<Box<dyn AnyLightObject>>,
    pub objects: Vec<Box<dyn AnySceneObject>>,
    pub config: SceneConfiguration,
}

pub struct SceneConfiguration {
    pub world_color: Color,
    pub world_refractive_index: f64,
    pub ambient_light: Option<Color>,
    pub maximum_light_recursion: u8,
}

impl Default for SceneConfiguration {
    fn default() -> Self {
        SceneConfiguration {
            world_color: Color::BLACK,
            world_refractive_index: 1.0,
            ambient_light: Some(Color::new(0.2, 0.2, 0.2)),
            maximum_light_recursion: 2,
        }
    }
}

pub trait AnySceneObject {
    fn color_at(&self, point: Vec3) -> Color;
    fn check_collision(&self, ray: &Ray) -> Option<Vec3>;
    fn normal_at(&self, point: Vec3) -> Option<Vec3>;
    fn effects(&self) -> &TextureEffects;
}

pub struct SceneObject<T: Texture, P: Shape> {
    pub texture: T,
    pub primitive: P,
    pub effects: TextureEffects,
}

impl<T: Texture, P: Shape> AnySceneObject for SceneObject<T, P> {
    fn color_at(&self, point: Vec3) -> Color {
        let (u, v) = self.primitive.surface_mapping_at(point).unwrap();
        self.texture.color_at(u, v)
    }

    fn check_collision(&self, ray: &Ray) -> Option<Vec3> {
        self.primitive.check_collision(ray)
    }

    fn normal_at(&self, point: Vec3) -> Option<Vec3> {
        self.primitive.normal_at(point)
    }

    fn effects(&self) -> &TextureEffects {
        &self.effects
    }
}

pub trait RayEmitter {
    fn width(&self) -> f64;
    fn height(&self) -> f64;
    fn size_ratio(&self) -> f64 {
        self.width() / self.height()
    }
    fn generate_ray(&self, canvas_x: UnitInterval, canvas_y: UnitInterval) -> Ray;
}
