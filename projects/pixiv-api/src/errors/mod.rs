use std::fmt::{Debug, Formatter};
use std::error::Error;
use std::fmt::Display;

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
    /// An unknown error.
    UnknownError,
    RequestError {
        /// The message of the error.
        message: String,
        context: String,
    },
}


impl PixivError {
    pub fn request_error(message: impl Into<String>, context: impl Into<String>) -> Self {
        Self {
            kind: Box::new(ExampleErrorKind::RequestError {
                message: message.into(),
                context: context.into(),
            }),
        }
    }
}