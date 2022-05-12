//! Rust backend for roam-nvim plugin
#[warn(missing_docs)]

#[macro_use]
extern crate diesel; // this gives us access to compile time validation of our schema.
#[macro_use]
extern crate diesel_migrations; // this gives us access to diesels migrations but built into the binary


pub mod backend;
pub mod database;
pub mod node_dir;
pub mod error;
pub mod types;
pub mod graph;
#[cfg(test)]
mod test_utils;

