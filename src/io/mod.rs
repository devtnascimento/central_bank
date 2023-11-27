use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub enum RequestError<'a> {
    KeyNotFound(&'a str),
    Other(&'a str),
}

impl<'a> Display for RequestError<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            RequestError::KeyNotFound(ref key) => write!(f, "Pix key {} not found", key),
            RequestError::Other(ref message) => write!(f, "{}", message),
        }
    }
}

impl<'a> Error for RequestError<'a> {}
