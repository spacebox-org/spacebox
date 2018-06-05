use std::io;
use std::result;

#[derive(Debug)]
pub enum FsError {
    IOError(io::Error),
}

impl From<io::Error> for FsError {
    fn from(val: io::Error) -> FsError {
        FsError::IOError(val)
    }
}

pub type Error = FsError;
pub type Result<T> = result::Result<T, FsError>;