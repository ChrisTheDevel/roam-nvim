use crate::error::BackendError;
use std::{path::{PathBuf, Path}, ffi::OsStr, os::unix::prelude::MetadataExt};
use walkdir::WalkDir;
use blake2::{Blake2b512, Digest};


/// Struct which handles interaction with the markdown files which builds up the graph
/// contains methods of interacting with the dir as a whole
pub struct NodesDir {
    /// dir location with all markdown notes that make up the graph
    nodes_dir: PathBuf,
    /// extension of the markdown files (default .md)
    nodes_extension: String,
    /// the blake2 hasher
    hasher: Blake2b512,
}

impl NodesDir {
    /// Gets a list of tuples containing the markdown file names in nodes_dir and their corresponding mtime
    /// Is used to check if the database cache is up to date.
    fn get_m_times(&self) -> Result<Vec<(PathBuf, i64)>, BackendError> {
        Ok(WalkDir::new(&self.nodes_dir).into_iter().filter_entry(|entry| {
            let path = entry.path();
            path.is_dir() || path.extension().and_then(OsStr::to_str) == Some(&self.nodes_extension)
        }).filter_map(|entry_result|{
            match entry_result {
                Ok(entry) => {
                    let path = entry.clone().into_path();
                    let mtime = path.metadata().expect("Could not retrieve metadata").mtime();
                    Some((path, mtime))
                },
                Err(_) => None,
            }
        }).collect())
    }

    fn get_hash(&mut self, path: &Path) -> Result<String, BackendError> {
        let mut file = std::fs::File::open(path).map_err(|_| {
            BackendError::NodesDirError { message: "Could not open the file to compute its hash".into() }
        })?;
        // the hasher implements the writer trait so we can simply stream the file data into the
        // hasher
        let _n_bytes_streamed = std::io::copy(&mut file, &mut self.hasher).map_err(|_| {
            BackendError::NodesDirError { message: "Could not hash file content".into() }
        })?;
        let hash = self.hasher.finalize_reset();
        let hash_string: String = base16ct::upper::encode_string(&hash);
        Ok(hash_string)
    }

}

pub struct NodesDirBuilder {
    nodes_dir: Option<PathBuf>,
    node_extension: Option<String>,
}

impl NodesDirBuilder {
    pub fn new() -> Self {
        Self {
            nodes_dir: None,
            node_extension: Some("md".into()),
        }
    }

    pub fn dir_path(mut self, path: PathBuf) -> Self {
        self.nodes_dir = Some(path);
        self
    }

    pub fn extension(mut self, extension: String) -> Self {
        self.node_extension = Some(extension);
        self
    }

    pub fn build(self) -> Result<NodesDir, BackendError> {
        if self.nodes_dir.is_none() {
            return Err(BackendError::NodesDirBuilderError {
                message: "Builder was not provided path to nodes dir!".into(),
            });
        }
        let dir_path = self.nodes_dir.unwrap();
        let extension = self.node_extension.unwrap();
        Ok(NodesDir {
            nodes_dir: dir_path,
            nodes_extension: extension,
            hasher: Blake2b512::new(), // the hasher is not something we provide to the user to configure
        })
    }
}
