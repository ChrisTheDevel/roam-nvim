
#[derive(Debug)]
pub enum BackendError {
    /// Errors occuring when the plugin is initalized
    BackendInitError{message: String},
    /// Errors occuring when handling the sqlite database. Most likely faulty queries
    DatabaseError{message: String},
    /// Errors occuring when building the database struct. Unitialized fields or faulty paths
    DatabaseBuilderError{message: String},
    /// Errors occuring when building the NodesDir
    NodesDirBuilderError{message: String},
    /// Errors occuring when interacting with the nodes dir struct
    NodesDirError{message : String},
    /// Errors occuring when parsing a specific node
    ParseError
}

