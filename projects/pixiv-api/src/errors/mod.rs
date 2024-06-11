use std::fmt::{Debug, Formatter};
use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;

mod display;
mod convert;

/// The result type of this crate.
pub type Result<T> = std::result::Result<T, PixivError>;

/// A boxed error kind, wrapping an [ExampleErrorKind].
#[derive(Clone)]
pub struct PixivError {
    kind: Box<ExampleErrorKind>,
}

/// The kind of [PixivError].
#[derive(Debug, Clone)]
pub enum ExampleErrorKind {
    IoError {
        message: String,
        file: PathBuf,
    },
    RequestError {
        /// The message of the error.
        message: String,
        context: String,
    },
    /// An unknown error.
    UnknownError,
}


impl PixivError {
    pub fn io_error(message: impl Into<String>, file: impl Into<PathBuf>) -> Self {
        Self {
            kind: Box::new(ExampleErrorKind::IoError {
                message: message.into(),
                file: file.into(),
            }),
        }
    }

    pub fn request_error(message: impl Into<String>, context: impl Into<String>) -> Self {
        Self {
            kind: Box::new(ExampleErrorKind::RequestError {
                message: message.into(),
                context: context.into(),
            }),
        }
    }
}