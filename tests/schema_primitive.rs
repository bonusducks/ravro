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
