#![cfg_attr(feature = "nightly", feature(plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros))]

extern crate serde;

use std::cmp::PartialEq;

#[cfg(feature = "nightly")]
include!("lib.rs.in");

#[cfg(feature = "with_syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

impl PartialEq<Point> for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }

    fn ne(&self, other: &Point) -> bool {
        !self.eq(&other)
    } 
}

// Pure unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use serde::json::{self, Value};

    #[test]
    fn exercise_json() {
        // This is basically a cut-n-paste of the serde json example code, but
        // changing the println! statements into assertions. It's here just so
        // I can get some practice with the JSON deserialization code.

        let data: Value = json::from_str("{\"foo\": 13, \"bar\": \"baz\"}").unwrap();
        assert!(data.is_object());

        let obj = data.as_object().unwrap();
        let foo = obj.get("foo").unwrap();

        assert_eq!(foo.as_array(), None);
        assert_eq!(foo.is_number(), true);
        assert_eq!(foo.is_u64(), true);     // 13 can be coerced into a u64
        assert_eq!(foo.as_u64().unwrap(), 13);
        assert_eq!(foo.is_null(), false);
        assert_eq!(foo.is_f64(), false);
    }

    #[test]
    fn exercise_point() {
        let point = Point { x: 1, y: 2 };
        let ser_point = json::to_string(&point).unwrap();

        assert_eq!(ser_point, "{\"x\":1,\"y\":2}");

        let des_point : Point = json::from_str(&ser_point).unwrap();

        assert_eq!(point, des_point);
    }
}
