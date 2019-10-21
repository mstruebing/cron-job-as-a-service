// stdlib
use std::{fmt, io, result};

// modules
use dotenv;

pub type Result<T, E = Error> = result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Dotenv(dotenv::Error),
    Diesel(diesel::result::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            IO(err) => write!(fmt, "IO error ({})", err),
            Dotenv(err) => write!(fmt, "Dotenv error ({})", err),
            Diesel(err) => write!(fmt, "Diesel error ({})", err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<dotenv::Error> for Error {
    fn from(err: dotenv::Error) -> Self {
        Error::Dotenv(err)
    }
}
