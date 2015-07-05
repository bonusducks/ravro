
use schema_model::*;
use serde::json::{self};
use super::error::{Error, ErrorCode};

/// Decodes an Avro schema from a `&str`.
pub fn from_str(s: &str) -> Result<Schema, Error>  
{
    if s.starts_with("{") || s.starts_with("[") {
        let res : Schema = json::from_str(&s).unwrap();
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
        let res = Schema::new(&s);
        if res.is_err() { return Err(Error::SyntaxError(ErrorCode::NotValidPrimitiveType, 0, 0)) }
        Ok(res.unwrap())
    }
}
