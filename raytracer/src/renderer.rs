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
use std::iter::from_fn;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::thread::JoinHandle;
use std::time::Instant;

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

pub fn check_rendering_context(scene: &Scene) -> Result<(), String> {
    if cfg!(debug_assertions) {
        warn!("Debug compiled binary is used, performance will be low!");
    }
    debug!("render: {} objects to process", scene.objects.len());
    debug!("render: {} lights to process", scene.lights.len());
    if scene.lights.is_empty() {
        return Err(String::from("There is no light in the scene"));
    }
    Ok(())
}

pub fn render_parallel(
    scene: Scene,
    config: RenderConfiguration,
) -> Result<impl Iterator<Item = Result<Pixel, String>>, String> {
    check_rendering_context(&scene)?;

    let (tx, rx) = channel();

    std::thread::spawn(move || {
        let scene = &scene;
        let config = &config;
        let pixel_width = 1.0 / config.canvas_width as f64;
        let pixel_height = 1.0 / config.canvas_height as f64;

        rayon::scope(move |s| {
            for y in 0..config.canvas_height {
                for x in 0..config.canvas_width {
                    let tx = tx.clone();
                    s.spawn(move |_| {
                        let canvas_x = x as f64 / (config.canvas_width as f64);
                        let canvas_y = y as f64 / (config.canvas_height as f64);
                        let res_color = config.render_strategy.render_pixel(
                            &scene,
                            canvas_x,
                            canvas_y,
                            pixel_width,
                            pixel_height,
                        );
                        let pixel = match res_color {
                            Ok(color) => Ok(Pixel::new(x, y, color)),
                            Err(err) => Err(err),
                        };
                        tx.send(pixel).unwrap();
                    });
                }
            }
        });
    });

    Ok((rx.into_iter()))
}

pub fn render_sequential(
    scene: Scene,
    config: RenderConfiguration,
) -> Result<impl Iterator<Item = Result<Pixel, String>>, String> {
    check_rendering_context(&scene)?;
    Ok(AreaRenderIterator::with_full_area(scene, config))
}

pub struct AreaRenderIterator{
    scene: Scene,
    config: RenderConfiguration,
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

impl AreaRenderIterator {
    pub fn new(
        scene: Scene,
        config: RenderConfiguration,
        area_x: u32,
        area_y: u32,
        area_width: u32,
        area_height: u32,
    ) -> AreaRenderIterator {
        AreaRenderIterator {
            pixel_width: 1.0 / config.canvas_width as f64,
            pixel_height: 1.0 / config.canvas_height as f64,
            scene,
            config,
            area_x_origin: area_x,
            area_y_origin: area_y,
            area_width,
            area_height,
            area_x_current: area_x,
            area_y_current: area_y,

        }
    }

    pub fn with_full_area(
        scene: Scene,
        config: RenderConfiguration,
    ) -> AreaRenderIterator {
        let area_width = config.canvas_width;
        let area_height = config.canvas_height;
        Self::new(
            scene,
            config,
            0,
            0,
            area_width,
            area_height,
        )
    }

    pub fn total_pixels(&self) -> usize {
        (self.area_width * self.area_height) as usize
    }
}

impl Iterator for AreaRenderIterator {
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
            &self.scene,
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
