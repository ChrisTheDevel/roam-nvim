//! This crate is the backend for the [rs-core nvim plugin]: https://github.com/ChrisTheDevel/rs-core

#![warn(missing_debug_implementations, missing_docs)]

#[macro_use]
extern crate diesel;

mod schema;
mod utils;
