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

#[macro_use]
mod macros;

mod is_union {
    use ravro::schema::Schema;
    use serde::json::{self, Value};

    test!{is_simple_union, {
        let s1 = Schema::String(String::from("boolean"));
        let s2 = Schema::String(String::from("int"));

        let union = Schema::Union(vec!(s1, s2));

        assert!(union.is_union())
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
