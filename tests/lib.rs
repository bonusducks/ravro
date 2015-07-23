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

// Integration style tests
extern crate ravro;
extern crate serde;

/*
mod enum_type {
    mod fullname {
        use ravro::schema::SchemaOld;

        #[test]
        fn must_have_name() {
            let enum_result = SchemaOld::new_enum("", &vec!["A1"]);
            assert!(enum_result.is_err());
            assert_eq!(enum_result.unwrap_err(), "Enums must have a name");
        }

        // Enums share the same code paths for the rest of fullname as records, so
        // I'm not going to repeat all the tests.
    }

    mod ser {

    }

    mod des {

    }
}
*/

/*
mod record {
    mod fullname {
        use ravro::schema::SchemaOld;

        #[test]
        fn must_have_name() {
            let rec_result = SchemaOld::new_rec("", &Vec::new());
            assert!(rec_result.is_err());
            assert_eq!(rec_result.unwrap_err(), "Records must have a name");
        }

        #[test]
        fn well_formed_name() {
            let rec_result = SchemaOld::new_rec("foo_bar_123", &Vec::new());
            assert!(rec_result.is_ok());

            let rec = rec_result.unwrap();
            assert_eq!("foo_bar_123", &rec.fullname().unwrap());
        }

        #[test]
        fn invalid_name() {
            let rec_result = SchemaOld::new_rec("foo_bar_%%%", &Vec::new());
            assert!(rec_result.is_err());

            let err = rec_result.unwrap_err();
            assert!(err.contains("not well formed"));
        }

        #[test]
        fn name_cant_end_with_period() {
            let rec_result = SchemaOld::new_rec("foo.", &Vec::new());
            assert!(rec_result.is_err());

            let err = rec_result.unwrap_err();
            assert!(err.contains("not well formed"));
        }

        #[test]
        fn name_cant_start_with_period() {
            let rec_result = SchemaOld::new_rec(".foo", &Vec::new());
            assert!(rec_result.is_err());

            let err = rec_result.unwrap_err();
            assert!(err.contains("not well formed"));
        }

        #[test]
        fn name_is_fullname() {
            let rec_result = SchemaOld::new_rec("x.y.foo", &Vec::new());
            assert!(rec_result.is_ok());

            let rec = rec_result.unwrap();
            assert_eq!("x.y.foo", &rec.fullname().unwrap());
        }

        #[test]
        fn name_and_namespace() {
            let rec_type = SchemaOld::new_rec_full("foo",&Vec::new(),"x.y","",&Vec::new()).unwrap();
            let fullname = rec_type.fullname().unwrap();

            assert_eq!("x.y.foo", fullname);
        }

        #[test]
        fn ignore_namespace() {
            let rec_type = SchemaOld::new_rec_full("x.y.foo",&Vec::new(),"a.b","",&Vec::new()).unwrap();
            let fullname = rec_type.fullname().unwrap();

            assert_eq!("x.y.foo", fullname);
        }

        #[test]
        fn namespace_cant_start_with_period() {
            let rec_type_result = SchemaOld::new_rec_full("foo",&Vec::new(),".x.y","",&Vec::new());
            assert!(rec_type_result.is_err());

            let err = rec_type_result.unwrap_err();
            assert!(err.contains("not well formed"));
        }

        #[test]
        fn namespace_cant_end_with_period() {
            let rec_type_result = SchemaOld::new_rec_full("foo",&Vec::new(),"x.y.","",&Vec::new());
            assert!(rec_type_result.is_err());

            let err = rec_type_result.unwrap_err();
            assert!(err.contains("not well formed"));
        }
    }

    mod ser {
        use ravro::schema::{self, SchemaOld, Field};
        use serde::json::{self, Value};

        #[test]
        fn record_type_1() {
            // This is about the simplest record serialization test I can think of
            let rec_type = schema::from_str(&r#"{"type":"record", "name":"foo", "fields":[]}"#).unwrap();
            let ser_rec_type = json::to_string(&rec_type).unwrap();

            assert_eq!(ser_rec_type, r#"{"type":"record","name":"foo","namespace":null,"doc":null,"aliases":[],"fields":[]}"#)
        }

        #[test]
        fn record_type_2() {
            let rec_type = schema::from_str(&r#"{"fields":[], "type":"record", "name":"foo", "namespace":"x.y"}"#).unwrap();
            let ser_rec_type = json::to_string(&rec_type).unwrap();

            assert_eq!(ser_rec_type, r#"{"type":"record","name":"foo","namespace":"x.y","doc":null,"aliases":[],"fields":[]}"#)
        }

        
        #[test]
        fn record_type_3() {
            let json_str = r#"{ "type" : "record", 
                                "name" : "foo", 
                                "fields" : [{"type":"boolean", "name":"f1"}]
                              }"#;
            let rec_type = schema::from_str(&json_str).unwrap();
            let ser_rec_type = json::to_string_pretty(&rec_type).unwrap();

            // Single line is getting hard to read.
            assert_eq!(ser_rec_type, 
r#"{
  "type": "record",
  "name": "foo",
  "namespace": null,
  "doc": null,
  "aliases": [],
  "fields": [
    {
      "name": "f1",
      "type": "boolean",
      "default": null,
      "doc": null,
      "order": null,
      "aliases": null
    }
  ]
}"#);
        }

        #[test]
        fn record_type_4() {
            // this string and the one in test record_type_3() are equivalent.
            // problem is, they won't acutally compare as equal at the moment.
            let json_str = r#"{ "type" : "record",
                                "name" : "foo",
                                "fields" : [{"type":{"type":"boolean"}, "name":"f1"}]
                              }"#;
            let rec_type = schema::from_str(&json_str).unwrap();
            let ser_rec_type = json::to_string_pretty(&rec_type).unwrap();

            // Single line is getting hard to read.
            assert_eq!(ser_rec_type,
r#"{
  "type": "record",
  "name": "foo",
  "namespace": null,
  "doc": null,
  "aliases": [],
  "fields": [
    {
      "name": "f1",
      "type": {
        "type": "boolean"
      },
      "default": null,
      "doc": null,
      "order": null,
      "aliases": null
    }
  ]
}"#);
        }

        #[test]
        fn record_type_5() {
            let json_str = r#"{ "type" : "record", 
                                "name" : "foo", 
                                "fields" : [
                                    {"type":"boolean", "name":"f1", "default": false},
                                    {"name":"f2", "type":"int", "aliases":["a","b"]}
                                ]
                              }"#;
            let rec_type = schema::from_str(&json_str).unwrap();
            let ser_rec_type = json::to_string_pretty(&rec_type).unwrap();

            // Single line is getting hard to read.
            assert_eq!(ser_rec_type, 
r#"{
  "type": "record",
  "name": "foo",
  "namespace": null,
  "doc": null,
  "aliases": [],
  "fields": [
    {
      "name": "f1",
      "type": "boolean",
      "default": false,
      "doc": null,
      "order": null,
      "aliases": null
    },
    {
      "name": "f2",
      "type": "int",
      "default": null,
      "doc": null,
      "order": null,
      "aliases": [
        "a",
        "b"
      ]
    }
  ]
}"#);
        }

        // temporary tests to work out fields...
        #[test]
        fn field_type_1() {
            let field = Field::new("f1", "boolean").unwrap();
            let ser_field = json::to_string(&field).unwrap();

            assert_eq!(ser_field, r#"{"name":"f1","type":"boolean","default":null,"doc":null,"order":null,"aliases":null}"#);
        }

        #[test]
        fn field_type_2() {
            let field = Field::new_full("f1", "boolean", &None, "", "", &Vec::new()).unwrap();
            let ser_field = json::to_string(&field).unwrap();

            assert_eq!(ser_field, r#"{"name":"f1","type":"boolean","default":null,"doc":null,"order":null,"aliases":null}"#);
        }

        #[test]
        fn field_type_3() {
            let field = Field::new_full("f1", "boolean", &Some(Value::Bool(true)), "some docs", "descending", &Vec::new()).unwrap();
            let ser_field = json::to_string(&field).unwrap();

            assert_eq!(ser_field, r#"{"name":"f1","type":"boolean","default":true,"doc":"some docs","order":"descending","aliases":null}"#);
        }

        #[test]
        fn field_type_4() {
            let mut aliases : Vec<String> = Vec::new();
            aliases.push("a".to_string());
            aliases.push("b".to_string());

            let field = Field::new_full("f1", "boolean", &Some(Value::Bool(false)), "", "", &aliases).unwrap();
            let ser_field = json::to_string(&field).unwrap();

            assert_eq!(ser_field, r#"{"name":"f1","type":"boolean","default":false,"doc":null,"order":null,"aliases":["a","b"]}"#);
        }
    }

    mod des {
        use ravro::schema::{self, SchemaOld, Field};
        use serde::json::{Value};

        #[test]
        fn record_type_1() {
            let des_rec_type = schema::from_str(&r#"{"fields":[], "type":"record", "name":"foo"}"#).unwrap();
            let rec_type = SchemaOld::new_rec("foo",&Vec::new()).unwrap();

            assert_eq!(rec_type, des_rec_type);
        }

        #[test]
        fn record_type_2() {
            let des_rec_type = schema::from_str(&r#"{"fields":[], "type":"record", "name":"foo", "namespace":"x.y"}"#).unwrap();
            let rec_type = SchemaOld::new_rec_full("foo",&Vec::new(),"x.y","",&Vec::new()).unwrap();

            assert_eq!(rec_type, des_rec_type);
        }

        #[test]
        fn bad_name() {
            let des_rec_result = schema::from_str(&r#"{"fields":[], "type":"record", "name":"foo_%%", "namespace":"x.y"}"#);
            assert!(des_rec_result.is_err());

            let err = des_rec_result.unwrap_err();
            let err_msg = format!("{:?}", err); // there's got to be a better way to do this.
            assert!(err_msg.contains("Name is not valid/well formed"));
        }

        #[test]
        fn record_type_3() {
            let json_str = r#"{ "type" : "record", 
                                "name" : "foo", 
                                "fields" : [{"type":"boolean", "name":"f1"}]
                              }"#;
            let des_rec_type = schema::from_str(&json_str).unwrap();
            let field = Field::new("f1", "boolean").unwrap();
            let field_vec = vec![field];
            let rec_type = SchemaOld::new_rec_full("foo", &field_vec, "", "", &Vec::new()).unwrap();

            assert_eq!(rec_type, des_rec_type);
        }

        #[test]
        fn record_type_5() {
            let json_str = r#"{ "type" : "record", 
                                "name" : "foo", 
                                "fields" : [
                                    {"type":"boolean", "name":"f1", "default":false},
                                    {"name":"f2", "type":"int", "aliases":["a","b"]}
                                ]
                              }"#;
            let des_rec_type = schema::from_str(&json_str).unwrap();

            let field1 = Field::new_full("f1", "boolean", &Some(Value::Bool(false)), "", "", &Vec::new()).unwrap();
            let mut alias_vec : Vec<String> = Vec::new();
            alias_vec.push("a".to_string());
            alias_vec.push("b".to_string());
            let field2 = Field::new_full("f2", "int", &None, "", "", &alias_vec).unwrap();
            let field_vec = vec![field1, field2];
            let rec_type = SchemaOld::new_rec_full("foo", &field_vec, "", "", &Vec::new()).unwrap();

            assert_eq!(rec_type, des_rec_type);
        }
    }
}
*/

mod primitive {
    mod is_primitive {
        use ravro::schema::Schema;
        use serde::json::{self, Value};

        #[test]
        fn is_bool() {
            let s = Schema::String(String::from("boolean"));
            assert!(s.is_primitive());
        }

        #[test]
        fn is_null() {
            let s = Schema::String(String::from("null"));
            assert!(s.is_primitive());
        }

        #[test]
        fn is_int() {
            let s = Schema::String(String::from("int"));
            assert!(s.is_primitive());
        }

        #[test]
        fn is_long() {
            let s = Schema::String(String::from("long"));
            assert!(s.is_primitive());
        }

        #[test]
        fn is_float() {
            let s = Schema::String(String::from("float"));
            assert!(s.is_primitive());
        }

        #[test]
        fn is_double() {
            let s = Schema::String(String::from("double"));
            assert!(s.is_primitive());
        }

        #[test]
        fn is_bytes() {
            let s = Schema::String(String::from("bytes"));
            assert!(s.is_primitive());
        }

        #[test]
        fn is_string() {
            let s = Schema::String(String::from("string"));
            assert!(s.is_primitive());
        }

        #[test]
        fn is_not_primitive_string() {
            let s = Schema::String(String::from("bogus"));
            assert_eq!(s.is_primitive(), false);
        }

        #[test]
        fn array_is_not_primitive() {
            let a = Schema::Array(vec!());
            assert_eq!(a.is_primitive(), false);
        }

        #[test]
        fn object_is_not_primitive() {
            let val : Value = json::from_str(r#"{"type":"string"}"#).unwrap(); // about as simple an object as we can get
            let s = Schema::Object(val);

            assert_eq!(s.is_primitive(), false);
        }

        #[test]
        fn null_is_not_primitive() {
            let n = Schema::Null;
            assert_eq!(n.is_primitive(), false);
        }
    }

    /*
    mod fullname {
        use ravro::schema::SchemaOld;

        #[test]
        fn int_fullname() {
            let s = SchemaOld::new("int").unwrap();
            assert_eq!("int", s.fullname().unwrap());
        }
    }

    mod convert {
        use ravro::schema::PrimitiveTypeEnum;

        #[test]
        fn null_to_from_str() {
            let null_raw = "null";
            assert_eq!(null_raw, PrimitiveTypeEnum::Null.to_str());
            assert_eq!(PrimitiveTypeEnum::Null, PrimitiveTypeEnum::from_str(&null_raw).unwrap());
        }

        #[test]
        fn boolean_to_from_str() {
            let boolean_raw = "boolean";
            assert_eq!(boolean_raw, PrimitiveTypeEnum::Boolean.to_str());
            assert_eq!(PrimitiveTypeEnum::Boolean, PrimitiveTypeEnum::from_str(&boolean_raw).unwrap());
        }

        #[test]
        fn int_to_from_str() {
            let int_raw = "int";
            assert_eq!(int_raw, PrimitiveTypeEnum::Int.to_str());
            assert_eq!(PrimitiveTypeEnum::Int, PrimitiveTypeEnum::from_str(&int_raw).unwrap());        
        }

        #[test]
        fn long_to_from_str() {
            let long_raw = "long";
            assert_eq!(long_raw, PrimitiveTypeEnum::Long.to_str());
            assert_eq!(PrimitiveTypeEnum::Long, PrimitiveTypeEnum::from_str(&long_raw).unwrap());        
        }

        #[test]
        fn float_to_from_str() {
            let float_raw = "float";
            assert_eq!(float_raw, PrimitiveTypeEnum::Float.to_str());
            assert_eq!(PrimitiveTypeEnum::Float, PrimitiveTypeEnum::from_str(&float_raw).unwrap());        
        }

        #[test]
        fn double_to_from_str() {
            let double_raw = "double";
            assert_eq!(double_raw, PrimitiveTypeEnum::Double.to_str());
            assert_eq!(PrimitiveTypeEnum::Double, PrimitiveTypeEnum::from_str(&double_raw).unwrap());        
        }

        #[test]
        fn bytes_to_from_str() {
            let bytes_raw = "bytes";
            assert_eq!(bytes_raw, PrimitiveTypeEnum::Bytes.to_str());
            assert_eq!(PrimitiveTypeEnum::Bytes, PrimitiveTypeEnum::from_str(&bytes_raw).unwrap());
        }

        #[test]
        fn string_to_from_str() {
            let string_raw = "string";
            assert_eq!(string_raw, PrimitiveTypeEnum::String.to_str());
            assert_eq!(PrimitiveTypeEnum::String, PrimitiveTypeEnum::from_str(&string_raw).unwrap());
        }

        #[test]
        fn bogus_to_from_str() {
            let bogus_raw = "bogus";
            assert_eq!(None, PrimitiveTypeEnum::from_str(&bogus_raw));        
        }
    }
    */

    mod ser {
        use ravro::schema::{self, Schema};

        #[test]
        fn null_type() {
            // Note that the "null" primitive type isn't the same as Schema::Null
            let n = Schema::String(String::from("null"));
            let s = schema::to_string(&n).unwrap();

            assert_eq!(s, String::from(r#""null""#));
        }

        
        #[test]
        fn boolean_type() {
            let b = Schema::String(String::from("boolean"));
            let s = schema::to_string(&b).unwrap();

            assert_eq!(s, String::from(r#""boolean""#));
        }

        #[test]
        fn int_type() {            
            let b = Schema::String(String::from("int"));
            let s = schema::to_string(&b).unwrap();

            assert_eq!(s, String::from(r#""int""#));
        }

        #[test]
        fn long_type() {
            let b = Schema::String(String::from("long"));
            let s = schema::to_string(&b).unwrap();

            assert_eq!(s, String::from(r#""long""#));
        }

        #[test]
        fn float_type() {
            let b = Schema::String(String::from("float"));
            let s = schema::to_string(&b).unwrap();

            assert_eq!(s, String::from(r#""float""#));
        }

        #[test]
        fn double_type() {
            let b = Schema::String(String::from("double"));
            let s = schema::to_string(&b).unwrap();

            assert_eq!(s, String::from(r#""double""#));
        }

        #[test]
        fn bytes_type() {
            let b = Schema::String(String::from("bytes"));
            let s = schema::to_string(&b).unwrap();

            assert_eq!(s, String::from(r#""bytes""#));
        }

        #[test]
        fn string_type() {
            let b = Schema::String(String::from("string"));
            let s = schema::to_string(&b).unwrap();

            assert_eq!(s, String::from(r#""string""#));
        }
    }
    /*

    mod des {
        use ravro::schema::{self, SchemaOld};

        #[test]
        fn null_type() {
            let null_type = schema::from_str(&"null").unwrap();
            let null_type_2 = schema::from_str(&r#"{"type":"null"}"#).unwrap();
            let des_null_type  = SchemaOld::new(&"null").unwrap();

            assert_eq!(des_null_type, null_type);
            assert_eq!(des_null_type, null_type_2);
            assert_eq!(null_type, null_type_2);

            assert!(des_null_type.is_valid().is_ok());
        }

        #[test]
        fn boolean_type() {
            let bool_type = schema::from_str(&"boolean").unwrap();
            let bool_type_2 = schema::from_str(&r#"{"type":"boolean"}"#).unwrap();
            let des_bool_type = SchemaOld::new(&"boolean").unwrap();

            assert_eq!(des_bool_type, bool_type);
            assert_eq!(des_bool_type, bool_type_2);
            assert_eq!(bool_type, bool_type_2);

            assert!(des_bool_type.is_valid().is_ok());
        }

        #[test]
        fn int_type() {
            let int_type = schema::from_str(&"int").unwrap();
            let int_type_2 = schema::from_str(&r#"{"type":"int"}"#).unwrap();
            let des_int_type = SchemaOld::new(&"int").unwrap();

            assert_eq!(des_int_type, int_type);
            assert_eq!(des_int_type, int_type_2);
            assert_eq!(int_type, int_type_2);

            assert!(des_int_type.is_valid().is_ok());
        }

        #[test]
        fn long_type() {
            let long_type = schema::from_str(&"long").unwrap();
            let long_type_2 = schema::from_str(&r#"{"type":"long"}"#).unwrap();
            let des_long_type = SchemaOld::new(&"long").unwrap();

            assert_eq!(des_long_type, long_type);
            assert_eq!(des_long_type, long_type_2);
            assert_eq!(long_type, long_type_2);

            assert!(des_long_type.is_valid().is_ok());
        }

        #[test]
        fn float_type() {
            let float_type = schema::from_str(&"float").unwrap();
            let float_type_2 = schema::from_str(&r#"{"type":"float"}"#).unwrap();
            let des_float_type = SchemaOld::new(&"float").unwrap();

            assert_eq!(des_float_type, float_type);
            assert_eq!(des_float_type, float_type_2);
            assert_eq!(float_type, float_type_2);

            assert!(des_float_type.is_valid().is_ok());
        }

        #[test]
        fn double_type() {
            let double_type = schema::from_str(&"double").unwrap();
            let double_type_2 = schema::from_str(&r#"{"type":"double"}"#).unwrap();
            let des_double_type = SchemaOld::new(&"double").unwrap();

            assert_eq!(des_double_type, double_type);
            assert_eq!(des_double_type, double_type_2);
            assert_eq!(double_type, double_type_2);

            assert!(des_double_type.is_valid().is_ok());
        }

        #[test]
        fn bytes_type() {
            let bytes_type = schema::from_str(&"bytes").unwrap();
            let bytes_type_2 = schema::from_str(&r#"{"type":"bytes"}"#).unwrap();
            let des_bytes_type = SchemaOld::new(&"bytes").unwrap();

            assert_eq!(des_bytes_type, bytes_type);
            assert_eq!(des_bytes_type, bytes_type_2);
            assert_eq!(bytes_type, bytes_type_2);

            assert!(des_bytes_type.is_valid().is_ok());
        }
        
        #[test]
        fn string_type() {
            let string_type = schema::from_str(&"string").unwrap();
            let string_type_2 = schema::from_str(&r#"{"type":"string"}"#).unwrap();
            let des_string_type = SchemaOld::new(&"string").unwrap();

            assert_eq!(des_string_type, string_type);
            assert_eq!(des_string_type, string_type_2);
            assert_eq!(string_type, string_type_2);

            assert!(des_string_type.is_valid().is_ok());
        }

        #[test]
        fn bogus_type() {
            let bogus_type = schema::from_str(&"bogus");
            assert!(bogus_type.is_err());

            let bogus_type_2 = schema::from_str(&r#"{"type":"bogus"}"#);
            assert!(bogus_type_2.is_err());
        }
    }
    */
}

mod array {
    mod is_array {
        use ravro::schema::Schema;
        use serde::json::{self, Value};

        #[test]
        fn is_simple_array() {
            let s1 = Schema::String(String::from("boolean"));
            let s2 = Schema::String(String::from("int"));

            let arr_schema = Schema::Array(vec!(s1, s2));

            assert!(arr_schema.is_array());
        }

        #[test]
        fn is_empty_array() {
            // I haven't seen anything that says an empty Avro array is illegal, although
            // it certainly would be very useufl...
            let arr_schema = Schema::Array(vec!());

            assert!(arr_schema.is_array());
        }

        #[test]
        fn primitive_is_not_array() {
            let s = Schema::String(String::from("boolean"));
            assert_eq!(s.is_array(), false);
        }

        #[test]
        fn object_is_not_array() {
            let val : Value = json::from_str(r#"{"type":"string"}"#).unwrap(); // about as simple an object as we can get
            let s = Schema::Object(val);

            assert_eq!(s.is_array(), false);
        }

        #[test]
        fn null_is_not_array() {
            let n = Schema::Null;
            assert_eq!(n.is_array(), false);
        }
    }

    mod ser {
        use ravro::schema::{self, Schema};

        #[test]
        fn array_of_primitives() {
            let s1 = Schema::String(String::from("boolean"));
            let s2 = Schema::String(String::from("int"));
            let arr_schema = Schema::Array(vec!(s1, s2));
            let s = schema::to_string(&arr_schema).unwrap();

            assert_eq!(s, String::from(r#"["boolean","int"]"#));
        }
    }
}

mod object {
    mod is_object {
        use ravro::schema::Schema;
        use serde::json::{self, Value};

        #[test]
        fn is_object() {
            let val : Value = json::from_str(r#"{"type":"string"}"#).unwrap(); // about as simple an object as we can get
            let s = Schema::Object(val);

            assert!(s.is_object());
        }

        #[test]
        fn primitive_is_not_object() {
            let s = Schema::String(String::from("boolean"));
            assert_eq!(s.is_object(), false);
        }

        #[test]
        fn array_is_not_object() {
            let a = Schema::Array(vec![]);
            assert_eq!(a.is_object(), false);
        }

        #[test]
        fn null_is_not_object() {
            let n = Schema::Null;
            assert_eq!(n.is_object(), false);
        }
    }

    mod as_object {
        use ravro::schema::Schema;
        use serde::json::{self, Value};

        #[test]
        fn object_as_object() {
            let val : Value = json::from_str(r#"{"type":"string"}"#).unwrap();
            let o = Schema::Object(val);
            let o2 = o.as_object().unwrap();

            assert_eq!(o, o2);
        }

        // Based on this test, I'm not going to go through all 8 primitive types.
        #[test]
        fn boolean_as_object() {
            let val : Value = json::from_str(r#"{"type":"boolean"}"#).unwrap();
            let o = Schema::Object(val);

            let b = Schema::String(String::from("boolean"));
            let o2 = b.as_object().unwrap();

            assert_eq!(o, o2);
        }

        // Not sure if this is strictly corrct...
        #[test]
        fn array_as_object() {
            // While the following line is hte simplest representation of the array, because
            // the implementation is doing to_object on each element, we are getting the
            // longer, {"type":"blah"} representation. This may not be a good thing in the
            // long run.
            //let val : Value = json::from_str(r#"["boolean","int"]"#).unwrap();
            let val : Value = json::from_str(r#"[{"type":"boolean"},{"type":"int"}]"#).unwrap();
            let o = Schema::Object(val);

            let s1 = Schema::String(String::from("boolean"));
            let s2 = Schema::String(String::from("int"));
            let arr_schema = Schema::Array(vec!(s1, s2));

            let o2 = arr_schema.as_object().unwrap();

            assert_eq!(o, o2);
        }

        #[test]
        fn null_as_object() {
            let val : Value = json::from_str(r#"{"type":"null"}"#).unwrap();
            let o = Schema::Object(val);

            let n = Schema::Null;
            let o2 = n.as_object().unwrap();

            assert_eq!(o, o2);
        }
    }

    // The Schema::Object type can be specialized as a record.
    mod record {
        use ravro::schema::Schema;
        use serde::json::{self, Value};
        use serde::json::builder::ObjectBuilder;

        #[test]
        fn is_record() {
            let val = ObjectBuilder::new()
                .insert(String::from("type"), String::from("record"))
                .insert(String::from("name"), String::from("foo"))
                .insert_array(String::from("fields"), |bld| bld)   // empty field array
                .unwrap();
            let o = Schema::Object(val);

            assert!(o.is_record());
        }

        #[test]
        fn to_string() {
            let val = ObjectBuilder::new()
                .insert(String::from("type"), String::from("record"))
                .insert(String::from("name"), String::from("foo"))
                .insert_array(String::from("fields"), |bld| bld)   // empty field array
                .unwrap();
            let o = Schema::Object(val);

            let s = o.to_string();
            // It's in this order because Serde's JSON serialization puts the fields in
            // alphabetical order.
            let pretty = concat!(
                "{",
                "\"fields\":[],",
                "\"name\":\"foo\",",
                "\"type\":\"record\"",
                "}"
            );

            assert_eq!(s, pretty);
        }

        mod fullname {
            use ravro::schema::Schema;
            use serde::json::{self, Value};
            use serde::json::builder::ObjectBuilder;

            #[test]
            fn simple_name() {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from("foo"))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().unwrap(), String::from("foo"));
            }

            #[test]
            fn name_with_namespace() {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from("foo"))
                    .insert(String::from("namespace"), String::from("x.y"))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().unwrap(), String::from("x.y.foo"));
            }

            #[test]
            fn name_with_dots() {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from("a.b.foo"))
                    .insert(String::from("namespace"), String::from("x.y"))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().unwrap(), String::from("a.b.foo"));
            }

            #[test]
            fn name_cannot_be_empty() {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from(""))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().is_err(), true);
            }

            #[test]
            fn name_cannot_end_with_period() {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from("foo."))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().is_err(), true);
            }

            #[test]
            fn name_cannot_start_with_period() {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from(".foo"))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().is_err(), true);
            }

            #[test]
            fn name_cannot_start_with_number() {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from("9foo"))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().is_err(), true);
            }

            #[test]
            fn namespace_with_more_complicated_segments() {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from("foo"))
                    .insert(String::from("namespace"), String::from("Yadda_.FooBar12_34.blah_blah_blah"))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().unwrap(), String::from("Yadda_.FooBar12_34.blah_blah_blah.foo"));
            }

            #[test]
            fn namespace_cannot_have_trailing_period() {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from("foo"))
                    .insert(String::from("namespace"), String::from("x.y."))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().is_err(), true);
            }

            // These name tests aren't exhaustive; they can't be really unless I come up with
            // a state machine to drive all possible permutations of the regex provided in the
            // specification. So while this isn't perfect, and I'm not happy, I'm hoping I've
            // covered the most significant cases.
        }

        mod ser {
            use std::string::String;
            use ravro::schema::Schema;
            use serde::json::{self, Value};
            use serde::json::builder::ObjectBuilder;

            #[test]
            fn rec_1() {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from("foo"))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                let s = String::from(&o);
                // It's in this order because Serde's JSON serialization puts the fields in
                // alphabetical order.
                let pretty = concat!(
                    "{",
                    "\"fields\":[],",
                    "\"name\":\"foo\",",
                    "\"type\":\"record\"",
                    "}"
                );

                assert_eq!(s, pretty);
            }
        }
    }
}

mod null {
    mod is_null {
        use ravro::schema::Schema;
        use serde::json::{self, Value};

        #[test]
        fn is_null() {
            let n = Schema::Null;
            assert!(n.is_null())
        }

        #[test]
        fn primitive_is_not_null() {
            let s = Schema::String(String::from("boolean"));
            assert_eq!(s.is_null(), false);
        }

        #[test]
        fn array_is_not_null() {
            let a = Schema::Array(vec![]);
            assert_eq!(a.is_null(), false);
        }

        #[test]
        fn object_is_not_null() {
            let val : Value = json::from_str(r#"{"type":"string"}"#).unwrap(); // about as simple an object as we can get
            let s = Schema::Object(val);

            assert_eq!(s.is_null(), false);
        }
    }

    mod ser {
        use ravro::schema::{self, Schema};

        #[test]
        fn null_type() {
            // Note that the "null" primitive type isn't the same as Schema::Null
            let n = Schema::Null;
            let s = schema::to_string(&n).unwrap();

            assert_eq!(s, String::from(r#""null""#));
        }
    }
}
