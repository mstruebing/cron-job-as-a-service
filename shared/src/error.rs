// stdlib
use std::{fmt, io, result};

// modules
use diesel;
use dotenv;
// TODO: Do I really need this full dependency only to implement the error?
use r2d2;

pub type Result<T, E = Error> = result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Dotenv(dotenv::Error),
    Diesel(diesel::result::Error),
    R2d2(r2d2::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            IO(err) => write!(fmt, "IO error ({})", err),
            Dotenv(err) => write!(fmt, "Dotenv error ({})", err),
            Diesel(err) => write!(fmt, "Diesel error ({})", err),
            R2d2(err) => write!(fmt, "R2d2 error ({})", err),
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

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::Diesel(err)
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Self {
        Error::R2d2(err)
    }
}
