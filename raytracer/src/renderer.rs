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
use crate::ray_algorithm::AnyPixelRenderStrategy;
use crate::scene::Scene;
use log::{debug, info, warn};
use std::iter::{from_fn};
use std::time::Instant;

pub trait DrawCanvas {
    fn draw(&mut self, pixel: Pixel) -> Result<(), String>;
}

pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub color: Color,
}

impl Pixel {
    pub fn new(x: u32, y: u32, color: Color) -> Pixel {
        Pixel { x, y, color }
    }
}

pub struct RenderConfiguration {
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub render_strategy: Box<dyn AnyPixelRenderStrategy>,
}

pub struct ProgressiveRenderer<'a> {
    scene: &'a Scene,
    config: &'a RenderConfiguration,
}

impl ProgressiveRenderer<'_> {
    pub fn new<'a>(scene: &'a Scene, config: &'a RenderConfiguration) -> ProgressiveRenderer<'a> {
        ProgressiveRenderer { scene, config }
    }

    pub fn total_pixels(&self) -> u32 {
        self.config.canvas_width * self.config.canvas_height
    }

    pub fn render<E, F>(&self, mut callback: E, finally: F) -> Result<(), String>
    where
        E: FnMut(Pixel) -> Result<(), String>,
        F: Fn(),
    {
        let area_iterator = ProgressiveRendererIterator::new_try(self.scene, self.config, finally)?;
        for result in area_iterator {
            match result {
                Err(val) => return Err(val),
                Ok(pixel) => {
                    callback(pixel)?;
                }
            }
        }
        Ok(())
    }
}

pub fn simple_render_to_canvas(
    scene: &Scene,
    canvas: &mut impl DrawCanvas,
    config: &RenderConfiguration,
) -> Result<(), String> {
    ProgressiveRenderer::new(scene, config).render(|pixel| canvas.draw(pixel), || {})
}

pub struct ProgressiveRendererIterator<'a>(Box<dyn Iterator<Item = Result<Pixel, String>> + 'a>);

impl ProgressiveRendererIterator<'_> {
    pub fn new_try<'a, F: 'a + Fn()>(
        scene: &'a Scene,
        config: &'a RenderConfiguration,
        finally: F,
    ) -> Result<ProgressiveRendererIterator<'a>, String> {
        if cfg!(debug_assertions) {
            warn!("Debug compiled binary is used, performance will be low!");
        }
        debug!("render: {} objects to process", scene.objects.len());
        debug!("render: {} lights to process", scene.lights.len());
        if scene.lights.is_empty() {
            return Err(String::from("There is no light in the scene"));
        }
        let instant = Instant::now();
        let iter_end = move || {
            let instant_start = instant;
            finally();
            info!("render: done!");
            info!(
                "render: duration: {:.3} seconds",
                instant_start.elapsed().as_secs_f32()
            );
            None::<Result<Pixel, String>>
        };
        let area_render_iterator = AreaRenderIterator::with_full_area(scene, config);
        let render_iterator = area_render_iterator.chain(from_fn(iter_end)).fuse();

        Ok(ProgressiveRendererIterator(Box::new(render_iterator)))
    }
}

impl Iterator for ProgressiveRendererIterator<'_> {
    type Item = Result<Pixel, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub struct AreaRenderIterator<'a> {
    scene: &'a Scene,
    config: &'a RenderConfiguration,
    area_x_origin: u32,
    #[allow(dead_code)]
    area_y_origin: u32,
    area_width: u32,
    area_height: u32,
    area_x_current: u32,
    area_y_current: u32,
    pixel_width: f64,
    pixel_height: f64,
}

impl AreaRenderIterator<'_> {
    pub fn new<'a>(
        scene: &'a Scene,
        config: &'a RenderConfiguration,
        area_x: u32,
        area_y: u32,
        area_width: u32,
        area_height: u32,
    ) -> AreaRenderIterator<'a> {
        AreaRenderIterator {
            scene,
            config,
            area_x_origin: area_x,
            area_y_origin: area_y,
            area_width,
            area_height,
            area_x_current: area_x,
            area_y_current: area_y,
            pixel_width: 1.0 / config.canvas_width as f64,
            pixel_height: 1.0 / config.canvas_height as f64,
        }
    }

    pub fn with_full_area<'a>(
        scene: &'a Scene,
        config: &'a RenderConfiguration,
    ) -> AreaRenderIterator<'a> {
        Self::new(
            scene,
            config,
            0,
            0,
            config.canvas_width,
            config.canvas_height,
        )
    }

    pub fn total_pixels(&self) -> usize {
        (self.area_width * self.area_height) as usize
    }
}

impl Iterator for AreaRenderIterator<'_> {
    type Item = Result<Pixel, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.area_y_current >= self.area_height {
            return None;
        }
        let x = self.area_x_current;
        let y = self.area_y_current;
        let canvas_x = (self.area_x_current as f64) / (self.config.canvas_width as f64);
        let canvas_y = (self.area_y_current as f64) / (self.config.canvas_height as f64);
        let render_strategy = &*self.config.render_strategy;
        let result_color = render_strategy.render_pixel(
            self.scene,
            canvas_x,
            canvas_y,
            self.pixel_width,
            self.pixel_height,
        );
        let color = match result_color {
            Ok(val) => val,
            Err(val) => return Some(Err(val)),
        };
        self.area_x_current += 1;
        if self.area_x_current >= self.area_width {
            self.area_x_current = self.area_x_origin;
            self.area_y_current += 1;
        }
        Some(Ok(Pixel::new(x, y, color)))
    }
}
