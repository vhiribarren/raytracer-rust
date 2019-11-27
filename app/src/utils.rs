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
