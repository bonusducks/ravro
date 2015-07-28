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

use std::fmt;

use regex::Regex;
use serde::json::Value;
use serde::json::builder::ObjectBuilder;
use serde::json::ser::to_string;

use super::error::*;

#[derive(Clone, PartialEq, Debug)]
pub enum Schema {
    Null,
    String(String),
    Union(Vec<Schema>),
    Object(Value)
}

pub enum FieldSortOrder {
    Ascending,
    Descending,
    Ignore,
}

impl<'a> From<&'a FieldSortOrder> for String {
    fn from(s: &'a FieldSortOrder) -> String {
        match *s {
            FieldSortOrder::Ascending  => String::from("ascending"),
            FieldSortOrder::Descending => String::from("descending"),
            FieldSortOrder::Ignore     => String::from("ignore"),
        }
    }
}

impl fmt::Display for FieldSortOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

impl Schema {
    pub fn is_primitive(&self) -> bool {
        match *self {
            Schema::String(ref s) => {
                match s.as_ref() {
                    "null" | "boolean" | "int" | "long" | "float" | "double" | "bytes" | "string" => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match *self {
            Schema::Object(_) => true,
            _ => false,
        }
    }

    pub fn is_null(&self) -> bool {
        match *self {
            Schema::Null => true,
            _ => false,
        }
    }

    pub fn is_union(&self) -> bool {
        match *self {
            Schema::Union(_)   => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        self.is_complex_type("array")
    }

    pub fn is_record(&self) -> bool {
        self.is_complex_type("record")
    }

    pub fn is_enum(&self) -> bool {
        self.is_complex_type("enum")
    }

    pub fn is_map(&self) -> bool {
        self.is_complex_type("map")
    }

    pub fn is_fixed(&self) -> bool {
        self.is_complex_type("fixed")
    }

    fn is_complex_type(&self, type_name: &str) -> bool {
        match *self {
            Schema::Object(ref value) => {
                if let Some(&Value::String(ref t)) = value.find("type") {
                    t == type_name
                } else {
                    false
                }
            },
            _ => false
        }
    }

    /// Returns a new Schema representing the instance as a Schema::Object.
    /// If it's already a Schema::Object instance, a copy is returned.
    pub fn as_object(&self) -> Option<Schema> {
        match *self {
            Schema::Object(_) => Some(self.clone()),
            Schema::String(ref s) => {
                let value = ObjectBuilder::new()
                    .insert(String::from("type"), s.clone())
                    .unwrap();
                Some(Schema::Object(value))
            },
            Schema::Union(ref vec) => {
                let val_array = vec.iter()
                    .map(|e|
                        if let Some(Schema::Object(v)) = e.as_object() {
                            v
                        } else {
                            Value::String(String::from("null")) // really shouldn't get here...
                        }
                    )
                    .collect();
                Some(Schema::Object(Value::Array(val_array)))
            },
            Schema::Null => {
                let value = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("null"))
                    .unwrap();
                Some(Schema::Object(value))
            }
        }
    }

    pub fn fullname(&self) -> Result<String, &'static str> {
        match *self {
            Schema::Object(_) => {
                let fname = try!(self.valid_fullname());
                Ok(fname)
            }
            _ => Err("This schmea type doesn't support a fullname")
        }
    }

    fn object_fullname(&self, value: &Value) -> String {
        if let Some(&Value::String(ref name_value)) = value.find("name") {
            if let Some(&Value::String(ref ns_value)) = value.find("namespace") {
                if name_value.contains(".") {
                    name_value.clone()
                } else {
                    format!("{}.{}", ns_value, name_value)
                }
            } else {
                name_value.clone()
            }
        } else {
            String::from("")
        }
    }

    fn valid_fullname(&self) -> Result<String, &'static str> {
        match *self {
            Schema::Object(ref value) => {
                let fname = self.object_fullname(value);
                Ok(try!(self.check_name_segments(fname)))
            },
            _ => Err("Not a valid Schema type")
        }
    }

    fn check_name_segments(&self, name: String) -> Result<String, &'static str> {
        let segment_re = Regex::new(r#"^[A-Za-z_][A-Za-z0-9_]*$"#).unwrap();

        // yeah, I could have done this as for loop and bailed early
        let valid_name = name.split('.').fold(true, |valid, segment|
            valid && segment_re.is_match(&segment)
        );

        if valid_name {
            Ok(name)
        } else {
            Err("Name is not well formed")
        }
    }

    pub fn doc(&self) -> Option<&String> {
        match *self {
            Schema::Object(ref value) => {
                if let Some(&Value::String(ref doc)) = value.find("doc") {
                    Some(doc)
                } else {
                    None
                }
            },
            _ => None
        }
    }

    pub fn aliases(&self) -> Option<Vec<String>> {
        match *self {
            Schema::Object(ref value) => {
                if let Some(&Value::Array(ref value_vec)) = value.find("aliases") {
                    let mut alias_vec = Vec::new();
                    for value in value_vec {
                        match *value {
                            Value::String(ref s) => { alias_vec.push(s.clone()); }
                            _ => (),
                        }
                    }
                    Some(alias_vec)
                } else {
                    None
                }
            },
            _ => None
        }
    }

    pub fn symbols(&self) -> Option<Vec<String>> {
        match *self {
            Schema::Object(ref value) => {
                if let Some(&Value::Array(ref value_vec)) = value.find("symbols") {
                    let mut alias_vec = Vec::new();
                    for value in value_vec {
                        match *value {
                            Value::String(ref s) => { alias_vec.push(s.clone()); }
                            _ => (),
                        }
                    }
                    Some(alias_vec)
                } else {
                    None
                }
            },
            _ => None
        }
    }

    // Get the raw fields in a record
    pub fn fields(&self) -> Option<&Vec<Value>> {
        match *self {
            Schema::Object(ref value) => {
                if let Some(&Value::Array(ref array)) = value.find("fields") {
                    Some(&*array)
                } else {
                    None
                }
            }
            _ => None
        }
    }

    pub fn size(&self) -> Option<u64> {
        match *self {
            Schema::Object(ref value) => {
                if let Some(&Value::U64(size)) = value.find("size") {
                    Some(size)
                } else {
                    None
                }
            },
            _ => None
        }
    }

    pub fn is_valid(&self) -> Result<(),Error> {
        // get the schmea in it's most general form,
        // determine it's type, and then perform the appropriate
        // validation. If it's a union, do this for each element 
        // of the union.

        match *self {
            Schema::String(ref s) => Ok(try!(self.is_valid_schema_string(s))),
            Schema::Union(ref vec) => Ok(try!(self.is_valid_schema_union(vec))),
            Schema::Object(ref obj) => Ok(try!(self.is_valid_schema_object(obj))),
            _ => Err(Error::SyntaxError(ErrorCode::Unknown, 0, 0)) // catch all punt
        }
    }

    fn is_valid_schema_string(&self, s: &String) -> Result<(),Error> {
        // There are two cases here. One is that the string is a primitive type name.
        // The other is that the string refers to a previously defined schema name.
        // So it has to satify the same requirements as the the fullname name segments.
        if self.is_primitive() {
            Ok(())
        } else {
            let result = self.check_name_segments(s.clone());
            if result.is_ok() {
                Ok(())
            } else {
                Err(Error::SyntaxError(ErrorCode::NotWellFormedName, 0, 0))
            }
        }
    }

    fn is_valid_schema_union(&self, vec: &Vec<Schema>) -> Result<(),Error> {
        Err(Error::SyntaxError(ErrorCode::Unknown, 0, 0)) // catch all punt
    }

    fn is_valid_schema_object(&self, obj: &Value) -> Result<(),Error> {
        Err(Error::SyntaxError(ErrorCode::Unknown, 0, 0)) // catch all punt
    }
}

// When converting to string, keep it to JSON but the more compact style.
// For example, "string" and {"type":"string"} are always equivalent types,
// so we'll prefer "string" for compactness of representation.
impl<'a> From<&'a Schema> for String {
    fn from(s: &'a Schema) -> String {
        match *s {
            Schema::Null => String::from("\"null\""),
            Schema::String(ref s) => format!("\"{}\"", s.clone()),
            Schema::Object(ref v) => {
                let result = to_string(&v);
                if result.is_ok() {
                    result.unwrap()
                } else {
                    String::from("")
                }
            },
            Schema::Union(ref vec) => {
                let string_vec : Vec<String> = vec.iter().map(|schema| String::from(schema) ).collect();
                format!("[{}]", string_vec.connect(","))
            }
        }
    }
}

impl fmt::Display for Schema {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
