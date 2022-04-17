//! This crate is the backend for the [rs-core nvim plugin]: https://github.com/ChrisTheDevel/rs-core
#![warn(missing_debug_implementations, missing_docs)]

#[macro_use]
extern crate diesel; // this gives us access to compile time validation of our schema.
#[macro_use]
extern crate diesel_migrations; // this gives us access to diesels migrations but built into the binary

mod database;
mod schema; // the sqlite schema macros
mod utils; // various utils // the database wrapper
