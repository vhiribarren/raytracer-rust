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

mod sample_1;
mod utils;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use crate::utils::canvas::none::NoCanvas;
use crate::utils::canvas::sdl::WrapperCanvas;
use crate::utils::result::RaytracingResult;
use raytracer::renderer::strategy::StandardRenderStrategy;
use raytracer::renderer::{render, RenderConfiguration};
use raytracer::scene::Scene;

const APP_AUTHOR: &str = "Vincent Hiribarren";
const APP_NAME: &str = "raytracer-rust";
const APP_ABOUT: &str = "Toy project to test Rust";
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 576;
const CANVAS_WIDTH: u32 = 1024;
const CANVAS_HEIGHT: u32 = 576;

pub fn main() -> RaytracingResult {
    stderrlog::new().verbosity(4).init()?;

    let matches = clap::App::new(APP_NAME)
        .author(APP_AUTHOR)
        .about(APP_ABOUT)
        .version(APP_VERSION)
        .arg(
            clap::Arg::with_name("no-gui")
                .long("no-gui")
                .help("Does not display the rendering canvas."),
        )
        .get_matches();

    let scene = sample_1::generate_test_scene();
    let render_options = RenderConfiguration {
        canvas_width: CANVAS_WIDTH,
        canvas_height: CANVAS_HEIGHT,
        //render_strategy: Box::new(RandomAntiAliasingRenderStrategy {rays_per_pixel: 20}),
        render_strategy: Box::new(StandardRenderStrategy),
    };

    if matches.is_present("no-gui") {
        render_no_gui(&scene, &render_options)?;
    } else {
        render_sdl(&scene, &render_options)?;
    }

    Ok(())
}

fn render_no_gui(
    scene: &Scene,
    render_options: &RenderConfiguration,
) -> utils::result::RaytracingResult {
    render(&scene, &mut NoCanvas, &render_options)?;
    Ok(())
}

fn render_sdl(
    scene: &Scene,
    render_options: &RenderConfiguration,
) -> utils::result::RaytracingResult {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("RayTracer Test", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .build()?;
    let mut canvas = window.into_canvas().build()?;
    canvas.set_logical_size(CANVAS_WIDTH, CANVAS_HEIGHT)?;
    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    let mut wrapper_canvas = WrapperCanvas(&mut canvas);

    render(&scene, &mut wrapper_canvas, &render_options)?;

    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
