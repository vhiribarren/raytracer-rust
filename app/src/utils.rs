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

pub mod monitor {
    use log::warn;
    pub trait ProgressionMonitor: Sync {
        fn update(&self);
        fn clean(&self);
    }

    pub struct TermMonitor(indicatif::ProgressBar);

    impl TermMonitor {
        pub fn new(total_pixels: u64) -> TermMonitor {
            let progress_bar = indicatif::ProgressBar::new(total_pixels as u64);
            progress_bar.set_style(
                indicatif::ProgressStyle::default_bar()
                    .template("{msg} {bar} {percent}% Elapsed: {elapsed} ETA: {eta}"),
            );
            progress_bar.set_draw_delta((total_pixels / 100) as u64); // Update every percent
            progress_bar.set_message(format!("Processing {} pixels...", total_pixels).as_str());
            if progress_bar.is_hidden() {
                warn!("Cannot show progress bar, requires TTY");
            }
            TermMonitor(progress_bar)
        }
    }

    impl ProgressionMonitor for TermMonitor {
        fn update(&self) {
            self.0.inc(1);
        }
        fn clean(&self) {
            self.0.finish_and_clear();
        }
    }

    pub struct NoMonitor;

    impl ProgressionMonitor for NoMonitor {
        fn update(&self) {}

        fn clean(&self) {}
    }
}

pub mod result {
    use crate::utils::result::AppError::*;
    use log::SetLoggerError;
    use sdl2::render::TextureValueError;
    use sdl2::video::WindowBuildError;
    use sdl2::IntegerOrSdlError;
    use std::fmt::Result;
    use std::fmt::{Display, Formatter};

    impl From<WindowBuildError> for AppError {
        fn from(err: WindowBuildError) -> Self {
            SdlError(err.to_string())
        }
    }

    impl From<IntegerOrSdlError> for AppError {
        fn from(err: IntegerOrSdlError) -> Self {
            SdlError(err.to_string())
        }
    }

    impl From<SetLoggerError> for AppError {
        fn from(err: SetLoggerError) -> Self {
            LoggerError(err.to_string())
        }
    }

    impl From<TextureValueError> for AppError {
        fn from(err: TextureValueError) -> Self {
            SdlError(err.to_string())
        }
    }

    impl From<String> for AppError {
        fn from(err: String) -> Self {
            RaytracingError(err)
        }
    }

    pub type RaytracingResult = std::result::Result<(), AppError>;

    #[derive(Debug)]
    pub enum AppError {
        SdlError(String),
        RaytracingError(String),
        LoggerError(String),
        MiscError(String),
    }

    impl Display for AppError {
        fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
            match self {
                SdlError(val) => write!(formatter, "SDL: {}", val),
                RaytracingError(val) => write!(formatter, "RayTracer: {}", val),
                LoggerError(val) => write!(formatter, "Logger: {}", val),
                MiscError(val) => write!(formatter, "Other: {}", val),
            }
        }
    }
}

pub mod canvas {

    pub mod sdl {
        use raytracer::renderer::{DrawCanvas, Pixel};
        use sdl2::render::Canvas;

        pub struct WrapperCanvas<'a, T: sdl2::render::RenderTarget>(pub &'a mut Canvas<T>);

        impl<T: sdl2::render::RenderTarget> DrawCanvas for WrapperCanvas<'_, T> {
            fn draw(&mut self, p: Pixel) -> std::result::Result<(), String> {
                let draw_color = sdl2::pixels::Color::RGB(
                    (255.0 * p.color.red()) as u8,
                    (255.0 * p.color.green()) as u8,
                    (255.0 * p.color.blue()) as u8,
                );
                self.0.set_draw_color(draw_color);
                self.0
                    .draw_point(sdl2::rect::Point::new(p.x as i32, p.y as i32))?;
                Ok(())
            }
        }
    }

    pub mod none {
        use raytracer::renderer::{DrawCanvas, Pixel};

        pub struct NoCanvas;

        impl DrawCanvas for NoCanvas {
            fn draw(&mut self, _pixel: Pixel) -> Result<(), String> {
                Ok(())
            }
        }
    }
}
