// Integration style tests
extern crate ravro;
extern crate serde;

mod record {
    mod fullname {
        use ravro::schema::Schema;

        #[test]
        fn must_have_name() {
            let rec_result = Schema::new_rec("", &Vec::new());
            assert!(rec_result.is_err());
            assert_eq!(rec_result.unwrap_err(), "Records must have a name");
        }

        #[test]
        fn well_formed_name() {
            let rec_result = Schema::new_rec("foo_bar_123", &Vec::new());
            assert!(rec_result.is_ok());

            let rec = rec_result.unwrap();
            assert_eq!("foo_bar_123", &rec.fullname().unwrap());
        }

        #[test]
        fn invalid_name() {
            let rec_result = Schema::new_rec("foo_bar_%%%", &Vec::new());
            assert!(rec_result.is_err());

            let err = rec_result.unwrap_err();
            assert!(err.contains("not well formed"));
        }

        #[test]
        fn name_cant_end_with_period() {
            let rec_result = Schema::new_rec("foo.", &Vec::new());
            assert!(rec_result.is_err());

            let err = rec_result.unwrap_err();
            assert!(err.contains("not well formed"));
        }

        #[test]
        fn name_cant_start_with_period() {
            let rec_result = Schema::new_rec(".foo", &Vec::new());
            assert!(rec_result.is_err());

            let err = rec_result.unwrap_err();
            assert!(err.contains("not well formed"));
        }

        #[test]
        fn name_is_fullname() {
            let rec_result = Schema::new_rec("x.y.foo", &Vec::new());
            assert!(rec_result.is_ok());

            let rec = rec_result.unwrap();
            assert_eq!("x.y.foo", &rec.fullname().unwrap());
        }

        #[test]
        fn name_and_namespace() {
            let rec_type = Schema::new_rec_full("foo",&Vec::new(),"x.y","",&Vec::new()).unwrap();
            let fullname = rec_type.fullname().unwrap();

            assert_eq!("x.y.foo", fullname);
        }

        #[test]
        fn ignore_namespace() {
            let rec_type = Schema::new_rec_full("x.y.foo",&Vec::new(),"a.b","",&Vec::new()).unwrap();
            let fullname = rec_type.fullname().unwrap();

            assert_eq!("x.y.foo", fullname);
        }

        #[test]
        fn namespace_cant_start_with_period() {
            let rec_type_result = Schema::new_rec_full("foo",&Vec::new(),".x.y","",&Vec::new());
            assert!(rec_type_result.is_err());

            let err = rec_type_result.unwrap_err();
            assert!(err.contains("not well formed"));
        }

        #[test]
        fn namespace_cant_end_with_period() {
            let rec_type_result = Schema::new_rec_full("foo",&Vec::new(),"x.y.","",&Vec::new());
            assert!(rec_type_result.is_err());

            let err = rec_type_result.unwrap_err();
            assert!(err.contains("not well formed"));
        }
    }

    mod ser {
        use ravro::schema::{self, Field};
        use serde::json::{self};

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
                                    {"type":"boolean", "name":"f1"},
                                    {"name":"f2", "type":"int"}
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
      "default": null,
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
      "aliases": null
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
            let field = Field::new_full("f1", "boolean", "", "", "", &Vec::new()).unwrap();
            let ser_field = json::to_string(&field).unwrap();

            assert_eq!(ser_field, r#"{"name":"f1","type":"boolean","default":null,"doc":null,"order":null,"aliases":null}"#);
        }
    }

    mod des {
        use ravro::schema::{self, Schema, Field};

        #[test]
        fn record_type_1() {
            let des_rec_type = schema::from_str(&r#"{"fields":[], "type":"record", "name":"foo"}"#).unwrap();
            let rec_type = Schema::new_rec("foo",&Vec::new()).unwrap();

            assert_eq!(rec_type, des_rec_type);
        }

        #[test]
        fn record_type_2() {
            let des_rec_type = schema::from_str(&r#"{"fields":[], "type":"record", "name":"foo", "namespace":"x.y"}"#).unwrap();
            let rec_type = Schema::new_rec_full("foo",&Vec::new(),"x.y","",&Vec::new()).unwrap();

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
            let rec_type = Schema::new_rec_full("foo", &field_vec, "", "", &Vec::new()).unwrap();

            assert_eq!(rec_type, des_rec_type);
        }

        #[test]
        fn record_type_5() {
            let json_str = r#"{ "type" : "record", 
                                "name" : "foo", 
                                "fields" : [
                                    {"type":"boolean", "name":"f1"},
                                    {"name":"f2", "type":"int"}
                                ]
                              }"#;
            let des_rec_type = schema::from_str(&json_str).unwrap();
            let field1 = Field::new("f1", "boolean").unwrap();
            let field2 = Field::new("f2", "int").unwrap();
            let field_vec = vec![field1, field2];
            let rec_type = Schema::new_rec_full("foo", &field_vec, "", "", &Vec::new()).unwrap();

            assert_eq!(rec_type, des_rec_type);
        }
    }
}

mod primitive {
    mod fullname {
        use ravro::schema::Schema;

        #[test]
        fn int_fullname() {
            let s = Schema::new("int").unwrap();
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

    mod ser {
        use ravro::schema::{self};
        use serde::json::{self};

        #[test]
        fn null_type() {
            let null_type = schema::from_str(&"null").unwrap();
            let ser_null_type = json::to_string(&null_type).unwrap();

            assert_eq!(ser_null_type, r#"{"type":"null","name":null,"namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
        }  

        #[test]
        fn boolean_type() {
            let bool_type = schema::from_str(&"boolean").unwrap();
            let ser_bool_type = json::to_string(&bool_type).unwrap();

            assert_eq!(ser_bool_type, r#"{"type":"boolean","name":null,"namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
        }

        #[test]
        fn int_type() {
            let int_type = schema::from_str(&"int").unwrap();
            let ser_int_type = json::to_string(&int_type).unwrap();

            assert_eq!(ser_int_type, r#"{"type":"int","name":null,"namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
        }

        #[test]
        fn long_type() {
            let long_type = schema::from_str(&"long").unwrap();
            let ser_long_type = json::to_string(&long_type).unwrap();

            assert_eq!(ser_long_type, r#"{"type":"long","name":null,"namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
        }

        #[test]
        fn float_type() {
            let float_type = schema::from_str(&"float").unwrap();
            let ser_float_type = json::to_string(&float_type).unwrap();

            assert_eq!(ser_float_type, r#"{"type":"float","name":null,"namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
        }

        #[test]
        fn double_type() {
            let double_type = schema::from_str(&"double").unwrap();
            let ser_double_type = json::to_string(&double_type).unwrap();

            assert_eq!(ser_double_type, r#"{"type":"double","name":null,"namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
        }

        #[test]
        fn bytes_type() {
            let bytes_type = schema::from_str(&"bytes").unwrap();
            let ser_bytes_type = json::to_string(&bytes_type).unwrap();

            assert_eq!(ser_bytes_type, r#"{"type":"bytes","name":null,"namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
        }

        #[test]
        fn string_type() {
            let string_type = schema::from_str(&"string").unwrap();
            let ser_string_type = json::to_string(&string_type).unwrap();

            assert_eq!(ser_string_type, r#"{"type":"string","name":null,"namespace":null,"doc":null,"aliases":[],"fields":[]}"#);
        }
    }

    mod des {
        use ravro::schema::{self, Schema};

        #[test]
        fn null_type() {
            let null_type = schema::from_str(&"null").unwrap();
            let null_type_2 = schema::from_str(&r#"{"type":"null"}"#).unwrap();
            let des_null_type  = Schema::new(&"null").unwrap();

            assert_eq!(des_null_type, null_type);
            assert_eq!(des_null_type, null_type_2);
            assert_eq!(null_type, null_type_2);

            assert!(des_null_type.is_valid().is_ok());
        }

        #[test]
        fn boolean_type() {
            let bool_type = schema::from_str(&"boolean").unwrap();
            let bool_type_2 = schema::from_str(&r#"{"type":"boolean"}"#).unwrap();
            let des_bool_type = Schema::new(&"boolean").unwrap();

            assert_eq!(des_bool_type, bool_type);
            assert_eq!(des_bool_type, bool_type_2);
            assert_eq!(bool_type, bool_type_2);

            assert!(des_bool_type.is_valid().is_ok());
        }

        #[test]
        fn int_type() {
            let int_type = schema::from_str(&"int").unwrap();
            let int_type_2 = schema::from_str(&r#"{"type":"int"}"#).unwrap();
            let des_int_type = Schema::new(&"int").unwrap();

            assert_eq!(des_int_type, int_type);
            assert_eq!(des_int_type, int_type_2);
            assert_eq!(int_type, int_type_2);

            assert!(des_int_type.is_valid().is_ok());
        }

        #[test]
        fn long_type() {
            let long_type = schema::from_str(&"long").unwrap();
            let long_type_2 = schema::from_str(&r#"{"type":"long"}"#).unwrap();
            let des_long_type = Schema::new(&"long").unwrap();

            assert_eq!(des_long_type, long_type);
            assert_eq!(des_long_type, long_type_2);
            assert_eq!(long_type, long_type_2);

            assert!(des_long_type.is_valid().is_ok());
        }

        #[test]
        fn float_type() {
            let float_type = schema::from_str(&"float").unwrap();
            let float_type_2 = schema::from_str(&r#"{"type":"float"}"#).unwrap();
            let des_float_type = Schema::new(&"float").unwrap();

            assert_eq!(des_float_type, float_type);
            assert_eq!(des_float_type, float_type_2);
            assert_eq!(float_type, float_type_2);

            assert!(des_float_type.is_valid().is_ok());
        }

        #[test]
        fn double_type() {
            let double_type = schema::from_str(&"double").unwrap();
            let double_type_2 = schema::from_str(&r#"{"type":"double"}"#).unwrap();
            let des_double_type = Schema::new(&"double").unwrap();

            assert_eq!(des_double_type, double_type);
            assert_eq!(des_double_type, double_type_2);
            assert_eq!(double_type, double_type_2);

            assert!(des_double_type.is_valid().is_ok());
        }

        #[test]
        fn bytes_type() {
            let bytes_type = schema::from_str(&"bytes").unwrap();
            let bytes_type_2 = schema::from_str(&r#"{"type":"bytes"}"#).unwrap();
            let des_bytes_type = Schema::new(&"bytes").unwrap();

            assert_eq!(des_bytes_type, bytes_type);
            assert_eq!(des_bytes_type, bytes_type_2);
            assert_eq!(bytes_type, bytes_type_2);

            assert!(des_bytes_type.is_valid().is_ok());
        }
        
        #[test]
        fn string_type() {
            let string_type = schema::from_str(&"string").unwrap();
            let string_type_2 = schema::from_str(&r#"{"type":"string"}"#).unwrap();
            let des_string_type = Schema::new(&"string").unwrap();

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
}
