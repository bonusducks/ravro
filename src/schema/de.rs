
use schema_model::*;
use serde::json::{self};
use super::error::{Error, ErrorCode};

/// Decodes an Avro schema from a `&str`.
pub fn from_str(s: &str) -> Result<Schema, Error>  
{
    if s.starts_with("{") || s.starts_with("[") {
        let res : Schema = json::from_str(&s).unwrap();
        if res.is_valid().is_err() {
            // TODO: propogate validation error
            return Err(Error::SyntaxError(ErrorCode::NotValidPrimitiveType, 0, 0));
        }
        Ok(res)
    } else {
        let res = Schema::new(&s);
        if res.is_err() { return Err(Error::SyntaxError(ErrorCode::NotValidPrimitiveType, 0, 0)) }
        Ok(res.unwrap())
    }
}
