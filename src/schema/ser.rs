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