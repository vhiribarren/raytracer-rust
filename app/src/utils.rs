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

pub mod result {
    use crate::utils::result::AppError::*;
    use log::SetLoggerError;
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
        use sdl2::video::Window;

        pub struct WrapperCanvas<'a>(pub &'a mut Canvas<Window>);

        impl DrawCanvas for WrapperCanvas<'_> {
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
