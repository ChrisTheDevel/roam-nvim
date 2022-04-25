//! This module contains the wrapper struct for the sqlite database
pub mod builder;

use std::path::PathBuf;

// external crate imports
use diesel::prelude::*;
use diesel_migrations::RunMigrationsError;
embed_migrations!(); // here we inject a module 'embedded_migrations' which has all the migrations we want.

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
    use super::*;
    use crate::database::builder::DatabaseWrapperBuilder;
    use std::path::Path;

    /// function to delete old tb. Otherwise every test might run on an already existing db.
    fn delete_db(path: &Path) {
        std::fs::remove_dir_all(path).expect(&format!("could not remove {:?}", path));
    }

    #[test]
    /// we want to test that the migrations are done correctly
    fn test_migrations() {
        let test_temp_dir = std::env::temp_dir().join("test_migrations");
        let db_wrapper: DatabaseWrapper = DatabaseWrapperBuilder::new()
            .cache_path(test_temp_dir.clone())
            .database_name("test_migrations.db".into())
            .build()
            .expect("could not create db wrapper");
        assert!(db_wrapper.init_schema().is_ok());
        delete_db(&test_temp_dir)
    }

    #[test]
    /// we want to test that the migrations are done correctly when run multiple times
    fn test_multiple_migrations() {
        let test_temp_dir = std::env::temp_dir().join("test_multiple_migrations");

        let builder = DatabaseWrapperBuilder::new()
            .cache_path(test_temp_dir.clone())
            .database_name("test_multiple_migrations.db".into());
        {
            let db_wrapper: DatabaseWrapper = builder.build().expect("could not create db wrapper");
            assert!(db_wrapper.init_schema().is_ok());
            assert!(db_wrapper.init_schema().is_ok());
        }
        // the migrations should be ok to
        let db_wrapper: DatabaseWrapper = builder.build().expect("could not create db wrapper");
        assert!(db_wrapper.init_schema().is_ok());
        assert!(db_wrapper.init_schema().is_ok());
        delete_db(&test_temp_dir)
    }
}
