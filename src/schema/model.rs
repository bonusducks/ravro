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

use regex::Regex;
use serde::json::Value;
use serde::json::builder::ObjectBuilder;
use serde::json::ser::to_string;

#[derive(Clone, PartialEq, Debug)]
pub enum Schema {
    Null,
    String(String),
    Array(Vec<Schema>),
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

    pub fn is_array(&self) -> bool {
        match *self {
            Schema::Array(_)   => true,
            Schema::Object(ref value) => {
                // There is a complex type 'array', and the, err, I guess native array type.
                // This is the complex type.
                if let Some(&Value::String(ref t)) = value.find("type") {
                    t == "array"
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn is_object(&self) -> bool {
        match *self {
            Schema::Object(_) => true,
            _ => false,
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
            Schema::Array(ref v) => {
                let val_array = v.iter()
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

    pub fn is_null(&self) -> bool {
        match *self {
            Schema::Null => true,
            _ => false,
        }
    }

    pub fn is_record(&self) -> bool {
        match *self {
            Schema::Object(ref value) => {
                if let Some(&Value::String(ref t)) = value.find("type") {
                    t == "record"
                } else {
                    false
                }
            },
            _ => false
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
                let segment_re = Regex::new(r#"^[A-Za-z_][A-Za-z0-9_]*$"#).unwrap();

                // yeah, I could have done this as for loop and bailed early
                let valid_fname = fname.split('.').fold(true, |valid, segment|
                    valid && segment_re.is_match(&segment)
                );

                if valid_fname {
                    Ok(fname)
                } else {
                    Err("Name is not well formed")
                }
            },
            _ => Err("Not a valid Schema type")
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

    pub fn is_enum(&self) -> bool {
        match *self {
            Schema::Object(ref value) => {
                if let Some(&Value::String(ref t)) = value.find("type") {
                    t == "enum"
                } else {
                    false
                }
            },
            _ => false
        }
    }
}

impl ToString for Schema {
    fn to_string(&self) -> String {
        String::from(self)
    }
}

impl<'a> From<&'a Schema> for String {
    fn from(s: &'a Schema) -> String {
        if let Some(Schema::Object(ref v)) = s.as_object() {
            let r = to_string(&v);
            if r.is_ok() {
                r.unwrap()
            } else {
                String::from("")
            }
        } else {
            String::from("")
        }
    }
}
