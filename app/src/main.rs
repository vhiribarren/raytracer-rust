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

use crate::utils::canvas::none::NoCanvas;
use crate::utils::canvas::sdl::WrapperCanvas;
use crate::utils::canvas::DrawCanvas;
use crate::utils::monitor::ProgressionMonitor;
use crate::utils::monitor::{NoMonitor, TermMonitor};
use crate::utils::result::{AppError, VoidAppResult};
use log::info;
use raytracer::ray_algorithm::strategy::{
    RandomAntiAliasingRenderStrategy, StandardRenderStrategy,
};
use raytracer::ray_algorithm::AnyPixelRenderStrategy;
use raytracer::renderer::{render_scene, Pixel, RenderConfiguration};
use raytracer::result::Result;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use std::time::{Duration, Instant};

use simplelog::{Config, LevelFilter, TermLogger, TerminalMode};

const APP_AUTHOR: &str = "Vincent Hiribarren";
const APP_NAME: &str = "raytracer-rust";
const APP_ABOUT: &str = "Toy project to test Rust";
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

const ARG_NO_STATUS: &str = "no-status";
const ARG_NO_GUI: &str = "no-gui";
const ARG_NO_PROGRESSIVE: &str = "no-progressive";
const ARG_NO_PARALLEL: &str = "no-parallel";
const ARG_STRATEGY_RANDOM: &str = "strategy-random";
const ARG_WIDTH: &str = "width";
const ARG_HEIGHT: &str = "height";

const WINDOW_WIDTH: u32 = 800;
const CANVAS_WIDTH: u32 = 1024;
const SDL_WINDOW_CLEAR_COLOR: sdl2::pixels::Color = sdl2::pixels::Color {
    r: 77,
    g: 77,
    b: 170,
    a: 255,
};

fn main() -> VoidAppResult {
    TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Mixed)
        .expect("Error while initializing logger");

    let matches = clap::App::new(APP_NAME)
        .author(APP_AUTHOR)
        .about(APP_ABOUT)
        .version(APP_VERSION)
        .arg(
            clap::Arg::with_name(ARG_NO_STATUS)
                .long("no-status")
                .help("Do not display textual progressive bar (quicker)."),
        )
        .arg(
            clap::Arg::with_name(ARG_NO_GUI)
                .long("no-gui")
                .help("Do not display the result of the rendering."),
        )
        .arg(
            clap::Arg::with_name(ARG_NO_PROGRESSIVE)
                .long("no-progressive")
                .conflicts_with(ARG_NO_GUI)
                .help("Do not render in realtime in the window if GUI is activate (quicker)."),
        )
        .arg(
            clap::Arg::with_name(ARG_NO_PARALLEL)
                .long("no-parallel")
                .help("Do not use multithreading for parallel computation (slower)."),
        )
        .arg(
            clap::Arg::with_name(ARG_WIDTH)
                .short("w")
                .long("width")
                .takes_value(true)
                .conflicts_with(ARG_HEIGHT)
                .help(format!("Canvas width, default: {}.", CANVAS_WIDTH).as_str()),
        )
        .arg(
            clap::Arg::with_name(ARG_HEIGHT)
                .short("h")
                .long("height")
                .takes_value(true)
                .conflicts_with(ARG_WIDTH)
                .help("Canvas height."),
        )
        .arg(
            clap::Arg::with_name(ARG_STRATEGY_RANDOM)
                .long("strategy-random")
                .value_name("RAY_COUNT")
                .help("Average of RAY_COUNT random rays sent."),
        )
        .get_matches();

    // Generate scene to render
    let scene = sample_1::generate_test_scene();

    // Camera ratio
    let camera_ratio = scene.camera.size_ratio();
    let (canvas_width, canvas_height) =
        match (matches.value_of(ARG_WIDTH), matches.value_of(ARG_HEIGHT)) {
            (Some(_), Some(_)) => unreachable!(),
            (Some(w), None) => {
                let width = w.parse::<f64>().map_err(|e| {
                    AppError::BadArgument(format!("Error when parsing width value: {}", e))
                })?;
                (width as u32, (width / camera_ratio) as u32)
            }
            (None, Some(h)) => {
                let height = h.parse::<f64>().map_err(|e| {
                    AppError::BadArgument(format!("Error when parsing height value: {}", e))
                })?;
                ((height * camera_ratio) as u32, height as u32)
            }
            (None, None) => {
                let width = CANVAS_WIDTH as f64;
                (width as u32, (width / camera_ratio) as u32)
            }
        };

    // Ray casting strategy
    let render_strategy: Box<dyn AnyPixelRenderStrategy> =
        if let Some(strategy) = matches.value_of(ARG_STRATEGY_RANDOM) {
            let rays_per_pixel: u32 = strategy.parse().map_err(|e| {
                AppError::BadArgument(format!("Error when parsing strategy value: {}", e))
            })?;
            Box::new(RandomAntiAliasingRenderStrategy { rays_per_pixel })
        } else {
            Box::new(StandardRenderStrategy)
        };

    // Terminal progress bar
    let monitor: Box<dyn ProgressionMonitor> = if matches.is_present(ARG_NO_STATUS) {
        Box::new(NoMonitor)
    } else {
        Box::new(TermMonitor::new((canvas_height * canvas_width) as u64))
    };

    // Build options
    let config = RenderConfiguration {
        canvas_width,
        canvas_height,
        render_strategy,
    };

    info!("Camera ratio; {:.2}", camera_ratio);
    info!("Canvas size: {}x{}", canvas_width, canvas_height);

    // Sequential or parallel computation
    let render_iter = render_scene(scene, config, !matches.is_present(ARG_NO_PARALLEL), || {
        monitor.clean()
    })?;

    // Launch the computation / rendering
    if matches.is_present(ARG_NO_GUI) {
        render_no_gui(render_iter, &monitor)?;
    } else {
        let progressive_rendering = !matches.is_present(ARG_NO_PROGRESSIVE);
        render_sdl(
            render_iter,
            &monitor,
            canvas_width,
            canvas_height,
            camera_ratio,
            progressive_rendering,
        )?;
    }

    Ok(())
}

fn render_no_gui<M: AsRef<dyn ProgressionMonitor>>(
    render_iter: impl Iterator<Item = Result<Pixel>>,
    monitor: M,
) -> VoidAppResult {
    let monitor = monitor.as_ref();
    let mut canvas = NoCanvas;
    for pixel in render_iter {
        canvas.draw(pixel?)?;
        monitor.update();
    }
    Ok(())
}

#[allow(clippy::while_let_on_iterator)]
#[allow(clippy::collapsible_if)]
fn render_sdl<M: AsRef<dyn ProgressionMonitor>>(
    render_iter: impl Iterator<Item = Result<Pixel>>,
    monitor: M,
    canvas_width: u32,
    canvas_height: u32,
    camera_ratio: f64,
    progressive_rendering: bool,
) -> VoidAppResult {
    let monitor = monitor.as_ref();

    let mut render_iter = render_iter.peekable();
    let mut render_canvas =
        sdl2::surface::Surface::new(canvas_width, canvas_height, PixelFormatEnum::RGBA32)
            .map_err(AppError::SdlError)?
            .into_canvas()
            .map_err(AppError::SdlError)?;
    render_canvas.set_draw_color(SDL_WINDOW_CLEAR_COLOR);
    render_canvas.clear();

    if !progressive_rendering {
        // We prepare immediately the result before displaying it
        let mut wrapper_canvas = WrapperCanvas(&mut render_canvas);
        while let Some(pixel) = render_iter.next() {
            wrapper_canvas.draw(pixel?)?;
            monitor.update();
        }
    }

    let sdl_context = sdl2::init().map_err(AppError::SdlError)?;
    let video_subsystem = sdl_context.video().map_err(AppError::SdlError)?;
    let window = video_subsystem
        .window(
            "RayTracer Test",
            WINDOW_WIDTH,
            (WINDOW_WIDTH as f64 / camera_ratio) as u32,
        )
        .position_centered()
        .resizable()
        .build()?;

    let mut window_canvas = window.into_canvas().build()?;
    window_canvas.set_logical_size(canvas_width, canvas_height)?;
    window_canvas.set_draw_color(SDL_WINDOW_CLEAR_COLOR);
    // Paint and blit back buffer
    window_canvas.clear();
    window_canvas.present();

    let texture_creator = window_canvas.texture_creator();
    let mut texture = texture_creator.create_texture_from_surface(render_canvas.surface())?;

    let mut event_pump = sdl_context.event_pump().map_err(AppError::SdlError)?;
    'event_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    win_event: WindowEvent::Resized(w, h),
                    ..
                } => {
                    let (new_w, new_h) = if w as f64 / h as f64 > camera_ratio {
                        (w as u32, (w as f64 / camera_ratio) as u32)
                    } else {
                        ((h as f64 * camera_ratio) as u32, h as u32)
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
                    .set_size(canvas_width, canvas_height)?,
                _ => {}
            }
        }
        if render_iter.peek().is_some() {
            let instant = Instant::now();
            let mut wrapper_canvas = WrapperCanvas(&mut render_canvas);

            while let Some(pixel) = render_iter.next() {
                wrapper_canvas.draw(pixel?)?;
                monitor.update();
                if progressive_rendering {
                    if instant.elapsed().as_millis() > 20 {
                        break;
                    }
                }
            }
            texture = texture_creator.create_texture_from_surface(render_canvas.surface())?;
            window_canvas.clear();
            window_canvas
                .copy(&texture, None, None)
                .map_err(AppError::SdlError)?;
            window_canvas.present();
        } else {
            window_canvas.clear();
            window_canvas
                .copy(&texture, None, None)
                .map_err(AppError::SdlError)?;
            window_canvas.present();
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    Ok(())
}
