extern crate serde;

// Pure unit tests
#[cfg(test)]
mod tests {
    //use super::*;
    use serde::json::{self, Value};


    #[test]
    fn extercise_json() {
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
}
