//! This module contains the wrapper struct for the sqlite database
// stdlib imports
use std::path::{ Path, PathBuf};

// external crate imports
use diesel::prelude::*;
use diesel::result::ConnectionError;
use diesel_migrations::RunMigrationsError;
embed_migrations!(); // here we inject a module 'embedded_migrations' which has all the migrations we want.
use dirs;

/// Database struct which defines wanted interaction with the sqlite database
pub struct DatabaseWrapper {
    connection: SqliteConnection,
}

impl DatabaseWrapper {

    /// creates new sqlite wrapper from path provided by lua frontend
    pub fn new(database_url: &Path) -> Result<Self, ConnectionError> {
        let connection = SqliteConnection::establish(database_url.to_str().unwrap())?;
        Ok(Self {
            connection
        })
    }
    
    /// tries to give the database provided to `new(database_url)` a valid schema
    pub fn init_schema(&self) -> Result<(), RunMigrationsError> {
        // run migrations to make sure that the schema is valid
        embedded_migrations::run(&self.connection)?;
        Ok(())
    }
}



#[cfg(test)]
mod DatabaseTests {
    use super::*;

    /// removes the database after the test such that we can manually test for repeated access
    /// (without this, every test will be a repeated access.).
    fn clean_up_database(path: &Path) {
        std::fs::remove_file(path).expect(&format!("could not clean up file {} after test!", path.to_str().unwrap()));
    }

    /// creates a path for each database used in a test
    fn temp_dir_path(database_name: &str) -> PathBuf {
        // os dependent path to tmp directory
        let mut test_database_path: PathBuf = std::env::temp_dir();
        test_database_path.push(format!("{database_name}.db"));
        test_database_path
    }

    #[test]
    /// Tests database handle. This does not test the database struct but rather the enstablish
    /// function from the diesel library.
    fn test_create_database_connection() {
        // test_database_path
        let tdp = temp_dir_path("test_create_database_connection");
        // turn pathbuf into expected &str
        let path_os_string = tdp.clone().into_os_string();
        let path_string = path_os_string.to_str().unwrap(); // we unwrap since we will never handle paths that are not valid utf8
        let connection_result = SqliteConnection::establish(path_string);
        // here the connection must have succeeded.
        assert!(connection_result.is_ok());
        clean_up_database(&tdp);
    }

    #[test]
    /// Tests the sqlite wrapper creation. This should always work. Will run migrations
    fn test_create_database() {
        let tdb: PathBuf = temp_dir_path("test_create_database");
        assert!(DatabaseWrapper::new(&tdb).is_ok());
        clean_up_database(&tdb);
    }

    #[test]
    /// Test repeated access to the database.
    fn test_create_database_twice() {
        let tdb = temp_dir_path("test_create_database_twice");
        {
            // here we create a database instance
            let database = DatabaseWrapper::new(&tdb);
            // this should work
            assert!(database.is_ok());
        } // which is dropped here
        {
            // here we create a database instance again
            let database = DatabaseWrapper::new(&tdb);
            assert!(database.is_ok());
        } // which is dropped here
        clean_up_database(&tdb);
    }


    #[test]
    /// Test wheter we can create a database .cache. Might highlight some permission errors
    fn test_create_database_in_cache() {
        let cache_path = dirs::cache_dir().unwrap().join("rs-core").join("test_create_database_in_cache");
        assert!(DatabaseWrapper::new(&cache_path).is_ok());
        clean_up_database(&cache_path);
    }
}
