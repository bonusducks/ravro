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
    NotValidComplexType,
    NotValidArrayItemsType,
    NotValidType,
    ExpectedTypeAttribute,
    ExpectedFieldDefintion,
    ExpectedFieldTypeAttribute,
    ExpectedItemsAttribute,
    FieldDefaultTypeMismatch,
    FieldTooManyElementsOfSameType,
    UnknownFieldType,
    NotWellFormedName,
    FieldNameNotWellFormed,
    CannotNestArrays,
    JsonErrorCode(json::ErrorCode)
}

impl fmt::Debug for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Debug;

        match *self {
            ErrorCode::Unknown                  => "unexpected error".fmt(f),
            ErrorCode::NotValidPrimitiveType    => "not a valid primitiva data type".fmt(f),
            ErrorCode::NotValidComplexType      => "not a valid complex data type".fmt(f),
            ErrorCode::NotValidArrayItemsType   => "not a valid array items type".fmt(f),
            ErrorCode::NotValidType             => "not a valid type (i.e., integer, boolean)".fmt(f),
            ErrorCode::ExpectedTypeAttribute    => "expected type attribute for complex type".fmt(f),
            ErrorCode::ExpectedFieldDefintion   => "expected one or more record field definitions".fmt(f),
            ErrorCode::ExpectedFieldTypeAttribute    => "expected type attribute for field".fmt(f),
            ErrorCode::ExpectedItemsAttribute   => "expected items attribute for map type".fmt(f),
            ErrorCode::FieldDefaultTypeMismatch => "field default type does not match field type".fmt(f),
            ErrorCode::FieldTooManyElementsOfSameType => "field union type has too many elements of the same schema type".fmt(f),
            ErrorCode::UnknownFieldType         => "field type is unexpected/unknown".fmt(f),
            ErrorCode::NotWellFormedName        => "name is not valid/well formed".fmt(f),
            ErrorCode::FieldNameNotWellFormed   => "field name is not valid/well formed".fmt(f),
            ErrorCode::CannotNestArrays         => "arrays cannot be nested".fmt(f),
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

