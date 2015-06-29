
use schema_model::*;

pub enum Error {
    InvalidSchema,
}

/// Decodes an Avro schema from a `&str`.
pub fn from_str(s: &str) -> Result<Schema, Error>  
{
    if s.starts_with("{") || s.starts_with("[") {
        // JSON object or array
        // dummy code for now.
        Ok(Schema::new(&"int").unwrap())
    } else {
        let res = Schema::new(&s);
        if res.is_err() { return Err(Error::InvalidSchema) }
        Ok(res.unwrap())
    }
}
