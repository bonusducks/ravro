// Integration style tests
extern crate ravro;
extern crate serde;

use ravro::*;
use serde::json::{self};

#[test]
fn exercise_point() {
    let point = Point { x: 1, y: 2 };
    let ser_point = json::to_string(&point).unwrap();

    assert_eq!(ser_point, "{\"x\":1,\"y\":2}");

    let des_point : Point = json::from_str(&ser_point).unwrap();

    assert_eq!(point, des_point);
}

#[test]
fn ser_null_type() {
    let null_type = SimpleType { raw_type: "null".to_string() };
    let ser_null_type = json::to_string(&null_type).unwrap();

    assert_eq!(ser_null_type, r#"{"type":"null"}"#);
}

#[test]
fn des_null_type() {
    let null_type = SimpleType { raw_type: "null".to_string() };
    let des_null_type : SimpleType = json::from_str(r#"{"type":"null"}"#).unwrap();

    assert_eq!(des_null_type, null_type);
    assert!(des_null_type.is_valid().is_ok());
}

#[test]
fn ser_boolean_type() {
    let bool_type = SimpleType { raw_type: "boolean".to_string() };
    let ser_bool_type = json::to_string(&bool_type).unwrap();

    assert_eq!(ser_bool_type, r#"{"type":"boolean"}"#);
}

#[test]
fn des_boolean_type() {
    let bool_type = SimpleType { raw_type: "boolean".to_string() };
    let des_bool_type : SimpleType = json::from_str(r#"{"type":"boolean"}"#).unwrap();

    assert_eq!(des_bool_type, bool_type);
    assert!(des_bool_type.is_valid().is_ok());
}

#[test]
fn ser_int_type() {
    let int_type = SimpleType { raw_type: "int".to_string() };
    let ser_int_type = json::to_string(&int_type).unwrap();

    assert_eq!(ser_int_type, r#"{"type":"int"}"#);
}

#[test]
fn des_int_type() {
    let int_type = SimpleType { raw_type: "int".to_string() };
    let des_int_type : SimpleType = json::from_str(r#"{"type":"int"}"#).unwrap();

    assert_eq!(des_int_type, int_type);
    assert!(des_int_type.is_valid().is_ok());
}

#[test]
fn ser_long_type() {
    let long_type = SimpleType { raw_type: "long".to_string() };
    let ser_long_type = json::to_string(&long_type).unwrap();

    assert_eq!(ser_long_type, r#"{"type":"long"}"#);
}

#[test]
fn des_long_type() {
    let long_type = SimpleType { raw_type: "long".to_string() };
    let des_long_type : SimpleType = json::from_str(r#"{"type":"long"}"#).unwrap();

    assert_eq!(des_long_type, long_type);
    assert!(des_long_type.is_valid().is_ok());
}

#[test]
fn ser_float_type() {
    let float_type = SimpleType { raw_type: "float".to_string() };
    let ser_float_type = json::to_string(&float_type).unwrap();

    assert_eq!(ser_float_type, r#"{"type":"float"}"#);
}

#[test]
fn des_float_type() {
    let float_type = SimpleType { raw_type: "float".to_string() };
    let des_float_type : SimpleType = json::from_str(r#"{"type":"float"}"#).unwrap();

    assert_eq!(des_float_type, float_type);
    assert!(des_float_type.is_valid().is_ok());
}

#[test]
fn ser_double_type() {
    let double_type = SimpleType { raw_type: "double".to_string() };
    let ser_double_type = json::to_string(&double_type).unwrap();

    assert_eq!(ser_double_type, r#"{"type":"double"}"#);
}

#[test]
fn des_double_type() {
    let double_type = SimpleType { raw_type: "double".to_string() };
    let des_double_type : SimpleType = json::from_str(r#"{"type":"double"}"#).unwrap();

    assert_eq!(des_double_type, double_type);
    assert!(des_double_type.is_valid().is_ok());
}

#[test]
fn ser_bytes_type() {
    let bytes_type = SimpleType { raw_type: "bytes".to_string() };
    let ser_bytes_type = json::to_string(&bytes_type).unwrap();

    assert_eq!(ser_bytes_type, r#"{"type":"bytes"}"#);
}

#[test]
fn des_bytes_type() {
    let bytes_type = SimpleType { raw_type: "bytes".to_string() };
    let des_bytes_type : SimpleType = json::from_str(r#"{"type":"bytes"}"#).unwrap();

    assert_eq!(des_bytes_type, bytes_type);
    assert!(des_bytes_type.is_valid().is_ok());
}

#[test]
fn ser_string_type() {
    let string_type = SimpleType { raw_type: "string".to_string() };
    let ser_string_type = json::to_string(&string_type).unwrap();

    assert_eq!(ser_string_type, r#"{"type":"string"}"#);
}

#[test]
fn des_string_type() {
    let string_type = SimpleType { raw_type: "string".to_string() };
    let des_string_type : SimpleType = json::from_str(r#"{"type":"string"}"#).unwrap();

    assert_eq!(des_string_type, string_type);
    assert!(des_string_type.is_valid().is_ok());
}

#[test]
fn des_bogus_type() {
    let bogus_type = SimpleType { raw_type: "bogus".to_string() };
    let des_bogus_type : SimpleType = json::from_str(r#"{"type":"bogus"}"#).unwrap();

    // All this really tells us is that we are faithfully deserializing invalid simple types, 
    // which need to be validated.
    assert_eq!(des_bogus_type, bogus_type);
    assert!(des_bogus_type.is_valid().is_err());
}

#[test]
fn check_raw_simple_type_conversions() {
    let null_raw = "null".to_string();
    assert_eq!(null_raw, SimpleTypeEnum::Null.to_raw());
    assert_eq!(SimpleTypeEnum::Null, SimpleTypeEnum::from_raw(&null_raw).unwrap());

    let boolean_raw = "boolean".to_string();
    assert_eq!(boolean_raw, SimpleTypeEnum::Boolean.to_raw());
    assert_eq!(SimpleTypeEnum::Boolean, SimpleTypeEnum::from_raw(&boolean_raw).unwrap());

    let int_raw = "int".to_string();
    assert_eq!(int_raw, SimpleTypeEnum::Int.to_raw());
    assert_eq!(SimpleTypeEnum::Int, SimpleTypeEnum::from_raw(&int_raw).unwrap());

    let long_raw = "long".to_string();
    assert_eq!(long_raw, SimpleTypeEnum::Long.to_raw());
    assert_eq!(SimpleTypeEnum::Long, SimpleTypeEnum::from_raw(&long_raw).unwrap());

    let float_raw = "float".to_string();
    assert_eq!(float_raw, SimpleTypeEnum::Float.to_raw());
    assert_eq!(SimpleTypeEnum::Float, SimpleTypeEnum::from_raw(&float_raw).unwrap());

    let double_raw = "double".to_string();
    assert_eq!(double_raw, SimpleTypeEnum::Double.to_raw());
    assert_eq!(SimpleTypeEnum::Double, SimpleTypeEnum::from_raw(&double_raw).unwrap());

    let bytes_raw = "bytes".to_string();
    assert_eq!(bytes_raw, SimpleTypeEnum::Bytes.to_raw());
    assert_eq!(SimpleTypeEnum::Bytes, SimpleTypeEnum::from_raw(&bytes_raw).unwrap());

    let string_raw = "string".to_string();
    assert_eq!(string_raw, SimpleTypeEnum::String.to_raw());
    assert_eq!(SimpleTypeEnum::String, SimpleTypeEnum::from_raw(&string_raw).unwrap());

    let bogus_raw = "bogus".to_string();
    assert_eq!(None, SimpleTypeEnum::from_raw(&bogus_raw));
}