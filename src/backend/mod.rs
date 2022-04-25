mod builder;
// std imports
use std::{path::PathBuf, ffi::OsStr};
use std::fs;
use std::os::unix::fs::MetadataExt;

// external crate imports
use walkdir::WalkDir;

// internal crate imports
use crate::database::DatabaseWrapper;

pub struct Backend {
    database_wrapper: DatabaseWrapper,
    notes_dir: PathBuf,
    notes_extension: String,
}

impl Backend {
    /// Gets a list of tuples containing the markdown file names in notes_dir and their corresponding mtime
    /// Is used to check if the database cache is up to date.
    fn get_m_times(&self) -> Result<Vec<(PathBuf,i64)>, walkdir::Error>{
        Ok(WalkDir::new(&self.notes_dir).into_iter().filter_entry(|entry| {
            let path = entry.path();
            path.is_dir() || path.extension().and_then(OsStr::to_str) == Some(&self.notes_extension)
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
pub enum BackendError {
    BackendNotesError {
        message: &'static str
    },
    BackendDatabaseError {
        message: &'static str
    }
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            BackendError::BackendNotesError { message } => message,
            BackendError::BackendDatabaseError { message } => message,
        };
        write!(f, "BackendError: {}", message)
    }
}

#[cfg(test)]
mod backend_test {

}
