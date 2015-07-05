#![cfg_attr(feature = "nightly", feature(plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros))]

extern crate serde;
extern crate regex;

#[cfg(feature = "nightly")]
include!("lib.rs.in");

#[cfg(feature = "with_syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

pub mod schema;