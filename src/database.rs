// stdlib imports
use std::path::PathBuf;
// external crate imports
use diesel::prelude::*;
embed_migrations!(); // here we inject a module 'embedded_migrations' which has all the migrations we want.
                     // internal crate imports
use crate::error::BackendError;

/// Struct which handles connection to sqlite database. Exposes methods which can be used to
/// query/update the cache
pub struct Database {
    connection: SqliteConnection,
}

impl Database {
    // return all mtimes
    pub fn get_mtimes() {
        todo!()
    }

    pub fn get_hash() {
        todo!()
    }
}

struct DatabaseBuilder {
    database_location: Option<PathBuf>,
    database_name: Option<String>,
}

impl DatabaseBuilder {
    pub fn new() -> Self {
        Self {
            database_location: None,
            database_name: Some("CORE.db".into()),
        }
    }

    pub fn database_location(mut self, path: PathBuf) -> Self {
        self.database_location = Some(path);
        self
    }

    pub fn database_name(mut self, name: String) -> Self {
        self.database_name = Some(name);
        self
    }
    pub fn build(self) -> Result<Database, BackendError> {
        if self.database_location.is_none() {
            return Err(BackendError::DatabaseBuilderError {
                message: "All fields were not initialized".into(),
            });
        }
        // it is safe to unwrap
        let location = self.database_location.unwrap();
        let name = self.database_name.unwrap();
        let database_path = location.join(name);
        let database_path_str =
            database_path
                .to_str()
                .ok_or_else(|| BackendError::DatabaseBuilderError {
                    message: "Could not convert path to valid utf8 str!".into(),
                })?;
        let connection = SqliteConnection::establish(database_path_str).map_err(|_| {
            BackendError::DatabaseBuilderError {
                message: "Could not establish connection!".into(),
            }
        })?;
        embedded_migrations::run(&connection).map_err(|_| BackendError::DatabaseBuilderError {
            message: "Builder could not run migrations properly".into(),
        })?;
        Ok(Database { connection })
    }
}

#[cfg(test)]
mod database_tests {
    use super::*;
    use crate::test_utils::*;

    #[test]
    fn create_database_test() {
        let db_name = "create_database_test";
        let (path, remove) = db_temp_dir(db_name);
        let builder = DatabaseBuilder::new();
        let database = builder
            .database_name(format!("{db_name}.db"))
            .database_location(path)
            .build();
        assert!(database.is_ok());
        assert!(remove().is_ok());
    }

    #[test]
    fn create_database_twice_test() {
        let db_name = "create_database_twice_test";
        let (path, remove) = db_temp_dir(db_name);
        {
            let builder = DatabaseBuilder::new();
            let database = builder
                .database_location(path.clone())
                .database_name(format!("{db_name}.db"))
                .build();
            assert!(database.is_ok());
        }
        let builder = DatabaseBuilder::new();
        let database = builder
            .database_location(path.clone())
            .database_name(db_name.into())
            .build();
        assert!(database.is_ok());
        assert!(remove().is_ok());
    }

    #[test]
    fn create_database_incomplete_test() {
        let db_name = "create_database_twice_test";
        let (_path, remove) = db_temp_dir(db_name);
        let builder = DatabaseBuilder::new();
        let database = builder
            .database_name(db_name.into())
            .build();
        assert!(database.is_err());
        assert!(remove().is_ok());

    }
}
