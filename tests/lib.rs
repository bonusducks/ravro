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

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate ravro;
extern crate serde;

lazy_static! {
    pub static ref LOGGER_INIT: () = env_logger::init().unwrap();
}

macro_rules! test {
    ($f:ident, $body:block) => {
        #[test]
        fn $f() {
            use ::LOGGER_INIT;
            if *LOGGER_INIT == () {
                $body
            } else {
                panic!("failed to init logging system");
            }
        }
    }
}

mod primitive {
    mod is_primitive {
        use ravro::schema::Schema;
        use serde::json::{self, Value};

        test!{is_bool, {
            let s = Schema::String(String::from("boolean"));
            assert!(s.is_primitive());
        }}

        test!{is_null, {
            let s = Schema::String(String::from("null"));
            assert!(s.is_primitive());
        }}

        test!{is_int, {
            let s = Schema::String(String::from("int"));
            assert!(s.is_primitive());
        }}

        test!{is_long, {
            let s = Schema::String(String::from("long"));
            assert!(s.is_primitive());
        }}

        test!{is_float, {
            let s = Schema::String(String::from("float"));
            assert!(s.is_primitive());
        }}

        test!{is_double, {
            let s = Schema::String(String::from("double"));
            assert!(s.is_primitive());
        }}

        test!{is_bytes, {
            let s = Schema::String(String::from("bytes"));
            assert!(s.is_primitive());
        }}

        test!{is_string, {
            let s = Schema::String(String::from("string"));
            assert!(s.is_primitive());
        }}

        test!{is_not_primitive_string, {
            let s = Schema::String(String::from("bogus"));
            assert_eq!(s.is_primitive(), false);
        }}

        test!{union_is_not_primitive, {
            let u = Schema::Union(vec!());
            assert_eq!(u.is_primitive(), false);
        }}

        test!{object_is_not_primitive, {
            let val : Value = json::from_str(r#"{"type":"string"}"#).unwrap(); // about as simple an object as we can get
            let s = Schema::Object(val);

            assert_eq!(s.is_primitive(), false);
        }}

        test!{null_is_not_primitive, {
            let n = Schema::Null;
            assert_eq!(n.is_primitive(), false);
        }}
    }

    mod ser {
        use ravro::schema::{self, Schema};

        test!{null_type, {
            // Note that the "null" primitive type isn't the same as Schema::Null
            let n = Schema::String(String::from("null"));
            let s = schema::to_string(&n).unwrap();

            assert_eq!(s, String::from(r#""null""#));
        }}

        test!{boolean_type, {
            let b = Schema::String(String::from("boolean"));
            let s = schema::to_string(&b).unwrap();

            assert_eq!(s, String::from(r#""boolean""#));
        }}

        test!{int_type, {
            let b = Schema::String(String::from("int"));
            let s = schema::to_string(&b).unwrap();

            assert_eq!(s, String::from(r#""int""#));
        }}

        test!{long_type, {
            let b = Schema::String(String::from("long"));
            let s = schema::to_string(&b).unwrap();

            assert_eq!(s, String::from(r#""long""#));
        }}

        test!{float_type, {
            let b = Schema::String(String::from("float"));
            let s = schema::to_string(&b).unwrap();

            assert_eq!(s, String::from(r#""float""#));
        }}

        test!{double_type, {
            let b = Schema::String(String::from("double"));
            let s = schema::to_string(&b).unwrap();

            assert_eq!(s, String::from(r#""double""#));
        }}

        test!{bytes_type, {
            let b = Schema::String(String::from("bytes"));
            let s = schema::to_string(&b).unwrap();

            assert_eq!(s, String::from(r#""bytes""#));
        }}

        test!{string_type, {
            let b = Schema::String(String::from("string"));
            let s = schema::to_string(&b).unwrap();

            assert_eq!(s, String::from(r#""string""#));
        }}
    }

    mod de {
        use ravro::schema::{self, Schema};

        test!{null_type, {
            // Note that the "null" primitive type isn't the same as Schema::Null
            let s = Schema::String(String::from("null"));
            let n = schema::from_str(r#""null""#).unwrap();

            assert_eq!(s, n);
        }}

        test!{boolean_type, {
            let s = Schema::String(String::from("boolean"));
            let b = schema::from_str(r#""boolean""#).unwrap();

            assert_eq!(s, b);
        }}

        test!{int_type, {
            let s = Schema::String(String::from("int"));
            let i = schema::from_str(r#""int""#).unwrap();

            assert_eq!(s, i);
        }}

        test!{long_type, {
            let s = Schema::String(String::from("long"));
            let l = schema::from_str(r#""long""#).unwrap();

            assert_eq!(s, l);
        }}

        test!{float_type, {
            let s = Schema::String(String::from("float"));
            let f = schema::from_str(r#""float""#).unwrap();

            assert_eq!(s, f);
        }}

        test!{double_type, {
            let s = Schema::String(String::from("double"));
            let d = schema::from_str(r#""double""#).unwrap();

            assert_eq!(s, d);
        }}

        test!{bytes_type, {
            let s = Schema::String(String::from("bytes"));
            let b = schema::from_str(r#""bytes""#).unwrap();

            assert_eq!(s, b);
        }}

        test!{string_type, {
            let s = Schema::String(String::from("string"));
            let st = schema::from_str(r#""string""#).unwrap();

            assert_eq!(s, st);
        }}
    }

    mod is_valid {
        use ravro::schema::Schema;

        test!{null_type, {
            let s = Schema::String(String::from("null"));
            assert!(s.is_valid().is_ok());
        }}

        test!{good_name, {
            let s = Schema::String(String::from("foobar"));
            assert!(s.is_valid().is_ok());
        }}

        test!{bad_name, {
            let s = Schema::String(String::from(";;foobar;;"));
            assert!(s.is_valid().is_err());
        }}
    }
}

mod union {
    mod is_union {
        use ravro::schema::Schema;
        use serde::json::{self, Value};

        test!{is_simple_union, {
            let s1 = Schema::String(String::from("boolean"));
            let s2 = Schema::String(String::from("int"));

            let union = Schema::Union(vec!(s1, s2));

            assert!(union.is_union());
        }}

        test!{is_empty_union, {
            // I haven't seen anything that says an empty Avro union is illegal, although
            // it certainly would be very useufl...
            let union = Schema::Union(vec!());

            assert!(union.is_union());
        }}

        test!{primitive_is_not_union, {
            let s = Schema::String(String::from("boolean"));
            assert_eq!(s.is_union(), false);
        }}

        test!{object_is_not_union, {
            let val : Value = json::from_str(r#"{"type":"string"}"#).unwrap(); // about as simple an object as we can get
            let s = Schema::Object(val);

            assert_eq!(s.is_union(), false);
        }}

        test!{null_is_not_union, {
            let n = Schema::Null;
            assert_eq!(n.is_union(), false);
        }}
    }

    mod ser {
        use ravro::schema::{self, Schema};

        test!{union_of_primitives, {
            let s1 = Schema::String(String::from("boolean"));
            let s2 = Schema::String(String::from("int"));
            let union = Schema::Union(vec!(s1, s2));
            let s = schema::to_string(&union).unwrap();

            assert_eq!(s, String::from(r#"["boolean","int"]"#));
        }}
    }

    mod de {
        use ravro::schema::{self, Schema, UnionBuilder};

        test!{union_1, {
            let u = UnionBuilder::new()
                        .push_record(|bld|
                            bld
                                .name("foo")
                                .namespace("x.y")
                                .doc("bar baz")
                                .fields(|fab|
                                    fab.push(|fb| fb.name("f1").field_type(Schema::String(String::from("int"))) )
                                       .push(|fb| fb.name("f2").field_type(Schema::String(String::from("boolean"))) )
                                )
                        )
                        .unwrap();

            let pretty = concat!(
                "[{",
                "\"doc\":\"bar baz\",",
                "\"fields\":[",
                "{\"name\":\"f1\",\"type\":\"int\"},",
                "{\"name\":\"f2\",\"type\":\"boolean\"}],",
                "\"name\":\"foo\",",
                "\"namespace\":\"x.y\",",
                "\"type\":\"record\"",
                "}]"
            );

            let u2 = schema::from_str(pretty).unwrap();

            assert_eq!(u, u2);
        }}

        test!{union_2, {
            let u = UnionBuilder::new()
                        .push_schema(Schema::String(String::from("int")))
                        .push_schema(Schema::String(String::from("boolean")))
                        .unwrap();

            // Converting to the simplified string type representation works only
            // at the "top level" currently.
            let pretty = "[\"int\",\"boolean\"]";

            let u2 = schema::from_str(pretty).unwrap();

            assert_eq!(u, u2);
        }}

        test!{union_3, {
            let u = UnionBuilder::new()
                        .push_map(|bld|
                            bld.values(Schema::String(String::from("string")))
                        )
                        .unwrap();

            let pretty = concat!(
                "[{",
                "\"type\":\"map\",",
                "\"values\":{\"type\":\"string\"}",
                "}]"
            );

            let u2 = schema::from_str(pretty).unwrap();

            assert_eq!(u, u2);
        }}

        test!{cannot_nest_arrays, {
            // So this is valid JSON, but according to the schema spec, unions are not allowed to
            // nest.
            let pretty = concat!(
                "[{",
                "\"type\":\"map\",",
                "\"values\":{\"type\":\"string\"}",
                "},",
                "[\"int\",\"boolean\"],",
                "]"
            );

            let u = schema::from_str(pretty);

            assert!(u.is_err());
        }}
    }

    mod builder {
        use ravro::schema::{Schema, UnionBuilder};
        //use serde::json::Value;

        test!{is_union, {
            let u = UnionBuilder::new()
                    .unwrap();

            assert!(u.is_union());
            assert!(u.fullname().is_err()); // unions don't have names
        }}

        test!{has_fixed, {
            let u = UnionBuilder::new()
                        .push_fixed(|bld|
                            bld.name("md5").namespace("x.y").size(16)
                        )
                        .unwrap();

            let s = String::from(&u);

            let pretty = concat!(
                "[",
                "{\"name\":\"md5\",",
                "\"namespace\":\"x.y\",",
                "\"size\":16,",
                "\"type\":\"fixed\"}",
                "]"
            );

            assert_eq!(s, pretty);
        }}

        test!{has_map, {
            let u = UnionBuilder::new()
                        .push_map(|bld|
                            bld.values(Schema::String(String::from("string")))
                        )
                        .unwrap();

            let s = String::from(&u);

            let pretty = concat!(
                "[{",
                "\"type\":\"map\",",
                "\"values\":{\"type\":\"string\"}",
                "}]"
            );

            assert_eq!(s, pretty);
        }}

        test!{has_array, {
            let u = UnionBuilder::new()
                        .push_array(|bld|
                            bld.items(Schema::String(String::from("string")))
                        )
                        .unwrap();

            let s = String::from(&u);

            let pretty = concat!(
                "[{",
                "\"items\":{\"type\":\"string\"},",
                "\"type\":\"array\"",
                "}]"
            );

            assert_eq!(s, pretty);
        }}

        test!{has_enum, {
            let u = UnionBuilder::new()
                        .push_enum(|bld|
                            bld
                                .name("foo")
                                .namespace("x.y")
                                .doc("bar baz")
                                .symbols(|sb| sb.push("A1").push("A2") )
                        )
                        .unwrap();

            let s = String::from(&u);

            let pretty = concat!(
                "[{",
                "\"doc\":\"bar baz\",",
                "\"name\":\"foo\",",
                "\"namespace\":\"x.y\",",
                "\"symbols\":[\"A1\",\"A2\"],",
                "\"type\":\"enum\"",
                "}]"
            );

            assert_eq!(s, pretty);
        }}

        test!{has_record, {
            let u = UnionBuilder::new()
                        .push_record(|bld|
                            bld
                                .name("foo")
                                .namespace("x.y")
                                .doc("bar baz")
                                .fields(|fab|
                                    fab.push(|fb| fb.name("f1").field_type(Schema::String(String::from("int"))) )
                                       .push(|fb| fb.name("f2").field_type(Schema::String(String::from("boolean"))) )
                                )
                        )
                        .unwrap();

            let s = String::from(&u);

            let pretty = concat!(
                "[{",
                "\"doc\":\"bar baz\",",
                "\"fields\":[",
                "{\"name\":\"f1\",\"type\":\"int\"},",
                "{\"name\":\"f2\",\"type\":\"boolean\"}],",
                "\"name\":\"foo\",",
                "\"namespace\":\"x.y\",",
                "\"type\":\"record\"",
                "}]"
            );

            assert_eq!(s, pretty);
        }}

        test!{has_schema, {
            let u = UnionBuilder::new()
                        .push_schema(Schema::String(String::from("int")))
                        .push_schema(Schema::String(String::from("boolean")))
                        .unwrap();

            let s = String::from(&u);

            let pretty = "[\"int\",\"boolean\"]";

            assert_eq!(s, pretty);
        }}
    }
}

mod object {
    mod is_object {
        use ravro::schema::Schema;
        use serde::json::{self, Value};

        test!{is_object, {
            let val : Value = json::from_str(r#"{"type":"string"}"#).unwrap(); // about as simple an object as we can get
            let s = Schema::Object(val);

            assert!(s.is_object());
        }}

        test!{primitive_is_not_object, {
            let s = Schema::String(String::from("boolean"));
            assert_eq!(s.is_object(), false);
        }}

        test!{union_is_not_object, {
            let u = Schema::Union(vec![]);
            assert_eq!(u.is_object(), false);
        }}

        test!{null_is_not_object, {
            let n = Schema::Null;
            assert_eq!(n.is_object(), false);
        }}
    }

    mod as_object {
        use ravro::schema::Schema;
        use serde::json::{self, Value};

        test!{object_as_object, {
            let val : Value = json::from_str(r#"{"type":"string"}"#).unwrap();
            let o = Schema::Object(val);
            let o2 = o.as_object().unwrap();

            assert_eq!(o, o2);
        }}

        // Based on this test, I'm not going to go through all 8 primitive types.
        test!{boolean_as_object, {
            let val : Value = json::from_str(r#"{"type":"boolean"}"#).unwrap();
            let o = Schema::Object(val);

            let b = Schema::String(String::from("boolean"));
            let o2 = b.as_object().unwrap();

            assert_eq!(o, o2);
        }}

        // Not sure if this is strictly corrct...
        test!{union_as_object, {
            // While the following line is hte simplest representation of the union, because
            // the implementation is doing to_object on each element, we are getting the
            // longer, {"type":"blah"} representation. This may not be a good thing in the
            // long run.
            //let val : Value = json::from_str(r#"["boolean","int"]"#).unwrap();
            let val : Value = json::from_str(r#"[{"type":"boolean"},{"type":"int"}]"#).unwrap();
            let o = Schema::Object(val);

            let s1 = Schema::String(String::from("boolean"));
            let s2 = Schema::String(String::from("int"));
            let union = Schema::Union(vec!(s1, s2));

            let o2 = union.as_object().unwrap();

            assert_eq!(o, o2);
        }}

        test!{null_as_object, {
            let val : Value = json::from_str(r#"{"type":"null"}"#).unwrap();
            let o = Schema::Object(val);

            let n = Schema::Null;
            let o2 = n.as_object().unwrap();

            assert_eq!(o, o2);
        }}
    }

    // The Schema::Object type can be specialized as a record.
    mod record {
        use ravro::schema::Schema;
        use serde::json::builder::ObjectBuilder;

        test!{is_record, {
            let val = ObjectBuilder::new()
                .insert(String::from("type"), String::from("record"))
                .insert(String::from("name"), String::from("foo"))
                .insert_array(String::from("fields"), |bld| bld)   // empty field array
                .unwrap();
            let o = Schema::Object(val);

            assert!(o.is_record());
        }}

        test!{to_string, {
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
        }}

        test!{has_doc, {
            let val = ObjectBuilder::new()
                .insert(String::from("type"), String::from("record"))
                .insert(String::from("name"), String::from("foo"))
                .insert(String::from("doc"), String::from("yadda yadda"))
                .insert_array(String::from("fields"), |bld| bld)   // empty field array
                .unwrap();
            let o = Schema::Object(val);

            assert_eq!(o.doc().unwrap(), "yadda yadda");
        }}

        mod fullname {
            use ravro::schema::Schema;
            use serde::json::builder::ObjectBuilder;

            test!{simple_name, {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from("foo"))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().unwrap(), String::from("foo"));
            }}

            test!{name_with_namespace, {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from("foo"))
                    .insert(String::from("namespace"), String::from("x.y"))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().unwrap(), String::from("x.y.foo"));
            }}

            test!{name_with_dots, {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from("a.b.foo"))
                    .insert(String::from("namespace"), String::from("x.y"))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().unwrap(), String::from("a.b.foo"));
            }}

            test!{name_cannot_be_empty, {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from(""))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().is_err(), true);
            }}

            test!{name_cannot_end_with_period, {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from("foo."))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().is_err(), true);
            }}

            test!{name_cannot_start_with_period, {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from(".foo"))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().is_err(), true);
            }}

            test!{name_cannot_start_with_number, {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from("9foo"))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().is_err(), true);
            }}

            test!{namespace_with_more_complicated_segments, {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from("foo"))
                    .insert(String::from("namespace"), String::from("Yadda_.FooBar12_34.blah_blah_blah"))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().unwrap(), String::from("Yadda_.FooBar12_34.blah_blah_blah.foo"));
            }}

            test!{namespace_cannot_have_trailing_period, {
                let val = ObjectBuilder::new()
                    .insert(String::from("type"), String::from("record"))
                    .insert(String::from("name"), String::from("foo"))
                    .insert(String::from("namespace"), String::from("x.y."))
                    .insert_array(String::from("fields"), |bld| bld)   // empty field array
                    .unwrap();
                let o = Schema::Object(val);

                assert_eq!(o.fullname().is_err(), true);
            }}

            // These name tests aren't exhaustive; they can't be really unless I come up with
            // a state machine to drive all possible permutations of the regex provided in the
            // specification. So while this isn't perfect, and I'm not happy, I'm hoping I've
            // covered the most significant cases.
        }

        mod ser {
            use ravro::schema::Schema;
            use serde::json::builder::ObjectBuilder;

            test!{rec_1, {
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
            }}
        }

        mod de {
            use ravro::schema::{self, RecordBuilder, Schema};

            test!{rec_1, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab| fab )  // empty fields array
                    .unwrap();

                let pretty = concat!(
                    "{",
                    "\"fields\":[],",
                    "\"name\":\"foo\",",
                    "\"type\":\"record\"",
                    "}"
                );

                let r2 = schema::from_str(pretty).unwrap();

                assert_eq!(r, r2);
            }}

            test!{rec_2, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|bld|
                        bld
                        .push(|fb|
                            fb.name("bar")
                              .field_type(Schema::String(String::from("boolean")))
                        )
                        .push(|fb|
                            fb.name("baz")
                              .field_type(Schema::String(String::from("int")))
                              .doc("yadda yadda")
                        )
                    )
                    .unwrap();

                // It's in this order because Serde's JSON serialization puts the fields in
                // alphabetical order, which is not the Avro cannonical order.
                let pretty = concat!(
                    "{",
                    "\"fields\":[",
                    "{\"name\":\"bar\",\"type\":\"boolean\"},",
                    "{\"doc\":\"yadda yadda\",\"name\":\"baz\",\"type\":\"int\"}",
                    "],",
                    "\"name\":\"foo\",",
                    "\"type\":\"record\"",
                    "}"
                );

                let r2 = schema::from_str(pretty).unwrap();

                assert_eq!(r, r2);
            }}

            test!{rec_3, {
                let aliases_vec = vec!["bar", "baz"];

                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb.name("bar")
                              .field_type(Schema::String(String::from("string")))
                              .aliases(aliases_vec)
                        )
                    )
                    .unwrap();

                // It's in this order because Serde's JSON serialization puts the fields in
                // alphabetical order, which is not the Avro cannonical order.
                let pretty = concat!(
                    "{",
                    "\"fields\":[{\"aliases\":[\"bar\",\"baz\"],\"name\":\"bar\",\"type\":\"string\"}],",
                    "\"name\":\"foo\",",
                    "\"type\":\"record\"",
                    "}"
                );

                let r2 = schema::from_str(pretty).unwrap();

                assert_eq!(r, r2);
            }}
        }

        mod builder {
            use ravro::schema::{FieldSortOrder, RecordBuilder, Schema};
            use serde::json::Value;

            test!{is_record, {
                let r = RecordBuilder::new()
                    .unwrap();

                assert!(r.is_record());
                assert!(r.fullname().is_err()); // didn't assign a name to the record.
            }}

            test!{has_name, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .unwrap();

                assert!(r.is_record());
                assert_eq!(r.fullname().unwrap(), "foo");
            }}

            test!{has_namespace, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .namespace("x.y")
                    .unwrap();

                assert_eq!(r.fullname().unwrap(), "x.y.foo");
            }}

            test!{has_doc, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .doc("yadda yadda")
                    .unwrap();

                assert_eq!(r.doc().unwrap(), "yadda yadda");
            }}

            test!{has_aliases, {
                let aliases_vec = vec!["bar", "baz"];
                let r = RecordBuilder::new()
                    .name("foo")
                    .aliases(aliases_vec)
                    .unwrap();

                assert_eq!(r.aliases().unwrap(), vec![String::from("bar"), String::from("baz")]);
            }}

            test!{has_one_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb.name("bar")
                              .field_type(Schema::String(String::from("boolean")))
                        )
                    )
                    .unwrap();

                let s = String::from(&r);
                // It's in this order because Serde's JSON serialization puts the fields in
                // alphabetical order, which is not the Avro cannonical order.
                let pretty = concat!(
                    "{",
                    "\"fields\":[{\"name\":\"bar\",\"type\":\"boolean\"}],",
                    "\"name\":\"foo\",",
                    "\"type\":\"record\"",
                    "}"
                );

                assert_eq!(s, pretty);
            }}

            test!{has_multiple_fields, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|bld|
                        bld
                        .push(|fb|
                            fb.name("bar")
                              .field_type(Schema::String(String::from("boolean")))
                        )
                        .push(|fb|
                            fb.name("baz")
                              .field_type(Schema::String(String::from("int")))
                              .doc("yadda yadda")
                        )
                    )
                    .unwrap();

                let s = String::from(&r);
                // It's in this order because Serde's JSON serialization puts the fields in
                // alphabetical order, which is not the Avro cannonical order.
                let pretty = concat!(
                    "{",
                    "\"fields\":[",
                    "{\"name\":\"bar\",\"type\":\"boolean\"},",
                    "{\"doc\":\"yadda yadda\",\"name\":\"baz\",\"type\":\"int\"}",
                    "],",
                    "\"name\":\"foo\",",
                    "\"type\":\"record\"",
                    "}"
                );

                assert_eq!(s, pretty);
            }}

            test!{has_field_with_order, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb.name("bar")
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
                    "\"fields\":[{\"name\":\"bar\",\"order\":\"ascending\",\"type\":\"string\"}],",
                    "\"name\":\"foo\",",
                    "\"type\":\"record\"",
                    "}"
                );

                assert_eq!(s, pretty);
            }}

            test!{has_field_with_aliases, {
                let aliases_vec = vec!["bar", "baz"];

                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb.name("bar")
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
                    "\"fields\":[{\"aliases\":[\"bar\",\"baz\"],\"name\":\"bar\",\"type\":\"string\"}],",
                    "\"name\":\"foo\",",
                    "\"type\":\"record\"",
                    "}"
                );

                assert_eq!(s, pretty);
            }}

            test!{has_field_with_default, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb.name("bar")
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
                    "\"fields\":[{\"default\":\"one two three\",\"name\":\"bar\",\"type\":\"string\"}],",
                    "\"name\":\"foo\",",
                    "\"type\":\"record\"",
                    "}"
                );

                assert_eq!(s, pretty);
            }}
        }

        mod is_valid {
            use ravro::schema::{ArrayBuilder, MapBuilder, RecordBuilder, Schema};
            use ravro::schema::error::{Error, ErrorCode};
            use serde::json::{self, Value};

            test!{missing_name, {
                let r = RecordBuilder::new()
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::NotWellFormedName);
                } else {
                    assert!(false);
                }
            }}

            test!{missing_fields, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::ExpectedFieldDefintion);
                } else {
                    assert!(false);
                }
            }}

            test!{empty_field_array_ok, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab| fab ) // has effect of creating empty fields array, which is OK according to the spc.
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_ok());
            }}

            test!{field_missing_name, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab| 
                        fab.push(|fb| fb.field_type(Schema::String(String::from("string"))) ) 
                    ) 
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::FieldNameNotWellFormed);
                } else {
                    assert!(false);
                }
            }}

            test!{field_has_bad_name, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab| 
                        fab.push(|fb| fb.name(";;foo;;") ) 
                    ) 
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::FieldNameNotWellFormed);
                } else {
                    assert!(false);
                }
            }}

            test!{field_missing_type, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab| 
                        fab.push(|fb| fb.name("bar") ) 
                    ) 
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::ExpectedFieldTypeAttribute);
                } else {
                    assert!(false);
                }
            }}

            test!{bad_default_for_string_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(Schema::String(String::from("string")))
                                .default(Value::U64(99))
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::FieldDefaultTypeMismatch);
                } else {
                    assert!(false);
                }
            }}

            test!{ok_default_for_string_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(Schema::String(String::from("string")))
                                .default(Value::String(String::from("yadda")))
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_ok());
            }}

            test!{bad_default_for_int_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(Schema::String(String::from("int")))
                                .default(Value::String(String::from("yadda")))
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::FieldDefaultTypeMismatch);
                } else {
                    assert!(false);
                }
            }}

            test!{ok_default_for_int_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(Schema::String(String::from("int")))
                                .default(Value::I64(10i64))
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_ok());
            }}

            test!{bad_default_for_null_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(Schema::String(String::from("null")))
                                .default(Value::String(String::from("yadda")))
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::FieldDefaultTypeMismatch);
                } else {
                    assert!(false);
                }
            }}

            test!{ok_default_for_null_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(Schema::String(String::from("null")))
                                .default(Value::Null)
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_ok());
            }}

            test!{bad_default_for_boolean_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(Schema::String(String::from("boolean")))
                                .default(Value::String(String::from("yadda")))
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::FieldDefaultTypeMismatch);
                } else {
                    assert!(false);
                }
            }}

            test!{ok_default_for_boolean_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(Schema::String(String::from("boolean")))
                                .default(Value::Bool(true))
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_ok());
            }}

            test!{bad_default_for_long_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(Schema::String(String::from("long")))
                                .default(Value::String(String::from("yadda")))
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::FieldDefaultTypeMismatch);
                } else {
                    assert!(false);
                }
            }}

            test!{ok_default_for_long_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(Schema::String(String::from("long")))
                                .default(Value::U64(1_000_000_000))
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_ok());
            }}

            test!{bad_default_for_float_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(Schema::String(String::from("float")))
                                .default(Value::String(String::from("yadda")))
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::FieldDefaultTypeMismatch);
                } else {
                    assert!(false);
                }
            }}

            test!{ok_default_for_float_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(Schema::String(String::from("float")))
                                .default(Value::F64(1.1))
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_ok());
            }}

            test!{bad_default_for_bytes_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(Schema::String(String::from("bytes")))
                                .default(Value::F64(1.1))
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::FieldDefaultTypeMismatch);
                } else {
                    assert!(false);
                }
            }}

            test!{ok_default_for_bytes_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(Schema::String(String::from("bytes")))
                                .default(Value::String(String::from("\\u00FF")))
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_ok());
            }}

            test!{bad_default_for_named_type_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(Schema::String(String::from("LongList")))
                                .default(Value::F64(1.1))
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::FieldDefaultTypeMismatch);
                } else {
                    assert!(false);
                }
            }}

            test!{ok_default_for_named_type_field, {
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                // Realistically, this field type would be ["null", "LongList"],
                                // and the actual default value would be "null" and "LongList"
                                // would be the name of a record type.
                                .field_type(Schema::String(String::from("LongList")))
                                .default(Value::String(String::from("blah")))
                        )
                    )
                    .unwrap();

                // NOTE: This test is going to break once I implement verifything that the
                //       type name actually exists.
                let valid = r.is_valid();
                assert!(valid.is_ok());
            }}

            test!{bad_default_for_record_type_field, {
                let r1 = RecordBuilder::new()
                    .name("bar")
                    .fields(|fab|
                        fab.push(|fb|
                            fb.name("yadda")
                              .field_type(Schema::String(String::from("string")))
                        )
                    )
                    .unwrap();

                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(r1)
                                .default(Value::String(String::from("broken")))
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::FieldDefaultTypeMismatch);
                } else {
                    assert!(false);
                }
            }}

            test!{good_default_for_record_type_field, {
                let r1 = RecordBuilder::new()
                    .name("bar")
                    .fields(|fab|
                        fab.push(|fb|
                            fb.name("yadda")
                              .field_type(Schema::String(String::from("string")))
                        )
                    )
                    .unwrap();

                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(r1)
                                .default(json::from_str(r#"{"yadda":"blah"}"#).unwrap())
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_ok());
            }}

            test!{bad_default_for_union_type_field, {
                let v = vec![Schema::String(String::from("null")), Schema::String(String::from("string"))];
                let u = Schema::Union(v);
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(u)
                                .default(Value::F64(1.1))
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::FieldDefaultTypeMismatch);
                } else {
                    assert!(false);
                }
            }}

            test!{ok_default_for_union_type_field, {
                let v = vec![Schema::String(String::from("null")), Schema::String(String::from("string"))];
                let u = Schema::Union(v);
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(u)
                                .default(Value::Null)
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_ok());
            }}

            test!{cannot_have_two_array_types_in_union_type_field, {
                let a1 = ArrayBuilder::new()
                    .items(Schema::String(String::from("int")))
                    .unwrap();
                let a2 = ArrayBuilder::new()
                    .items(Schema::String(String::from("string")))
                    .unwrap();

                let v = vec![a1, a2];
                let u = Schema::Union(v);
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(u)
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::FieldTooManyElementsOfSameType);
                } else {
                    assert!(false);
                }
            }}

            test!{cannot_have_two_map_types_in_union_type_field, {
                let a1 = MapBuilder::new()
                    .values(Schema::String(String::from("int")))
                    .unwrap();
                let a2 = MapBuilder::new()
                    .values(Schema::String(String::from("string")))
                    .unwrap();

                let v = vec![a1, a2];
                let u = Schema::Union(v);
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(u)
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_err());

                if let Some(Error::SyntaxError(code, _, _)) = valid.err() {
                    assert_eq!(code, ErrorCode::FieldTooManyElementsOfSameType);
                } else {
                    assert!(false);
                }
            }}

            test!{one_array_one_map_in_union_type_field, {
                let a1 = MapBuilder::new()
                    .values(Schema::String(String::from("int")))
                    .unwrap();
                let a2 = ArrayBuilder::new()
                    .items(Schema::String(String::from("string")))
                    .unwrap();

                let v = vec![a1, a2];
                let u = Schema::Union(v);
                let r = RecordBuilder::new()
                    .name("foo")
                    .fields(|fab|
                        fab.push(|fb|
                            fb
                                .name("bar")
                                .field_type(u)
                        )
                    )
                    .unwrap();

                let valid = r.is_valid();
                assert!(valid.is_ok());
            }}
        }
    }

    mod enum_type {// enum is a keyword...
        use ravro::schema::Schema;
        use serde::json::builder::ObjectBuilder;

        test!{is_enum, {
            let val = ObjectBuilder::new()
                .insert(String::from("type"), String::from("enum"))
                .insert(String::from("name"), String::from("foo"))
                .insert_array(String::from("symbols"), |bld| bld.push(String::from("A1")) )
                .unwrap();
            let o = Schema::Object(val);

            assert!(o.is_enum());
        }}

        test!{to_string, {
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
        }}

        mod de {
            use ravro::schema::{self, EnumBuilder};
            
            test!{enum_1, {
                let e = EnumBuilder::new()
                    .name("foo")
                    .symbols(|bld|
                        bld.push("A1").push("A2")
                    )
                    .unwrap();

                let pretty = concat!(
                    "{",
                    "\"name\":\"foo\",",
                    "\"symbols\":[\"A1\",\"A2\"],",
                    "\"type\":\"enum\"",
                    "}"
                );

                let e2 = schema::from_str(pretty).unwrap();

                assert_eq!(e, e2);
            }}
        }

        mod builder {
            use ravro::schema::EnumBuilder;

            test!{is_enum, {
                let e = EnumBuilder::new()
                    .unwrap();

                assert!(e.is_enum());
                assert!(e.fullname().is_err()); // didn't assign a name to the enum.
            }}

            test!{has_name, {
                let e = EnumBuilder::new()
                    .name("foo")
                    .unwrap();

                assert_eq!(e.fullname().unwrap(), "foo");
            }}

            test!{has_namespace, {
                let e = EnumBuilder::new()
                    .name("foo")
                    .namespace("x.y")
                    .unwrap();

                assert_eq!(e.fullname().unwrap(), "x.y.foo");
            }}

            test!{has_doc, {
                let e = EnumBuilder::new()
                    .name("foo")
                    .doc("yadda yadda")
                    .unwrap();

                assert_eq!(e.doc().unwrap(), "yadda yadda");
            }}

            test!{has_aliases, {
                let aliases_vec = vec!["bar", "baz"];
                let e = EnumBuilder::new()
                    .name("foo")
                    .aliases(aliases_vec)
                    .unwrap();

                assert_eq!(e.aliases().unwrap(), vec![String::from("bar"), String::from("baz")]);
            }}

            test!{has_symbols, {
                let e = EnumBuilder::new()
                    .name("foo")
                    .symbols(|bld|
                        bld.push("A1").push("A2")
                    )
                    .unwrap();

                assert_eq!(e.symbols().unwrap(), vec![String::from("A1"), String::from("A2")]);
            }}
        }
    }

    mod array {
        use ravro::schema::Schema;
        use serde::json::builder::ObjectBuilder;

        test!{is_array, {
            let val = ObjectBuilder::new()
                .insert(String::from("type"), String::from("array"))
                .insert(String::from("items"), String::from("string"))
                .unwrap();
            let o = Schema::Object(val);

            assert!(o.is_array());
        }}

        test!{to_string, {
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
        }}

        mod de {
            use ravro::schema::{self, ArrayBuilder, Schema};

            test!{array_1, {
                let a = ArrayBuilder::new()
                    .items(Schema::String(String::from("boolean")))
                    .unwrap();

                let pretty = concat!(
                    "{",
                    "\"items\":{\"type\":\"boolean\"},",
                    "\"type\":\"array\"",
                    "}"
                );

                let a2 = schema::from_str(pretty).unwrap();

                assert_eq!(a, a2);
            }}
        }

        mod builder {
            use ravro::schema::{ArrayBuilder, Schema};

            test!{is_array, {
                let a = ArrayBuilder::new()
                    .unwrap();

                assert!(a.is_array());
            }}

            test!{has_items, {
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
            }}
        }
    }

    mod map {
        use ravro::schema::Schema;
        use serde::json::builder::ObjectBuilder;

        test!{is_map, {
            let val = ObjectBuilder::new()
                .insert(String::from("type"), String::from("map"))
                .insert(String::from("values"), String::from("string"))
                .unwrap();
            let o = Schema::Object(val);

            assert!(o.is_map());
        }}

        test!{to_string, {
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
        }}

        mod de {
            use ravro::schema::{self, MapBuilder, Schema};

            test!{map_1, {
                let a = MapBuilder::new()
                    .values(Schema::String(String::from("boolean")))
                    .unwrap();

                let pretty = concat!(
                    "{",
                    "\"type\":\"map\",",
                    "\"values\":{\"type\":\"boolean\"}",
                    "}"
                );

                let a2 = schema::from_str(pretty).unwrap();

                assert_eq!(a, a2);
            }}
        }

        mod builder {
            use ravro::schema::{MapBuilder, Schema};

            test!{is_map, {
                let a = MapBuilder::new()
                    .unwrap();

                assert!(a.is_map());
            }}

            test!{has_values, {
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
            }}
        }
    }

    mod fixed {
        use ravro::schema::Schema;
        use serde::json::builder::ObjectBuilder;

        test!{is_map, {
            let val = ObjectBuilder::new()
                .insert(String::from("type"), String::from("fixed"))
                .insert(String::from("size"), 16)
                .insert(String::from("name"), String::from("md5"))
                .unwrap();
            let o = Schema::Object(val);

            assert!(o.is_fixed());
        }}

        test!{to_string, {
            let val = ObjectBuilder::new()
                .insert(String::from("type"), String::from("fixed"))
                .insert(String::from("size"), 16)
                .insert(String::from("name"), String::from("md5"))
                .unwrap();
            let o = Schema::Object(val);

            let s = o.to_string();
            // It's in this order because Serde's JSON serialization puts the fields in
            // alphabetical order.
            let pretty = concat!(
                "{",
                "\"name\":\"md5\",",
                "\"size\":16,",
                "\"type\":\"fixed\"",
                "}"
            );

            assert_eq!(s, pretty);
        }}

        mod de {
            use ravro::schema::{self, FixedBuilder};

            test!{fixed_1, {
                let f = FixedBuilder::new()
                    .name("md5")
                    .size(16)
                    .unwrap();

                let pretty = concat!(
                    "{",
                    "\"name\":\"md5\",",
                    "\"size\":16,",
                    "\"type\":\"fixed\"",
                    "}"
                );

                let f2 = schema::from_str(pretty).unwrap();

                assert_eq!(f, f2);
            }}
        }

        mod builder {
            use ravro::schema::FixedBuilder;

            test!{is_fixed, {
                let f = FixedBuilder::new()
                    .unwrap();

                assert!(f.is_fixed());
            }}

            test!{has_name, {
                let f = FixedBuilder::new()
                    .name("foo")
                    .unwrap();

                assert_eq!(f.fullname().unwrap(), "foo");
            }}

            test!{has_namespace, {
                let f = FixedBuilder::new()
                    .name("foo")
                    .namespace("x.y")
                    .unwrap();

                assert_eq!(f.fullname().unwrap(), "x.y.foo");
            }}

            test!{has_aliases, {
                let aliases_vec = vec!["bar", "baz"];
                let f = FixedBuilder::new()
                    .name("foo")
                    .aliases(aliases_vec)
                    .unwrap();

                assert_eq!(f.aliases().unwrap(), vec![String::from("bar"), String::from("baz")]);
            }}

            test!{has_size, {
                let f = FixedBuilder::new()
                    .name("foo")
                    .size(16)
                    .unwrap();

                assert_eq!(f.size().unwrap(), 16);
            }}
        }
    }
}

mod null {
    mod is_null {
        use ravro::schema::Schema;
        use serde::json::{self, Value};

        test!{is_null, {
            let n = Schema::Null;
            assert!(n.is_null())
        }}

        test!{primitive_is_not_null, {
            let s = Schema::String(String::from("boolean"));
            assert_eq!(s.is_null(), false);
        }}

        test!{union_is_not_null, {
            let u = Schema::Union(vec![]);
            assert_eq!(u.is_null(), false);
        }}

        test!{object_is_not_null, {
            let val : Value = json::from_str(r#"{"type":"string"}"#).unwrap(); // about as simple an object as we can get
            let s = Schema::Object(val);

            assert_eq!(s.is_null(), false);
        }}
    }

    mod ser {
        use ravro::schema::{self, Schema};

        test!{null_type, {
            // Note that the "null" primitive type isn't the same as Schema::Null
            let n = Schema::Null;
            let s = schema::to_string(&n).unwrap();

            assert_eq!(s, String::from(r#""null""#));
        }}
    }
}
