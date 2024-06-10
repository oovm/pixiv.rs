use super::*;

impl Error for PixivError {}


impl Debug for PixivError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl Display for PixivError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}


impl Display for ExampleErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownError => { write!(f, "UnknownError") }
            Self::RequestError { message, context } => {
                write!(f, "RequestError: {}\n    at: {}", message, context)
            }
            Self::IoError { message, file } => {
                write!(f, "IoError: {}\n    at: {}", message, file.display())
            }
        }
    }
}