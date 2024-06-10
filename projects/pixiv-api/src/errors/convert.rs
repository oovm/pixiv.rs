use super::*;

impl From<ExampleErrorKind> for PixivError {
    fn from(value: ExampleErrorKind) -> Self {
        Self {
            kind: Box::new(value),
        }
    }
}

impl From<reqwest::Error> for PixivError {
    fn from(value: reqwest::Error) -> Self {
        Self {
            kind: Box::new(ExampleErrorKind::RequestError {
                message: value.to_string(),
                context: "".to_string(),
            }),
        }
    }
}

impl From<std::io::Error> for PixivError {
    fn from(value: std::io::Error) -> Self {
        Self {
            kind: Box::new(ExampleErrorKind::IoError {
                message: value.to_string(),
                file: PathBuf::new(),
            }),
        }
    }
}

