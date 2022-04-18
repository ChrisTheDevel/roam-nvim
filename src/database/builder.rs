// stdlib imports
use std::path::PathBuf;
// external crate imports
use diesel::prelude::*;
// interal crate imports
use super::DatabaseWrapper;

/// A builder struct for DatabaseWrapper
pub struct DatabaseWrapperBuilder {
    cache_path: Option<PathBuf>,
    database_name: Option<String>,
}

impl DatabaseWrapperBuilder {
    /// Create unitialized builder struct. All builder methods must be used to create a valid
    /// DatabaseWrapper
    pub fn new() -> Self {
        Self {
            cache_path: None,
            database_name: None,
        }
    }

    /// Provides the cache_path (some folder in .cache) to the builder
    pub fn cache_path(mut self, path: PathBuf) -> Self {
        self.cache_path = Some(path);
        self
    }

    /// Provides the name of database to the builder. Name should contain extension
    pub fn database_name(mut self, name: String) -> Self {
        self.database_name = Some(name);
        self
    }

    /// Takes the provided paths, creates them if they do not exist. Is nonconsuming
    pub fn build(&self) -> Result<DatabaseWrapper, DatabaseBuilderError> {
        // we can only create the wrapper when all fields have been initialized;
        if !(self.cache_path.is_some() && self.database_name.is_some()) {
            return Err(DatabaseBuilderError {
                message: String::from(
                    "All fields of builder has not been initialized. Cannot build wrapper",
                ),
            });
        }
        // it is safe to take the cache_path
        let cache_path = self.cache_path.clone().unwrap();
        let database_name = self.database_name.clone().unwrap();
        // create the dirs if necessary. The establish function from diesel does not do this.
        if !cache_path.exists() {
            std::fs::create_dir_all(&cache_path).map_err(|_| DatabaseBuilderError {
                message: String::from("Could not create the cache dirs!"),
            })?;
        }

        let database_path = cache_path.join(database_name);
        let database_url = database_path.to_str().ok_or_else(|| DatabaseBuilderError {
            message: String::from("Could not convert path to valid utf8"),
        })?;

        let connection: SqliteConnection =
            SqliteConnection::establish(database_url).map_err(|_| DatabaseBuilderError {
                message: String::from("Could not establish connection to sqlite database"),
            })?;
        Ok(DatabaseWrapper { connection })
    }
}

#[derive(Clone, Debug)]
/// An error type for the DatabaseWrapperBuilder
pub struct DatabaseBuilderError {
    message: String,
}

impl std::fmt::Display for DatabaseBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DatabaseBuilderError: {}", self.message)
    }
}

#[cfg(test)]
mod DatabaseBuilderTests {
    use super::*;
    use std::path::Path;

    /// function to delete old tb. Otherwise every test might run on an already existing db.
    fn delete_db(path: &Path) {
        std::fs::remove_dir_all(path).expect(&format!("could not remove {:?}", path));
    }

    #[test]
    /// Test creating db on path that does not exist yet
    fn test_builder_without_existing_path() {
        let test_temp_dir = std::env::temp_dir().join("test_builder_without_existing_path");

        // create a builder struct and init is using methods
        let builder = DatabaseWrapperBuilder::new();
        let database_wrapper = builder
            .cache_path(test_temp_dir.clone())
            .database_name("test_builder_without_existing_path.db".into())
            .build();
        assert!(database_wrapper.is_ok());
        delete_db(&test_temp_dir);
    }

    #[test]
    /// we test trying to create a wrapper without passing all the fields to the builder (should
    /// fail)
    fn test_builder_uninitialized() {
        let builder = DatabaseWrapperBuilder::new();
        let wrapper = builder.build();
        assert!(wrapper.is_err());
    }

    #[test]
    /// we want to make sure that we can also create a wrapper the cache_path already exists.
    fn test_builder_with_existing_path() {
        let test_temp_dir = std::env::temp_dir().join("test_builder_with_existing_path");
        // we create the dirs ahead of time. This should work since we're only using std functions.
        assert!(std::fs::create_dir_all(test_temp_dir.clone()).is_ok());
        // now we create the builder
        let builder = DatabaseWrapperBuilder::new()
            .cache_path(test_temp_dir.clone())
            .database_name("test_builder_with_existing_path.db".into());
        // and then we use it multiple times.
        {
            let database_wrapper = builder.build();
            // the builder should have successfully created a wrapper
            assert!(database_wrapper.is_ok());
        } // here we drop the wrapper and therefore the connection

        // it is now safe to create a new connection
        let database_wrapper = builder.build();
        // the builder should have successfully created a wrapper despite the previous
        // connection/creation of dirs
        assert!(database_wrapper.is_ok());
        delete_db(&test_temp_dir);
    }
}
