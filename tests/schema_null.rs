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
