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
use std::time::{Duration, Instant};

use crate::utils::canvas::none::NoCanvas;
use crate::utils::canvas::sdl::WrapperCanvas;
use crate::utils::monitor::TermMonitor;
use crate::utils::result::RaytracingResult;
use raytracer::ray_algorithm::strategy::StandardRenderStrategy;
use raytracer::renderer::{DrawCanvas, ProgressiveRendererIterator, RenderConfiguration};
use raytracer::scene::Scene;
use sdl2::pixels::PixelFormatEnum;
use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};

const APP_AUTHOR: &str = "Vincent Hiribarren";
const APP_NAME: &str = "raytracer-rust";
const APP_ABOUT: &str = "Toy project to test Rust";
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

const WINDOW_WIDTH: u32 = 1024;
const WINDOW_HEIGHT: u32 = 576;
const CANVAS_WIDTH: u32 = 1024;
const CANVAS_HEIGHT: u32 = 576;

pub fn main() -> RaytracingResult {
    TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Mixed).unwrap();

    let matches = clap::App::new(APP_NAME)
        .author(APP_AUTHOR)
        .about(APP_ABOUT)
        .version(APP_VERSION)
        .arg(
            clap::Arg::with_name("no-gui")
                .long("no-gui")
                .help("Does not display the ray_algorithm canvas."),
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
    let total_pixels = render_options.canvas_height * render_options.canvas_width;
    let term_monitor = TermMonitor::new(total_pixels as u64);
    let finally = || {
        term_monitor.clean();
    };
    let render_iterator = ProgressiveRendererIterator::new_try(scene, render_options, finally)?;
    let mut canvas = NoCanvas;

    for pixel in render_iterator {
        canvas.draw(pixel.unwrap())?;
        term_monitor.update();
    }
    Ok(())
}

#[allow(clippy::while_let_on_iterator)]
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

    let mut window_canvas = window.into_canvas().build()?;
    window_canvas.set_logical_size(CANVAS_WIDTH, CANVAS_HEIGHT)?;
    window_canvas.set_draw_color(sdl2::pixels::Color::RGB(77, 77, 170));
    window_canvas.clear();
    window_canvas.present();
    window_canvas.clear();

    let texture_creator = window_canvas.texture_creator();

    let mut render_canvas =
        sdl2::surface::Surface::new(WINDOW_WIDTH, WINDOW_HEIGHT, PixelFormatEnum::RGBA32)?
            .into_canvas()?;

    let total_pixels = render_options.canvas_width * render_options.canvas_height;
    let term_monitor = TermMonitor::new(total_pixels as u64);
    let finally = || {
        term_monitor.clean();
    };
    let renderer_iterator = ProgressiveRendererIterator::new_try(scene, render_options, finally)?;
    let mut renderer_iterator = renderer_iterator.peekable();

    let mut event_pump = sdl_context.event_pump()?;
    'event_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'event_loop,
                _ => {}
            }
        }
        if renderer_iterator.peek().is_some() {
            let instant = Instant::now();
            let mut wrapper_canvas = WrapperCanvas(&mut render_canvas);

            while let Some(pixel) = renderer_iterator.next() {
                wrapper_canvas.draw(pixel.unwrap())?;
                term_monitor.update();
                if instant.elapsed().as_millis() > 20 {
                    break;
                }
            }
            let texture = texture_creator.create_texture_from_surface(render_canvas.surface())?;
            window_canvas.copy(&texture, None, None)?;
            window_canvas.present();
        } else {
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    Ok(())
}
