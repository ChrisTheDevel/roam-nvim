//! This crate is the backend for the [rs-core nvim plugin]: https://github.com/ChrisTheDevel/rs-core
#![warn(missing_debug_implementations, missing_docs)]

#[macro_use]
extern crate diesel; // this gives us access to compile time validation of our schema.
#[macro_use]
extern crate diesel_migrations; // this gives us access to diesels migrations but built into the binary

mod database;
mod schema; // the sqlite schema macros
mod utils; // various utils // the database wrapper
mod nodes_dir; // ties the notes directory and sqlite cache together. This is the part that interfaces with the lua frontend.
#[cfg(test)]
mod test_utils; // util functions that we only want to use during tests
