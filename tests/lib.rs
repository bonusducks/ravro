// Integration style tests
extern crate ravro;
extern crate serde;

use ravro::schema::*;
use serde::json::{self};

// I'm so not happy with the serialization...

#[test]
fn ser_null_type() {
    let null_type = Schema::new(&"null").unwrap();
    let ser_null_type = json::to_string(&null_type).unwrap();

    assert_eq!(ser_null_type, r#"{"type":"null","namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
}

#[test]
fn des_null_type() {
    let null_type = Schema::new(&"null").unwrap();
    let des_null_type : Schema = json::from_str(r#"{"type":"null"}"#).unwrap();

    assert_eq!(des_null_type, null_type);
    assert!(des_null_type.is_valid().is_ok());
}

#[test]
fn ser_boolean_type() {
    let bool_type = Schema::new(&"boolean").unwrap();
    let ser_bool_type = json::to_string(&bool_type).unwrap();

    assert_eq!(ser_bool_type, r#"{"type":"boolean","namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
}

#[test]
fn des_boolean_type() {
    let bool_type = Schema::new(&"boolean").unwrap();
    let des_bool_type : Schema = json::from_str(r#"{"type":"boolean"}"#).unwrap();

    assert_eq!(des_bool_type, bool_type);
    assert!(des_bool_type.is_valid().is_ok());
}

#[test]
fn ser_int_type() {
    let int_type = Schema::new(&"int").unwrap();
    let ser_int_type = json::to_string(&int_type).unwrap();

    assert_eq!(ser_int_type, r#"{"type":"int","namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
}

#[test]
fn des_int_type() {
    let int_type = Schema::new(&"int").unwrap();
    let des_int_type : Schema = json::from_str(r#"{"type":"int"}"#).unwrap();

    assert_eq!(des_int_type, int_type);
    assert!(des_int_type.is_valid().is_ok());
}

#[test]
fn ser_long_type() {
    let long_type = Schema::new(&"long").unwrap();
    let ser_long_type = json::to_string(&long_type).unwrap();

    assert_eq!(ser_long_type, r#"{"type":"long","namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
}

#[test]
fn des_long_type() {
    let long_type = Schema::new(&"long").unwrap();
    let des_long_type : Schema = json::from_str(r#"{"type":"long"}"#).unwrap();

    assert_eq!(des_long_type, long_type);
    assert!(des_long_type.is_valid().is_ok());
}

#[test]
fn ser_float_type() {
    let float_type = Schema::new(&"float").unwrap();
    let ser_float_type = json::to_string(&float_type).unwrap();

    assert_eq!(ser_float_type, r#"{"type":"float","namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
}

#[test]
fn des_float_type() {
    let float_type = Schema::new(&"float").unwrap();
    let des_float_type : Schema = json::from_str(r#"{"type":"float"}"#).unwrap();

    assert_eq!(des_float_type, float_type);
    assert!(des_float_type.is_valid().is_ok());
}

#[test]
fn ser_double_type() {
    let double_type = Schema::new(&"double").unwrap();
    let ser_double_type = json::to_string(&double_type).unwrap();

    assert_eq!(ser_double_type, r#"{"type":"double","namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
}

#[test]
fn des_double_type() {
    let double_type = Schema::new(&"double").unwrap();
    let des_double_type : Schema = json::from_str(r#"{"type":"double"}"#).unwrap();

    assert_eq!(des_double_type, double_type);
    assert!(des_double_type.is_valid().is_ok());
}

#[test]
fn ser_bytes_type() {
    let bytes_type = Schema::new(&"bytes").unwrap();
    let ser_bytes_type = json::to_string(&bytes_type).unwrap();

    assert_eq!(ser_bytes_type, r#"{"type":"bytes","namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
}

#[test]
fn des_bytes_type() {
    let bytes_type = Schema::new(&"bytes").unwrap();
    let des_bytes_type : Schema = json::from_str(r#"{"type":"bytes"}"#).unwrap();

    assert_eq!(des_bytes_type, bytes_type);
    assert!(des_bytes_type.is_valid().is_ok());
}

#[test]
fn ser_string_type() {
    let string_type = Schema::new(&"string").unwrap();
    let ser_string_type = json::to_string(&string_type).unwrap();

    assert_eq!(ser_string_type, r#"{"type":"string","namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
}

#[test]
fn des_string_type() {
    let string_type = Schema::new(&"string").unwrap();
    let des_string_type : Schema = json::from_str(r#"{"type":"string"}"#).unwrap();

    assert_eq!(des_string_type, string_type);
    assert!(des_string_type.is_valid().is_ok());
}

#[test]
fn des_bogus_type() {
    let bogus_type = Schema::new(&"bogus");
    assert!(bogus_type.is_err());
}

#[test]
fn check_raw_simple_type_conversions() {
    let null_raw = "null".to_string();
    assert_eq!(null_raw, PrimitiveTypeEnum::Null.to_raw());
    assert_eq!(PrimitiveTypeEnum::Null, PrimitiveTypeEnum::from_raw(&null_raw).unwrap());

    let boolean_raw = "boolean".to_string();
    assert_eq!(boolean_raw, PrimitiveTypeEnum::Boolean.to_raw());
    assert_eq!(PrimitiveTypeEnum::Boolean, PrimitiveTypeEnum::from_raw(&boolean_raw).unwrap());

    let int_raw = "int".to_string();
    assert_eq!(int_raw, PrimitiveTypeEnum::Int.to_raw());
    assert_eq!(PrimitiveTypeEnum::Int, PrimitiveTypeEnum::from_raw(&int_raw).unwrap());

    let long_raw = "long".to_string();
    assert_eq!(long_raw, PrimitiveTypeEnum::Long.to_raw());
    assert_eq!(PrimitiveTypeEnum::Long, PrimitiveTypeEnum::from_raw(&long_raw).unwrap());

    let float_raw = "float".to_string();
    assert_eq!(float_raw, PrimitiveTypeEnum::Float.to_raw());
    assert_eq!(PrimitiveTypeEnum::Float, PrimitiveTypeEnum::from_raw(&float_raw).unwrap());

    let double_raw = "double".to_string();
    assert_eq!(double_raw, PrimitiveTypeEnum::Double.to_raw());
    assert_eq!(PrimitiveTypeEnum::Double, PrimitiveTypeEnum::from_raw(&double_raw).unwrap());

    let bytes_raw = "bytes".to_string();
    assert_eq!(bytes_raw, PrimitiveTypeEnum::Bytes.to_raw());
    assert_eq!(PrimitiveTypeEnum::Bytes, PrimitiveTypeEnum::from_raw(&bytes_raw).unwrap());

    let string_raw = "string".to_string();
    assert_eq!(string_raw, PrimitiveTypeEnum::String.to_raw());
    assert_eq!(PrimitiveTypeEnum::String, PrimitiveTypeEnum::from_raw(&string_raw).unwrap());

    let bogus_raw = "bogus".to_string();
    assert_eq!(None, PrimitiveTypeEnum::from_raw(&bogus_raw));
}