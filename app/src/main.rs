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

use std::time::{Duration, Instant};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use crate::utils::canvas::none::NoCanvas;
use crate::utils::canvas::sdl::WrapperCanvas;
use crate::utils::monitor::ProgressionMonitor;
use crate::utils::monitor::{NoMonitor, TermMonitor};
use crate::utils::result::{RaytracingResult};
use log::{info};
use raytracer::ray_algorithm::strategy::{
    RandomAntiAliasingRenderStrategy, StandardRenderStrategy,
};
use raytracer::ray_algorithm::AnyPixelRenderStrategy;
use raytracer::renderer::{DrawCanvas, ProgressiveRendererIterator, RenderConfiguration, render_parallel};
use raytracer::scene::Scene;

use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};

const APP_AUTHOR: &str = "Vincent Hiribarren";
const APP_NAME: &str = "raytracer-rust";
const APP_ABOUT: &str = "Toy project to test Rust";
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

const WINDOW_WIDTH: u32 = 800;
const CANVAS_WIDTH: u32 = 1024;
const SDL_WINDOW_CLEAR_COLOR: sdl2::pixels::Color = sdl2::pixels::Color {
    r: 77,
    g: 77,
    b: 170,
    a: 255,
};

pub fn main() -> RaytracingResult {
    TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Mixed).unwrap();

    let matches = clap::App::new(APP_NAME)
        .author(APP_AUTHOR)
        .about(APP_ABOUT)
        .version(APP_VERSION)
        .arg(
            clap::Arg::with_name("no-status")
                .long("no-status")
                .help("Do not display textual progressive bar (quicker)."),
        )
        .arg(
            clap::Arg::with_name("no-gui")
                .long("no-gui")
                .help("Do not display the result of the rendering."),
        )
        .arg(
            clap::Arg::with_name("no-progressive")
                .long("no-progressive")
                .conflicts_with("no-gui")
                .help("Do not render in realtime in the window if GUI is activate (quicker)."),
        )
        .arg(
            clap::Arg::with_name("width")
                .short("w")
                .long("width")
                .takes_value(true)
                .conflicts_with("height")
                .help(format!("Canvas width, default: {}.", CANVAS_WIDTH).as_str()),
        )
        .arg(
            clap::Arg::with_name("height")
                .short("h")
                .long("height")
                .takes_value(true)
                .conflicts_with("width")
                .help("Canvas height."),
        )
        .arg(
            clap::Arg::with_name("strategy-random")
                .long("strategy-random")
                .value_name("RAY_COUNT")
                .help("Average of RAY_COUNT random rays sent."),
        )
        .get_matches();

    let scene = sample_1::generate_test_scene();

    // Camera ratio
    let camera_ratio = scene.camera.size_ratio();
    let (canvas_width, canvas_height) =
        match (matches.value_of("width"), matches.value_of("height")) {
            (Some(_), Some(_)) => unreachable!(),
            (Some(w), None) => {
                let width = w.parse::<f64>().expect("A number is expected");
                (width as u32, (width / camera_ratio) as u32)
            }
            (None, Some(h)) => {
                let height = h.parse::<f64>().expect("A number is expected");
                ((height * camera_ratio) as u32, height as u32)
            }
            (None, None) => {
                let width = CANVAS_WIDTH as f64;
                (width as u32, (width / camera_ratio) as u32)
            }
        };

    info!("Camera ratio; {:.2}", camera_ratio);
    info!("Canvas size: {}x{}", canvas_width, canvas_height);

    // Ray casting strategy
    let render_strategy: Box<dyn AnyPixelRenderStrategy> = if matches.is_present("strategy-random")
    {
        let rays_per_pixel: u32 = matches
            .value_of("strategy-random")
            .unwrap()
            .parse()
            .expect("A number is expected");
        Box::new(RandomAntiAliasingRenderStrategy { rays_per_pixel })
    } else {
        Box::new(StandardRenderStrategy)
    };

    // Terminal progress bar
    let monitor: Box<dyn ProgressionMonitor> = if matches.is_present("no-status") {
        Box::new(NoMonitor)
    } else {
        Box::new(TermMonitor::new((canvas_height * canvas_width) as u64))
    };

    // Build options
    let render_options = RenderConfiguration {
        canvas_width,
        canvas_height,
        render_strategy,
    };

    if matches.is_present("no-gui") {
        render_no_gui_parallel(&scene, &render_options, monitor)?;
    } else {
        let progressive_rendering = !matches.is_present("no-progressive");
        render_sdl(&scene, &render_options, monitor, progressive_rendering)?;
    }

    Ok(())
}

fn render_no_gui<M: AsRef<dyn ProgressionMonitor>>(
    scene: &Scene,
    render_options: &RenderConfiguration,
    monitor: M,
) -> utils::result::RaytracingResult {
    let monitor = monitor.as_ref();
    let finally = || monitor.clean();
    let render_iterator = ProgressiveRendererIterator::new_try(scene, render_options, finally)?;
    let mut canvas = NoCanvas;

    for pixel in render_iterator {
        canvas.draw(pixel.unwrap())?;
        monitor.update();
    }
    Ok(())
}

fn render_no_gui_parallel<M: AsRef<dyn ProgressionMonitor>>(
    scene: &Scene,
    render_options: &RenderConfiguration,
    monitor: M,
) -> utils::result::RaytracingResult {
    let mut canvas = NoCanvas;
    let monitor = monitor.as_ref();
    let mut update = move |pixel| {
        canvas.draw(pixel).unwrap();
        monitor.update();
    };
    let mut finally = move || {
        monitor.clean();
    };
    render_parallel(scene, render_options, update, finally).map_err(|e| e.into())
}





#[allow(clippy::while_let_on_iterator)]
#[allow(clippy::collapsible_if)]
fn render_sdl<M: AsRef<dyn ProgressionMonitor>>(
    scene: &Scene,
    render_options: &RenderConfiguration,
    monitor: M,
    progressive_rendering: bool,
) -> utils::result::RaytracingResult {
    let monitor = monitor.as_ref();
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window(
            "RayTracer Test",
            WINDOW_WIDTH,
            (WINDOW_WIDTH as f64 / scene.camera.size_ratio()) as u32,
        )
        .position_centered()
        .resizable()
        .build()?;

    let mut window_canvas = window.into_canvas().build()?;
    window_canvas.set_logical_size(render_options.canvas_width, render_options.canvas_height)?;
    window_canvas.set_draw_color(SDL_WINDOW_CLEAR_COLOR);
    // Paint and blit back buffer
    window_canvas.clear();
    window_canvas.present();

    let texture_creator = window_canvas.texture_creator();

    let mut render_canvas = sdl2::surface::Surface::new(
        render_options.canvas_width,
        render_options.canvas_height,
        PixelFormatEnum::RGBA32,
    )?
    .into_canvas()?;
    render_canvas.clear();

    let finally = || {
        monitor.clean();
    };
    let renderer_iterator = ProgressiveRendererIterator::new_try(scene, render_options, finally)?;
    let mut renderer_iterator = renderer_iterator.peekable();

    let mut event_pump = sdl_context.event_pump()?;
    'event_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    let (new_w, new_h) = if w as f64 / h as f64 > scene.camera.size_ratio() {
                        (w as u32, (w as f64 / scene.camera.size_ratio()) as u32)
                    } else {
                        ((h as f64 * scene.camera.size_ratio()) as u32, h as u32)
                    };
                    window_canvas.window_mut().set_size(new_w, new_h)?
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'event_loop,
                Event::KeyDown {
                    keycode: Some(Keycode::F),
                    ..
                } => window_canvas
                    .window_mut()
                    .set_size(render_options.canvas_width, render_options.canvas_height)?,
                _ => {}
            }
        }
        if renderer_iterator.peek().is_some() {
            let instant = Instant::now();
            let mut wrapper_canvas = WrapperCanvas(&mut render_canvas);

            while let Some(pixel) = renderer_iterator.next() {
                wrapper_canvas.draw(pixel.unwrap())?;
                monitor.update();
                if progressive_rendering {
                    if instant.elapsed().as_millis() > 20 {
                        break;
                    }
                }
            }
            let texture = texture_creator.create_texture_from_surface(render_canvas.surface())?;
            window_canvas.clear();
            window_canvas.copy(&texture, None, None)?;
            window_canvas.present();
        } else {
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    Ok(())
}
