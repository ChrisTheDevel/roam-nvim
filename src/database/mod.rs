//! This module contains the wrapper struct for the sqlite database
mod builder;

// external crate imports
use diesel::prelude::*;
use diesel_migrations::RunMigrationsError;
embed_migrations!(); // here we inject a module 'embedded_migrations' which has all the migrations we want.

/// Database struct which defines wanted interaction with the sqlite database
pub struct DatabaseWrapper {
    connection: SqliteConnection,
}

impl DatabaseWrapper {
    /// tries to give the database provided to `new(database_url)` a valid schema
    pub fn init_schema(&self) -> Result<(), RunMigrationsError> {
        // run migrations to make sure that the schema is valid
        embedded_migrations::run(&self.connection)?;
        Ok(())
    }
}

