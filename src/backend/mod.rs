mod builder;
// std imports
use std::path::PathBuf;

// internal crate imports
use crate::database::DatabaseWrapper;

pub struct Backend {
    database_wrapper: DatabaseWrapper,
    notes_dir: PathBuf,
}


#[cfg(test)]
mod BackendTest {

}
