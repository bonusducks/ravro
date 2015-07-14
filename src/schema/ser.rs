
use schema_model::*;
use serde::json::{self};
use super::error::{Error, ErrorCode};
use std::string::FromUtf8Error;


pub fn to_string(schema: &Schema) -> Result<String, FromUtf8Error> {
	// Do this the brute force way for now, let's get pretty later.
	match *schema {
		Schema::Null => Ok(format!("\"{}\"", "null")),
		Schema::String(ref s) => Ok(format!("\"{}\"", s.clone())),
		Schema::Array(ref v) => vec_to_string(&v),
		_ => Ok("".to_string()),
	}
}

// I never said this was memory efficient, I'll work on that later...
fn vec_to_string(schemas: &Vec<Schema>) -> Result<String, FromUtf8Error> {
	let mut first_comma = true;
	let mut str_vec = Vec::new();

	str_vec.push("[".to_string());

	for schema in schemas {
		let scheam_string = try!(to_string(schema));
		if !first_comma {
			str_vec.push(",".to_string());
		}
		str_vec.push(scheam_string);
		first_comma = false;
	}
	str_vec.push("]".to_string());

	Ok(str_vec.connect(""))
}