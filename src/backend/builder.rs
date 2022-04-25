use std::path::PathBuf;

use super::Backend;
use crate::database::DatabaseWrapper;

pub struct BackendBuilder {
    database_wrapper: Option<DatabaseWrapper>,
    notes_dir: Option<PathBuf>,
    notes_extension: Option<String>,
}

impl BackendBuilder {
    /// Creates builder
    pub fn new() -> Self {
        BackendBuilder {
            database_wrapper: None,
            notes_dir: None,
            notes_extension: Some("md".into()), // default extension
        }
    }

    pub fn database_wrapper(mut self, database_wrapper: DatabaseWrapper) -> Self {
        self.database_wrapper = Some(database_wrapper);
        self
    }

    pub fn notes_dir(mut self, notes_dir: PathBuf) -> Self {
        self.notes_dir = Some(notes_dir);
        self
    }

    /// This method is optional since a default value is provided by `new`
    pub fn notes_extension(mut self, notes_extension: String) -> Self {
        self.notes_extension = Some(notes_extension);
        self
    }


    pub fn build(self) -> Result<Backend, BackendBuilderError> {
        if !(self.notes_dir.is_some() && self.database_wrapper.is_some()) {
            return Err(BackendBuilderError {
                message: "Could not build backend since not all fields were intialzided".into(),
            });
        }
        // this is safe to unwrap since we've already checked if it is Some.
        let notes_dir = self.notes_dir.unwrap();
        if !notes_dir.exists() {
            std::fs::create_dir_all(&notes_dir).map_err(|_| BackendBuilderError {
                message: String::from("Could not create the notes dirs!"),
            })?;
        }
        Ok(Backend {
            database_wrapper: self.database_wrapper.unwrap(),
            notes_dir,
            notes_extension: self.notes_extension.unwrap(),

        })
    }
}

#[derive(Clone, Debug)]
/// An error type for the DatabaseWrapperBuilder
pub struct BackendBuilderError {
    message: String,
}

impl std::fmt::Display for BackendBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BackendBuilderError: {}", self.message)
    }
}

#[cfg(test)]
mod BackendBuilderTests {
    use std::path::Path;

    use super::*;
    use crate::database::builder::{DatabaseBuilderError, DatabaseWrapperBuilder};

    /// util function to create a database wrapper (which should already work)
    fn create_wrapper() -> DatabaseWrapper {
        let temp_cache_path = std::env::temp_dir().join("backend_tests");
        DatabaseWrapperBuilder::new()
            .cache_path(temp_cache_path)
            .database_name("backend_test.db".into())
            .build()
            .expect("Could not create database wrapper")
    }

    fn remove_wrapper() {
        let temp_cache_path = std::env::temp_dir().join("backend_tests");
        std::fs::remove_dir_all(temp_cache_path).expect("could not remove database folder!");
    }

    fn create_notes_dir_path(dir_name: &str) -> PathBuf {
        std::env::temp_dir().join(dir_name)
    }

    fn remove_notes_dir(notes_dir_path: &Path) {
        std::fs::remove_dir_all(notes_dir_path).expect("could not remove database folder!");
    }

    #[test]
    /// tries to create backend
    fn create_backend() {
        let backend_builder = BackendBuilder::new();
        let wrapper = create_wrapper();
        let notes_dir = create_notes_dir_path("create_backend");

        let backend = backend_builder
            .notes_dir(notes_dir.clone())
            .database_wrapper(wrapper)
            .build();
        assert!(backend.is_ok());
        remove_wrapper();
        remove_notes_dir(&notes_dir);
    }

    #[test]
    fn create_backend_repeated() {
        let notes_dir = create_notes_dir_path("create_backend_repeated");
        {
            let wrapper = create_wrapper();
            let backend_builder = BackendBuilder::new();
            let backend = backend_builder
                .notes_dir(notes_dir.clone())
                .database_wrapper(wrapper)
                .build();
            assert!(backend.is_ok());
        }
        let wrapper = create_wrapper();
        let backend_builder = BackendBuilder::new();
        let backend = backend_builder
            .notes_dir(notes_dir.clone())
            .database_wrapper(wrapper)
            .build();
        assert!(backend.is_ok());
        remove_wrapper();
        remove_notes_dir(&notes_dir);
    }
}
