use std::error;
use std::fmt;
use std::io;

#[derive(Clone, PartialEq)]
pub enum ErrorCode {
    Unknown,
    NotValidPrimitiveType,
    NotWellFormedName,
}

impl fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Debug;

        match *self {
            ErrorCode::Unknown                  => "Unexpected error".fmt(f),
            ErrorCode::NotValidPrimitiveType    => "Not a valid primitiva data type".fmt(f),
            ErrorCode::NotWellFormedName        => "Name is not valid/well formed".fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    /// msg, line, col
    SyntaxError(ErrorCode, usize, usize),
    IoError(io::Error),
    MissingAttributeError(&'static str),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::SyntaxError(..) => "syntax error",
            Error::IoError(ref error) => error::Error::description(error),
            Error::MissingAttributeError(_) => "missing required attribute",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref error) => Some(error),
            _ => None,
        }
    }

}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::SyntaxError(ref code, line, col) => {
                write!(fmt, "{:?} at line {} column {}", code, line, col)
            }
            Error::IoError(ref error) => fmt::Display::fmt(error, fmt),
            Error::MissingAttributeError(ref attribute) => {
                write!(fmt, "missing attribute {}", attribute)
            }
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::IoError(error)
    }
}

