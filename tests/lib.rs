// Integration style tests
extern crate ravro;
extern crate serde;

use ravro::schema::{self, Schema, PrimitiveTypeEnum};
use serde::json::{self};

// I'm so not happy with the serialization...

#[test]
fn ser_null_type() {
    let null_type = schema::from_str(&"null").unwrap();
    let ser_null_type = json::to_string(&null_type).unwrap();

    assert_eq!(ser_null_type, r#"{"type":"null","name":null,"namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
}

#[test]
fn des_null_type() {
    let null_type = schema::from_str(&"null").unwrap();
    let null_type_2 = schema::from_str(&"{\"type\":\"null\"}").unwrap();
    let des_null_type  = Schema::new(&"null").unwrap();

    assert_eq!(des_null_type, null_type);
    assert_eq!(des_null_type, null_type_2);
    assert_eq!(null_type, null_type_2);

    assert!(des_null_type.is_valid().is_ok());
}

#[test]
fn ser_boolean_type() {
    let bool_type = schema::from_str(&"boolean").unwrap();
    let ser_bool_type = json::to_string(&bool_type).unwrap();

    assert_eq!(ser_bool_type, r#"{"type":"boolean","name":null,"namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
}

#[test]
fn des_boolean_type() {
    let bool_type = schema::from_str(&"boolean").unwrap();
    let bool_type_2 = schema::from_str(&r#"{"type":"boolean"}"#).unwrap();
    let des_bool_type = Schema::new(&"boolean").unwrap();

    assert_eq!(des_bool_type, bool_type);
    assert_eq!(des_bool_type, bool_type_2);
    assert_eq!(bool_type, bool_type_2);

    assert!(des_bool_type.is_valid().is_ok());
}

#[test]
fn ser_int_type() {
    let int_type = schema::from_str(&"int").unwrap();
    let ser_int_type = json::to_string(&int_type).unwrap();

    assert_eq!(ser_int_type, r#"{"type":"int","name":null,"namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
}

#[test]
fn des_int_type() {
    let int_type = schema::from_str(&"int").unwrap();
    let int_type_2 = schema::from_str(&r#"{"type":"int"}"#).unwrap();
    let des_int_type = Schema::new(&"int").unwrap();

    assert_eq!(des_int_type, int_type);
    assert_eq!(des_int_type, int_type_2);
    assert_eq!(int_type, int_type_2);

    assert!(des_int_type.is_valid().is_ok());
}

#[test]
fn ser_long_type() {
    let long_type = schema::from_str(&"long").unwrap();
    let ser_long_type = json::to_string(&long_type).unwrap();

    assert_eq!(ser_long_type, r#"{"type":"long","name":null,"namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
}

#[test]
fn des_long_type() {
    let long_type = schema::from_str(&"long").unwrap();
    let long_type_2 = schema::from_str(&r#"{"type":"long"}"#).unwrap();
    let des_long_type = Schema::new(&"long").unwrap();

    assert_eq!(des_long_type, long_type);
    assert_eq!(des_long_type, long_type_2);
    assert_eq!(long_type, long_type_2);

    assert!(des_long_type.is_valid().is_ok());
}

#[test]
fn ser_float_type() {
    let float_type = schema::from_str(&"float").unwrap();
    let ser_float_type = json::to_string(&float_type).unwrap();

    assert_eq!(ser_float_type, r#"{"type":"float","name":null,"namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
}

#[test]
fn des_float_type() {
    let float_type = schema::from_str(&"float").unwrap();
    let float_type_2 = schema::from_str(&r#"{"type":"float"}"#).unwrap();
    let des_float_type = Schema::new(&"float").unwrap();

    assert_eq!(des_float_type, float_type);
    assert_eq!(des_float_type, float_type_2);
    assert_eq!(float_type, float_type_2);

    assert!(des_float_type.is_valid().is_ok());
}

#[test]
fn ser_double_type() {
    let double_type = schema::from_str(&"double").unwrap();
    let ser_double_type = json::to_string(&double_type).unwrap();

    assert_eq!(ser_double_type, r#"{"type":"double","name":null,"namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
}

#[test]
fn des_double_type() {
    let double_type = schema::from_str(&"double").unwrap();
    let double_type_2 = schema::from_str(&r#"{"type":"double"}"#).unwrap();
    let des_double_type = Schema::new(&"double").unwrap();

    assert_eq!(des_double_type, double_type);
    assert_eq!(des_double_type, double_type_2);
    assert_eq!(double_type, double_type_2);

    assert!(des_double_type.is_valid().is_ok());
}

#[test]
fn ser_bytes_type() {
    let bytes_type = schema::from_str(&"bytes").unwrap();
    let ser_bytes_type = json::to_string(&bytes_type).unwrap();

    assert_eq!(ser_bytes_type, r#"{"type":"bytes","name":null,"namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
}

#[test]
fn des_bytes_type() {
    let bytes_type = schema::from_str(&"bytes").unwrap();
    let bytes_type_2 = schema::from_str(&r#"{"type":"bytes"}"#).unwrap();
    let des_bytes_type = Schema::new(&"bytes").unwrap();

    assert_eq!(des_bytes_type, bytes_type);
    assert_eq!(des_bytes_type, bytes_type_2);
    assert_eq!(bytes_type, bytes_type_2);

    assert!(des_bytes_type.is_valid().is_ok());
}

#[test]
fn ser_string_type() {
    let string_type = schema::from_str(&"string").unwrap();
    let ser_string_type = json::to_string(&string_type).unwrap();

    assert_eq!(ser_string_type, r#"{"type":"string","name":null,"namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
}

#[test]
fn des_string_type() {
    let string_type = schema::from_str(&"string").unwrap();
    let string_type_2 = schema::from_str(&r#"{"type":"string"}"#).unwrap();
    let des_string_type = Schema::new(&"string").unwrap();

    assert_eq!(des_string_type, string_type);
    assert_eq!(des_string_type, string_type_2);
    assert_eq!(string_type, string_type_2);

    assert!(des_string_type.is_valid().is_ok());
}

#[test]
fn des_bogus_type() {
    let bogus_type = schema::from_str(&"bogus");
    assert!(bogus_type.is_err());

    let bogus_type_2 = schema::from_str(&r#"{"type":"bogus"}"#);
    assert!(bogus_type_2.is_err());
}

#[test]
fn ser_record_type_1() {
    // This is about the simplest record serialization test I can think of
    let rec_type = schema::from_str(&r#"{"type":"record", "name":"foo", "fields":[]}"#).unwrap();
    let ser_rec_type = json::to_string(&rec_type).unwrap();

    assert_eq!(ser_rec_type, r#"{"type":"record","name":"foo","namespace":null,"doc":null,"aliases":[],"fields":[]}"#)
}

#[test]
fn check_raw_simple_type_conversions() {
    let null_raw = "null".to_string();
    assert_eq!(null_raw, PrimitiveTypeEnum::Null.to_string());
    assert_eq!(PrimitiveTypeEnum::Null, PrimitiveTypeEnum::from_string(&null_raw).unwrap());

    let boolean_raw = "boolean".to_string();
    assert_eq!(boolean_raw, PrimitiveTypeEnum::Boolean.to_string());
    assert_eq!(PrimitiveTypeEnum::Boolean, PrimitiveTypeEnum::from_string(&boolean_raw).unwrap());

    let int_raw = "int".to_string();
    assert_eq!(int_raw, PrimitiveTypeEnum::Int.to_string());
    assert_eq!(PrimitiveTypeEnum::Int, PrimitiveTypeEnum::from_string(&int_raw).unwrap());

    let long_raw = "long".to_string();
    assert_eq!(long_raw, PrimitiveTypeEnum::Long.to_string());
    assert_eq!(PrimitiveTypeEnum::Long, PrimitiveTypeEnum::from_string(&long_raw).unwrap());

    let float_raw = "float".to_string();
    assert_eq!(float_raw, PrimitiveTypeEnum::Float.to_string());
    assert_eq!(PrimitiveTypeEnum::Float, PrimitiveTypeEnum::from_string(&float_raw).unwrap());

    let double_raw = "double".to_string();
    assert_eq!(double_raw, PrimitiveTypeEnum::Double.to_string());
    assert_eq!(PrimitiveTypeEnum::Double, PrimitiveTypeEnum::from_string(&double_raw).unwrap());

    let bytes_raw = "bytes".to_string();
    assert_eq!(bytes_raw, PrimitiveTypeEnum::Bytes.to_string());
    assert_eq!(PrimitiveTypeEnum::Bytes, PrimitiveTypeEnum::from_string(&bytes_raw).unwrap());

    let string_raw = "string".to_string();
    assert_eq!(string_raw, PrimitiveTypeEnum::String.to_string());
    assert_eq!(PrimitiveTypeEnum::String, PrimitiveTypeEnum::from_string(&string_raw).unwrap());

    let bogus_raw = "bogus".to_string();
    assert_eq!(None, PrimitiveTypeEnum::from_string(&bogus_raw));
}