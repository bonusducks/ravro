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

use super::model::Schema;
use super::error::*;
use serde::json::{self};

pub fn from_str(s: &str) -> Result<Schema, Error> {
    let s = s.trim();
    if s.starts_with("[") || s.starts_with("{") {
        let value = json::from_str(s);
        if value.is_ok() {
            Ok(Schema::Object(value.unwrap()))
        } else {
            // Translate the serde::json Error to our Error
            let err = value.unwrap_err();
            match err {
                json::Error::SyntaxError(se, line, col) => Err(Error::SyntaxError(ErrorCode::JsonErrorCode(se), line, col)),
                json::Error::IoError(ioe) => Err(Error::IoError(ioe)),
                json::Error::MissingFieldError(_) => {
                    // Don't have a corresponding match, really.
                    Err(Error::SyntaxError(ErrorCode::Unknown, 0, 0))
                }
            }
        }
    } else {
        if s.starts_with("\"") && s.ends_with("\"") {
            let s_without_quotes = s.trim_matches('"');
            Ok(Schema::String(String::from(s_without_quotes)))
        } else {
            Err(Error::SyntaxError(ErrorCode::NotValidPrimitiveType, 1, 0))
        }
    }
}

