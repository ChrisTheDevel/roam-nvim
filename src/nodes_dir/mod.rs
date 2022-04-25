mod builder;
// std imports
use std::{path::PathBuf, ffi::OsStr};
use std::fs;
use std::os::unix::fs::MetadataExt;

// external crate imports
use walkdir::WalkDir;


pub struct NodesDirWrapper<'a> {
    nodes_dir: PathBuf,
    nodes_extension: &'a str, // this string will live as long as the struct
}

impl NodesDirWrapper<'_> {
    /// Gets a list of tuples containing the markdown file names in nodes_dir and their corresponding mtime
    /// Is used to check if the database cache is up to date.
    fn get_m_times(&self) -> Result<Vec<(PathBuf,i64)>, walkdir::Error>{
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
}

#[derive(Clone, Debug)]
pub struct NodesDirError {
    message: &'static str
}

impl std::fmt::Display for NodesDirError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BackendError: {}", self.message)
    }
}

#[cfg(test)]
mod backend_test {

}
