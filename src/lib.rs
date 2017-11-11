extern crate tomorrow_core;

extern crate hyper;
extern crate hyper_native_tls;

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

pub const PACKAGE: &'static str = env!("CARGO_PKG_NAME");
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub mod json;
pub mod raw;

mod builder;
pub use self::builder::Builder;