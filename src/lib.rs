//! A simple, connection pool library.
#![crate_name = "apns"]
#![crate_type = "lib"]
#![allow(unstable)]
#![unstable]
#![warn(missing_docs)]
#![feature(slicing_syntax)]

#[cfg(feature = "ssl")] extern crate openssl;

extern crate "rustc-serialize" as rustc_serialize;
#[macro_use]extern crate log;
extern crate "net-utils" as utils;

//extern crate hyper as http;
pub mod message;
