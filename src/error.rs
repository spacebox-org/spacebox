use std::{
    error,
    fmt,
    result
};

/// This is the main error type for the Spacebox application.
/// All possible error cases for the application shouid be rolled
/// into one of the contained enum branches.
#[derive(Clone, Debug)]
pub enum Error {

}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            _ => write!(f, "")
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

/// This defines a custom result type that should be usable
/// with a single type parameter most places in the application.
pub type Result<T> = result::Result<T, Error>;