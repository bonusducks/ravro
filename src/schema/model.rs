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
                self.is_primitive_type_name(s)
            },
            _ => false,
        }
    }

    fn is_primitive_type_name(&self, name: &str) -> bool {
        match name {
            "null" | "boolean" | "int" | "long" | "float" | "double" | "bytes" | "string" => true,
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
            _ => Err("This schema type doesn't support a fullname")
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
        debug!("is_valid({:?})", self);

        match *self {
            Schema::String(ref s) => Ok(try!(self.is_valid_schema_string(s))),
            Schema::Union(ref vec) => Ok(try!(self.is_valid_schema_union(vec))),
            Schema::Object(ref json_val) => Ok(try!(self.is_valid_schema_object(json_val))),
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

    fn is_valid_schema_object(&self, json_val: &Value) -> Result<(),Error> {
        debug!("is_valid_schema_object({:?})", json_val);

        if let Some(&Value::String(ref type_name)) = json_val.find("type") {
            match type_name.as_ref() {
                "record" => Ok(try!(self.is_valid_schema_record(json_val))),
                "array"  => Ok(try!(self.is_valid_array(json_val))),
                "map"    => Ok(try!(self.is_valid_map(json_val))),
                _ => {
                    // The object '{"type":"<primitive>"}' is a valid representation
                    // of a primitive type, so try to account for that here.
                    let type_result = self.is_valid_schema_string(type_name);
                    if type_result.is_ok() {
                        Ok(())
                    } else {
                        Err(Error::SyntaxError(ErrorCode::NotValidComplexType, 0, 0))
                    }
                }
            }
        } else {
            Err(Error::SyntaxError(ErrorCode::ExpectedTypeAttribute, 0, 0))
        }
    }

    fn is_valid_map(&self, json_val: &Value) -> Result<(),Error> {
        // Similar to array, we need to verify that there is a "values" attribute and
        // that it's value is either a string representing a primitive or named schmea,
        // or it's a full schema object, which needs to be validated recurisvely.
        debug!("is_valid_map({:?})", json_val);

        if let Some(values_val) = json_val.find("values") {
            match *values_val {
                Value::String(ref type_name) => {
                    Ok(try!(self.is_valid_schema_string(type_name)))
                },
                Value::Object(_) => {
                    let schema = Schema::Object(values_val.clone());
                    Ok(try!(schema.is_valid()))
                },
                _ => {
                    Err(Error::SyntaxError(ErrorCode::NotValidMapValuesType, 0, 0))
                }
            }
        } else {
            Err(Error::SyntaxError(ErrorCode::ExpectedValuesAttribute, 0, 0))
        }
    }

    fn is_valid_array(&self, json_val: &Value) -> Result<(),Error> {
        // All we can really do here is verify that there is an "items" attribute and
        // that it's value is either a string representing a primitive or named
        // schmea, or it's a full schmea object, which needs to be validated
        // unto itself.
        debug!("is_valid_array({:?})", json_val);

        if let Some(items_value) = json_val.find("items") {
            match *items_value {
                Value::String(ref type_name) => {
                    Ok(try!(self.is_valid_schema_string(type_name)))
                },
                Value::Object(_) => {
                    let schema = Schema::Object(items_value.clone());
                    Ok(try!(schema.is_valid()))
                },
                _ => {
                    Err(Error::SyntaxError(ErrorCode::NotValidArrayItemsType, 0, 0))
                }
            }
        } else {
            Err(Error::SyntaxError(ErrorCode::ExpectedItemsAttribute, 0, 0))
        }
    }

    fn is_valid_schema_record(&self, json_val: &Value) -> Result<(),Error> {
        let fn_result = self.fullname();
        if fn_result.is_err() {
            return Err(Error::SyntaxError(ErrorCode::NotWellFormedName, 0, 0));
        }

        if let Some(&Value::Array(ref value_vec)) = json_val.find("fields") {
            let mut record_ns = None;
            if let Some(&Value::String(ref ns)) = json_val.find("namespace") {
                record_ns = Some(ns);
            }

            Ok(try!(self.is_valid_schema_fields(value_vec, &record_ns)))
        } else {
            Err(Error::SyntaxError(ErrorCode::ExpectedFieldDefintion, 0, 0))
        }
    }

    fn is_valid_schema_fields(&self, value_vec: &Vec<Value>, record_ns: &Option<&String>) -> Result<(),Error> {
        for field_value in value_vec.iter() {
            try!(self.is_valid_schema_field(&field_value, record_ns));
        }
        Ok(())
    }

    fn is_valid_field_type(&self, field_type: &Value) -> Result<(),Error> {
        match *field_type {
            Value::String(ref s) => {
                Ok(try!(Schema::String(s.clone()).is_valid()))
            },
            Value::Array(ref value_vec) => {
                // For unions (which are represented by JSON arrays), we have the added
                // restriction that we can have only one "array" or "map" type within
                // the union.
                let mut array_count = 0;
                let mut map_count = 0;

                for value in value_vec.iter() {
                    try!(self.is_valid_field_type(&value));

                    if let Some(&Value::String(ref type_name)) = value.find("type") {
                        match type_name.as_ref() {
                            "array" => { array_count += 1; },
                            "map"   => { map_count += 1; },
                            _       => { /* don't care */ },
                        }
                    }
                }

                if array_count > 1 || map_count > 1 {
                    return Err(Error::SyntaxError(ErrorCode::FieldTooManyElementsOfSameType, 0, 0));
                }

                Ok(())
            },
            Value::Object(_) => {
                Ok(try!(Schema::Object(field_type.clone()).is_valid()))
            },
            _ => {
                return Err(Error::SyntaxError(ErrorCode::UnknownFieldType, 0, 0));
            },
        }
    }

    fn does_type_name_match_def_value(&self, type_name: &String, default_value: &Value) -> bool {
        let mut default_type_matches_field_type = false;
        if self.is_primitive_type_name(type_name) {
            match type_name.as_ref() {
                "string" => {
                    if let &Value::String(_) = default_value {
                        default_type_matches_field_type = true;
                    }
                },
                "int" | "long" => {
                    match *default_value {
                        Value::I64(_) | Value::U64(_) => {
                            default_type_matches_field_type = true;
                        }
                        _ => {}
                    }
                },
                "null" => {
                    if let &Value::Null = default_value {
                        default_type_matches_field_type = true;
                    }
                },
                "boolean" => {
                    if let &Value::Bool(_) = default_value {
                        default_type_matches_field_type = true;
                    }
                },
                "float" | "double" => {
                    if let &Value::F64(_) = default_value {
                        default_type_matches_field_type = true;
                    }
                },
                "bytes" => {
                    if let &Value::String(_) = default_value {
                        default_type_matches_field_type = true;
                    }
                },
                _ => {
                    // Can't get here, as is_primitive_type_name() limits the possible values of s. If
                    // we have, something has gone horribly, horribly wrong.
                    unreachable!();
                }
            }
        } else {
            // at this point, since I'm not currently caching previously
            // defined types, assume it's the name of a Schema::Object.

            // TODO: Verify that this type name actually exists.
            if let &Value::String(_) = default_value {
                default_type_matches_field_type = true;
            }
        }

        default_type_matches_field_type
    }

    // TODO: this is just checking type matching, it's not cheching range for number
    //       values. For bytes, ... it's not clear what the requirement is.
    fn is_valid_field_default(&self, field_value: &Value, field_type: &Value) -> Result<(),Error> {
        if let Some(default_value) = field_value.find("default") {
            let mut default_type_matches_field_type = false;
            match *field_type {
                Value::String(ref s) => {
                    // Should be a primitive type or a previously defined type.
                    default_type_matches_field_type = self.does_type_name_match_def_value(s, default_value);
                },
                Value::Object(_) => {
                    // TODO
                },
                Value::Array(ref array_vec) => {
                    // Only the first element of the vector is to be considered when determining the default type.
                    if let Some(first_element) = array_vec.first() {
                        // ... and, we have to do this dance again...
                        match *first_element {
                            Value::Array(_) => {
                                return Err(Error::SyntaxError(ErrorCode::CannotNestArrays, 0, 0));
                            },
                            Value::Object(_) => {
                                // TODO
                            },
                            Value::String(ref s) => {
                                default_type_matches_field_type =
                                    self.does_type_name_match_def_value(s, default_value);
                            },
                            _ => { return Err(Error::SyntaxError(ErrorCode::NotValidType, 0, 0)); }
                        }
                    }
                }
                _ => { return Err(Error::SyntaxError(ErrorCode::NotValidType, 0, 0)); },
            }

            if !default_type_matches_field_type {
                return Err(Error::SyntaxError(ErrorCode::FieldDefaultTypeMismatch, 0, 0));
            }
        }
        Ok(())
    }

    fn is_valid_schema_field(&self, field_value: &Value, record_ns: &Option<&String>) -> Result<(),Error> {
        try!(self.is_valid_field_name(field_value, record_ns));

        // Name was the easy one. Now we need to check the type.
        let field_type: &Value;
        if let Some(ref ft) = field_value.find("type") {
            field_type = ft;
        } else {
            return Err(Error::SyntaxError(ErrorCode::ExpectedFieldTypeAttribute, 0, 0));
        }
        try!(self.is_valid_field_type(field_type));

        // OK, that wasn't so bad actually. For defaults, we need to match the JSON type
        // to the Avro type.
        try!(self.is_valid_field_default(field_value, field_type));

        Ok(())
    }

    fn is_valid_field_name(&self, field_value: &Value, record_ns: &Option<&String>) -> Result<(),Error> {
        // Has to satisfy the same name requirements as a record name, if the field 
        // does not have a namespace in the name but record does, then the field 
        // uses the record namespace to construct the field fullname.

        // TODO : refactor so field and record fullname creation and verification are the same.
        let fullname: String;
        let ref name: String;
        let mut ns: &String = &String::from("");

        if let Some(&Value::String(ref name_value)) = field_value.find("name") {
            name = name_value;
        } else {
            return Err(Error::SyntaxError(ErrorCode::FieldNameNotWellFormed, 0, 0));
        }

        if let &Some(ref parent_ns) = record_ns {
            ns = parent_ns;
        }

        if name.contains(".") || ns.eq("") {
            fullname = name.clone();
        } else {
            fullname = format!("{}.{}", ns, name);
        }

        let check_result = self.check_name_segments(fullname);
        if check_result.is_err() {
            return Err(Error::SyntaxError(ErrorCode::FieldNameNotWellFormed, 0, 0));
        }

        Ok(())
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
