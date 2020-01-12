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

#![cfg(target_arch = "wasm32")]

use crate::renderer::{render_scene, Pixel, RenderConfiguration};
use crate::result::Result;
use log::*;
use wasm_bindgen::prelude::*;
use std::str::FromStr;

#[allow(unused_imports)]
use crate::ray_algorithm::strategy::{RandomAntiAliasingRenderStrategy, StandardRenderStrategy};
use crate::scene::Scene;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn wasm_init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    #[cfg(feature = "console_log")]
    console_log::init_with_level(Level::Trace).expect("error initializing log");
}

#[wasm_bindgen]
pub struct Renderer {
    render_iterator: Box<dyn Iterator<Item = Result<Pixel>>>,
    img_buffer: Vec<u8>,
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl Renderer {
    pub fn new(scene_description: &str) -> std::result::Result<Renderer, JsValue> {
        let scene = Scene::from_str(scene_description).map_err(|e| e.to_string())?;
        //let config = <RenderConfiguration as Default>::default();
        let config = RenderConfiguration {
            canvas_width: 1024,
            canvas_height: 576,
            render_strategy: Box::new(RandomAntiAliasingRenderStrategy { rays_per_pixel: 50 }),
        };
        let width = config.canvas_width;
        let height = config.canvas_height;
        let img_buffer = vec![0; (config.canvas_width * config.canvas_height * 4) as usize];
        let render_iterator = Box::new(render_scene(scene, config, false).unwrap());
        Ok(Renderer {
            render_iterator,
            img_buffer,
            width,
            height,
        })
    }

    pub fn buffer_ptr(&self) -> *const u8 {
        self.img_buffer.as_ptr()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn next(&mut self) -> bool {
        match self.render_iterator.next() {
            None => false,
            Some(Ok(pixel)) => {
                let index = 4 * (pixel.x + pixel.y * self.width) as usize;
                self.img_buffer[index] = (pixel.color.red() * 255.0) as u8;
                self.img_buffer[index + 1] = (pixel.color.green() * 255.0) as u8;
                self.img_buffer[index + 2] = (pixel.color.blue() * 255.0) as u8;
                self.img_buffer[index + 3] = 0xFF;
                true
            }
            Some(Err(err)) => {
                warn!("{}", err);
                false
            }
        }
    }
}

// Test Scene
/////////////

mod test_scene {

    use crate::cameras::PerspectiveCamera;
    use crate::colors::Color;
    use crate::lights::LightPoint;
    use crate::primitives::{InfinitePlan, Sphere};
    use crate::scene::{Scene, SceneConfiguration, SceneObject};
    use crate::textures::{
        CheckedPattern, Mirror, PlainColorTexture, TextureEffects, Transparency,
    };
    use crate::vector::Vec3;
    use std::f64::consts::PI;
    use std::str::FromStr;

    pub(crate) fn generate_test_scene() -> Scene {
        let scene_toml = include_str!("../../samples/show_room_1.toml");
        Scene::from_str(scene_toml).unwrap()
    }
}
