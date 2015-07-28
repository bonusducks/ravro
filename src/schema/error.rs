// Copyright 2015 Glenn McAllisters
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::error;
use std::fmt;
use std::io;
use serde::json::{self};

#[derive(Clone, PartialEq)]
pub enum ErrorCode {
    Unknown,
    NotValidPrimitiveType,
    NotWellFormedName,
    CannotNestArrays,
    JsonErrorCode(json::ErrorCode)
}

impl fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Debug;

        match *self {
            ErrorCode::Unknown                  => "Unexpected error".fmt(f),
            ErrorCode::NotValidPrimitiveType    => "Not a valid primitiva data type".fmt(f),
            ErrorCode::NotWellFormedName        => "Name is not valid/well formed".fmt(f),
            ErrorCode::CannotNestArrays         => "Arrays cannot be nested".fmt(f),
            ErrorCode::JsonErrorCode(ref ec)    => ec.fmt(f),
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

