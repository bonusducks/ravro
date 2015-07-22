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

use schema_model::*;
use serde::json::{self};
use super::error::{Error, ErrorCode};

/// Decodes an Avro SchemaOld from a `&str`.
pub fn from_str(s: &str) -> Result<SchemaOld, Error>  
{
    if s.starts_with("{") || s.starts_with("[") {
        let res : SchemaOld = json::from_str(&s).unwrap();
        let is_valid = res.is_valid();

        if is_valid.is_err() {
            let err_str = is_valid.unwrap_err();
            let mut err_code : ErrorCode;

            // Brittle
            match err_str {
                "Name cannot be empty" | 
                "Name is not well formed" |
                "Records must have a name"      => err_code = ErrorCode::NotWellFormedName,
                "Not a valid primitive type"    => err_code = ErrorCode::NotValidPrimitiveType,
                _                               => err_code = ErrorCode::Unknown,
            }

            return Err(Error::SyntaxError(err_code, 0, 0));
        }
        Ok(res)
    } else {
        let res = SchemaOld::new(&s);
        if res.is_err() { return Err(Error::SyntaxError(ErrorCode::NotValidPrimitiveType, 0, 0)) }
        Ok(res.unwrap())
    }
}
