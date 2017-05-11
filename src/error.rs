use std::io;
use std::result;
use std::error::Error;
use std::fmt;
use std::string;


#[derive(Debug)]
pub enum QuicError {
    Io(io::Error),
    ParseError,
    SerializeError,
    FromUtf8Error(string::FromUtf8Error),
    PacketTooLarge,
}

pub type Result<T> = result::Result<T, QuicError>;

impl Error for QuicError {
    fn description(&self) -> &str {
        match *self {
            QuicError::Io(ref err) => err.description(),
            QuicError::FromUtf8Error(ref err) => err.description(),
            QuicError::ParseError => "Error parsing packet",
            QuicError::SerializeError => "Error serializing packet",
            QuicError::PacketTooLarge => "Packet too large",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            QuicError::Io(ref err) => Some(err),
            QuicError::FromUtf8Error(ref err) => Some(err),
            QuicError::ParseError | QuicError::SerializeError | QuicError::PacketTooLarge => None,
        }
    }
}

impl From<io::Error> for QuicError {
    fn from(err: io::Error) -> QuicError {
        QuicError::Io(err)
    }
}

impl From<string::FromUtf8Error> for QuicError {
    fn from(err: string::FromUtf8Error) -> QuicError {
        QuicError::FromUtf8Error(err)
    }
}

impl fmt::Display for QuicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QuicError::Io(ref err) => err.fmt(f),
            QuicError::FromUtf8Error(ref err) => err.fmt(f),
            QuicError::ParseError => write!(f, "Error parsing packet"),
            QuicError::SerializeError => write!(f, "Error serializing packet"),
            QuicError::PacketTooLarge => write!(f, "Packet too large"),
        }
    }
}