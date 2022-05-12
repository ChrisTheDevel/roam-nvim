//! Backend struct. Here all functionality is exposed to the lua part of the plugin
use std::path::PathBuf;

use crate::database::Database;
use crate::node_dir::NodesDir;
use crate::error::BackendError;
use crate::types::{Alias, Title};

/// Struct which exposes wanted functionality of backend code to the lua part of the plugin.
pub struct Backend {
    database: Database,
    node_dir: NodesDir,
}

impl Backend {
    /// Tries to create a backend using the options provided by the user
    pub fn new(options: BackendOptions) -> Result<Backend,BackendError> {
        todo!()
    }

    /// Gets the backlinks of a specific file
    pub fn get_backlinks(&self, path:PathBuf) -> Vec<PathBuf> {
        // queries the in memory representation of the graph
        todo!()
    }

    /// creates an alias which can be searched for instead of node title
    pub fn create_alias(&mut self, alias: String, path: PathBuf) {
        todo!()
    }

    /// retreieves the path corresponding to a specific alias
    pub fn get_path_alias(&self, alias: String) -> PathBuf {
        todo!()
    }

    /// Gets all alias path pairs
    pub fn get_all_alias_paths(&self) -> Vec<(Alias, PathBuf)> {
        todo!()
    }

    /// Gets a list of all title path pairs
    pub fn get_title_paths(&self) -> Vec<(Title, PathBuf)>{
        // queries the sqlite database
        todo!()
    }

    /// checks node_dir of any files has changed (mtime and hash is different). If so then reparse
    /// those files
    pub fn sync(&mut self) {
        // get mtimes from notes dir
        // get mtimes from database
        // get list of paths from database
        // get list of paths from notedir
        // delete all paths in database not in notedir
        // for all paths with different mtimes
        // get hash from database
        // computer hash
        // if Different then we know file has changed (rather than just having been opened and saved
        // without any changes)
        // and we use the sync_node_unchecked function
        todo!()
    }

    /// checks out a specific file for any changes, and if neccessary updates the database and
    /// graph. If 
    pub fn sync_node(&mut self, node_path: PathBuf) {
        // get mtime from notes_dir
        // get mtime from database
        // if different compare hash
        // if different parse
        todo!()
    }

    /// syncs file without checking for changes (does not compare mtimes or hashes). Is to be used from within a checked context.
    pub fn sync_node_unchecked(&mut self, node_path: PathBuf) {
        todo!()
    }
}

/// A struct representing all options given to the backend.
pub struct BackendOptions;

