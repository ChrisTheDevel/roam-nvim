use std::fs::File;
use std::path::PathBuf;

/// Creates a directory at location and returns the full path of the directory + function to
/// destory folder after use
pub fn db_temp_dir(db_name: &str) -> (PathBuf, impl FnOnce() -> Result<(), std::io::Error>) {
    let path = std::env::temp_dir().join(db_name);
    let path_clone = path.clone();
    std::fs::create_dir_all(path.clone()).expect("could not create temp dir for test");
    let closure = move || std::fs::remove_dir_all(path_clone);
    (path, closure)
}

// will create a temporary file inside the temp folder and then return a function to remove that
// file
pub fn temp_file(name: String) -> (PathBuf, File, impl FnOnce() -> Result<(), std::io::Error>) {
    let path = std::env::temp_dir().join(name);
    let file = std::fs::File::create(path.clone()).expect("could not create file in test");
    let path_clone = path.clone();
    let closure = move || std::fs::remove_file(path_clone);
    (path, file, closure)
}
