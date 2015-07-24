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

        #[test]
        fn has_doc() {
            let val = ObjectBuilder::new()
                .insert(String::from("type"), String::from("record"))
                .insert(String::from("name"), String::from("foo"))
                .insert(String::from("doc"), String::from("yadda yadda"))
                .insert_array(String::from("fields"), |bld| bld)   // empty field array
                .unwrap();
            let o = Schema::Object(val);

            assert_eq!(o.doc().unwrap(), "yadda yadda");
        }

        mod fullname {
            use ravro::schema::Schema;
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
            use ravro::schema::Schema;
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

        mod builder {
            use ravro::schema::{FieldSortOrder, RecordBuilder, Schema};
            use serde::json::Value;

            #[test]
            fn is_record() {
                let r = RecordBuilder::new()
                    .unwrap();

                assert!(r.is_record());
                assert!(r.fullname().is_err()); // didn't assign a name to the record.
            }

            #[test]
            fn has_name() {
                let r = RecordBuilder::new()
                    .name(String::from("foo"))
                    .unwrap();

                assert!(r.is_record());
                assert_eq!(r.fullname().unwrap(), "foo");
            }

            #[test]
            fn has_namespace() {
                let r = RecordBuilder::new()
                    .name(String::from("foo"))
                    .namespace(String::from("x.y"))
                    .unwrap();

                assert_eq!(r.fullname().unwrap(), "x.y.foo");
            }

            #[test]
            fn has_doc() {
                let r = RecordBuilder::new()
                    .name(String::from("foo"))
                    .doc(String::from("yadda yadda"))
                    .unwrap();

                assert_eq!(r.doc().unwrap(), "yadda yadda");
            }

            #[test]
            fn has_aliases() {
                let aliases_vec = vec![String::from("bar"), String::from("baz")];
                let r = RecordBuilder::new()
                    .name(String::from("foo"))
                    .aliases(aliases_vec)
                    .unwrap();

                assert_eq!(r.aliases().unwrap(), vec![String::from("bar"), String::from("baz")]);
            }

            #[test]
            fn has_one_field() {
                let r = RecordBuilder::new()
                    .name(String::from("foo"))
                    .fields(|fab|
                        fab.push(|fb|
                            fb.name(String::from("bar"))
                              .field_type(Schema::String(String::from("boolean")))
                        )
                    )
                    .unwrap();

                let s = String::from(&r);
                // It's in this order because Serde's JSON serialization puts the fields in
                // alphabetical order, which is not the Avro cannonical order.
                let pretty = concat!(
                    "{",
                    "\"fields\":[{\"name\":\"bar\",\"type\":{\"type\":\"boolean\"}}],",
                    "\"name\":\"foo\",",
                    "\"type\":\"record\"",
                    "}"
                );

                assert_eq!(s, pretty);
            }

            #[test]
            fn has_multiple_fields() {
                let r = RecordBuilder::new()
                    .name(String::from("foo"))
                    .fields(|bld|
                        bld
                        .push(|fb|
                            fb.name(String::from("bar"))
                              .field_type(Schema::String(String::from("boolean")))
                        )
                        .push(|fb|
                            fb.name(String::from("baz"))
                              .field_type(Schema::String(String::from("int")))
                              .doc(String::from("yadda yadda"))
                        )
                    )
                    .unwrap();

                let s = String::from(&r);
                // It's in this order because Serde's JSON serialization puts the fields in
                // alphabetical order, which is not the Avro cannonical order.
                let pretty = concat!(
                    "{",
                    "\"fields\":[",
                    "{\"name\":\"bar\",\"type\":{\"type\":\"boolean\"}},",
                    "{\"doc\":\"yadda yadda\",\"name\":\"baz\",\"type\":{\"type\":\"int\"}}",
                    "],",
                    "\"name\":\"foo\",",
                    "\"type\":\"record\"",
                    "}"
                );

                assert_eq!(s, pretty);
            }

            #[test]
            fn has_field_with_order() {
                let r = RecordBuilder::new()
                    .name(String::from("foo"))
                    .fields(|fab|
                        fab.push(|fb|
                            fb.name(String::from("bar"))
                              .field_type(Schema::String(String::from("string")))
                              .order(FieldSortOrder::Ascending)
                        )
                    )
                    .unwrap();

                let s = String::from(&r);
                // It's in this order because Serde's JSON serialization puts the fields in
                // alphabetical order, which is not the Avro cannonical order.
                let pretty = concat!(
                    "{",
                    "\"fields\":[{\"name\":\"bar\",\"order\":\"ascending\",\"type\":{\"type\":\"string\"}}],",
                    "\"name\":\"foo\",",
                    "\"type\":\"record\"",
                    "}"
                );

                assert_eq!(s, pretty);
            }

            #[test]
            fn has_field_with_aliases() {
                let aliases_vec = vec![String::from("bar"), String::from("baz")];

                let r = RecordBuilder::new()
                    .name(String::from("foo"))
                    .fields(|fab|
                        fab.push(|fb|
                            fb.name(String::from("bar"))
                              .field_type(Schema::String(String::from("string")))
                              .aliases(aliases_vec)
                        )
                    )
                    .unwrap();

                let s = String::from(&r);
                // It's in this order because Serde's JSON serialization puts the fields in
                // alphabetical order, which is not the Avro cannonical order.
                let pretty = concat!(
                    "{",
                    "\"fields\":[{\"aliases\":[\"bar\",\"baz\"],\"name\":\"bar\",\"type\":{\"type\":\"string\"}}],",
                    "\"name\":\"foo\",",
                    "\"type\":\"record\"",
                    "}"
                );

                assert_eq!(s, pretty);
            }

            #[test]
            fn has_field_with_default() {
                let r = RecordBuilder::new()
                    .name(String::from("foo"))
                    .fields(|fab|
                        fab.push(|fb|
                            fb.name(String::from("bar"))
                              .field_type(Schema::String(String::from("string")))
                              .default(Value::String(String::from("one two three")))
                        )
                    )
                    .unwrap();

                let s = String::from(&r);
                // It's in this order because Serde's JSON serialization puts the fields in
                // alphabetical order, which is not the Avro cannonical order.
                let pretty = concat!(
                    "{",
                    "\"fields\":[{\"default\":\"one two three\",\"name\":\"bar\",\"type\":{\"type\":\"string\"}}],",
                    "\"name\":\"foo\",",
                    "\"type\":\"record\"",
                    "}"
                );

                assert_eq!(s, pretty);
            }
        }
    }

    mod enum_type { // enum is a keyword...
        use ravro::schema::Schema;
        use serde::json::builder::ObjectBuilder;

        #[test]
        fn is_enum() {
            let val = ObjectBuilder::new()
                .insert(String::from("type"), String::from("enum"))
                .insert(String::from("name"), String::from("foo"))
                .insert_array(String::from("symbols"), |bld| bld.push(String::from("A1")) )
                .unwrap();
            let o = Schema::Object(val);

            assert!(o.is_enum());
        }

        #[test]
        fn to_string() {
            let val = ObjectBuilder::new()
                .insert(String::from("type"), String::from("enum"))
                .insert(String::from("name"), String::from("foo"))
                .insert_array(String::from("symbols"), |bld| bld.push(String::from("A1")) )
                .unwrap();
            let o = Schema::Object(val);

            let s = o.to_string();
            // It's in this order because Serde's JSON serialization puts the fields in
            // alphabetical order.
            let pretty = concat!(
                "{",
                "\"name\":\"foo\",",
                "\"symbols\":[\"A1\"],",
                "\"type\":\"enum\"",
                "}"
            );

            assert_eq!(s, pretty);
        }

        mod builder {
            use ravro::schema::EnumBuilder;

            #[test]
            fn is_enum() {
                let e = EnumBuilder::new()
                    .unwrap();

                assert!(e.is_enum());
                assert!(e.fullname().is_err()); // didn't assign a name to the enum.
            }

            #[test]
            fn has_name() {
                let e = EnumBuilder::new()
                    .name(String::from("foo"))
                    .unwrap();

                assert_eq!(e.fullname().unwrap(), "foo");
            }

            #[test]
            fn has_namespace() {
                let e = EnumBuilder::new()
                    .name(String::from("foo"))
                    .namespace(String::from("x.y"))
                    .unwrap();

                assert_eq!(e.fullname().unwrap(), "x.y.foo");
            }

            #[test]
            fn has_doc() {
                let e = EnumBuilder::new()
                    .name(String::from("foo"))
                    .doc(String::from("yadda yadda"))
                    .unwrap();

                assert_eq!(e.doc().unwrap(), "yadda yadda");
            }

            #[test]
            fn has_aliases() {
                let aliases_vec = vec![String::from("bar"), String::from("baz")];
                let e = EnumBuilder::new()
                    .name(String::from("foo"))
                    .aliases(aliases_vec)
                    .unwrap();

                assert_eq!(e.aliases().unwrap(), vec![String::from("bar"), String::from("baz")]);
            }

            #[test]
            fn has_symbols() {
                let e = EnumBuilder::new()
                    .name(String::from("foo"))
                    .symbols(|bld|
                        bld.push("A1").push("A2")
                    )
                    .unwrap();

                assert_eq!(e.symbols().unwrap(), vec![String::from("A1"), String::from("A2")]);
            }
        }
    }

    mod array {
        use ravro::schema::Schema;
        use serde::json::builder::ObjectBuilder;

        #[test]
        fn is_array() {
            let val = ObjectBuilder::new()
                .insert(String::from("type"), String::from("array"))
                .insert(String::from("items"), String::from("string"))
                .unwrap();
            let o = Schema::Object(val);

            assert!(o.is_array());
        }

        #[test]
        fn to_string() {
            let val = ObjectBuilder::new()
                .insert(String::from("type"), String::from("array"))
                .insert(String::from("items"), String::from("string")) // technically, the items value is a Schema.
                .unwrap();
            let o = Schema::Object(val);

            let s = o.to_string();
            // It's in this order because Serde's JSON serialization puts the fields in
            // alphabetical order.
            let pretty = concat!(
                "{",
                "\"items\":\"string\",",
                "\"type\":\"array\"",
                "}"
            );

            assert_eq!(s, pretty);
        }

        mod builder {
            use ravro::schema::{ArrayBuilder, Schema};

            #[test]
            fn is_array() {
                let a = ArrayBuilder::new()
                    .unwrap();

                assert!(a.is_array());
            }

            #[test]
            fn has_items() {
                let a = ArrayBuilder::new()
                    .items(Schema::String(String::from("boolean")))
                    .unwrap();

                let s = a.to_string();

                let pretty = concat!(
                    "{",
                    "\"items\":{\"type\":\"boolean\"},",
                    "\"type\":\"array\"",
                    "}"
                );

                assert_eq!(s, pretty);
            }
        }
    }

    mod map {
        use ravro::schema::Schema;
        use serde::json::builder::ObjectBuilder;

        #[test]
        fn is_map() {
            let val = ObjectBuilder::new()
                .insert(String::from("type"), String::from("map"))
                .insert(String::from("values"), String::from("string"))
                .unwrap();
            let o = Schema::Object(val);

            assert!(o.is_map());
        }

        #[test]
        fn to_string() {
            let val = ObjectBuilder::new()
                .insert(String::from("type"), String::from("map"))
                .insert(String::from("values"), String::from("string")) // technically, the items value is a Schema.
                .unwrap();
            let o = Schema::Object(val);

            let s = o.to_string();
            // It's in this order because Serde's JSON serialization puts the fields in
            // alphabetical order.
            let pretty = concat!(
                "{",
                "\"type\":\"map\",",
                "\"values\":\"string\"",
                "}"
            );

            assert_eq!(s, pretty);
        }

        mod builder {
            use ravro::schema::{MapBuilder, Schema};

            #[test]
            fn is_map() {
                let a = MapBuilder::new()
                    .unwrap();

                assert!(a.is_map());
            }

            #[test]
            fn has_values() {
                let a = MapBuilder::new()
                    .values(Schema::String(String::from("boolean")))
                    .unwrap();

                let s = a.to_string();

                let pretty = concat!(
                    "{",
                    "\"type\":\"map\",",
                    "\"values\":{\"type\":\"boolean\"}",
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
