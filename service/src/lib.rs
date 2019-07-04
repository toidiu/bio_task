#![allow(unused)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate slog;

extern crate paseto;

#[macro_use]
mod std_ext;
mod errors;
mod global;

mod backend;
mod data;

pub mod server;
