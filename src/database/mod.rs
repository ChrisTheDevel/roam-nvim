//! This module contains the wrapper struct for the sqlite database
pub mod builder;

use std::path::PathBuf;

// external crate imports
use diesel::prelude::*;

/// Database struct which defines wanted interaction with the sqlite database
pub struct DatabaseWrapper {
    connection: SqliteConnection,
}


impl DatabaseWrapper {

    fn add_file_to_cache(&mut self, file: PathBuf) {
        todo!()
    }

    /// returns list of titles together with their file paths.
    fn get_title_paths(&self) {
        todo!()
    }

    /// get the node titles and their paths of all nodes that link to this node
    fn get_backlinks(&self, path: PathBuf) {
        todo!()
    }
}

#[cfg(test)]
mod DatabaseWrapperTests {

}
