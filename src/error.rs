use std::io;
use std::result;
use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum QuicError {
    Io(io::Error),
    ParseError,
    SerializeError
}

pub type Result<T> = result::Result<T, QuicError>;

impl Error for QuicError {
    fn description(&self) -> &str {
        match *self {
            QuicError::Io(ref err) => err.description(),
            QuicError::ParseError => "Error parsing packet",
            QuicError::SerializeError => "Error serializing packet"
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            QuicError::Io(ref err) => Some(err),
            QuicError::ParseError => None,
            QuicError::SerializeError => None,
        }
    }
}

impl From<io::Error> for QuicError {
    fn from(err: io::Error) -> QuicError {
        QuicError::Io(err)
    }
}

impl fmt::Display for QuicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QuicError::Io(ref err) => err.fmt(f),
            QuicError::ParseError => write!(f, "Error parsing packet"),
            QuicError::SerializeError => write!(f, "Error serializing packet"),
        }
    }
}