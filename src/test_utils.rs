use std::fs::File;
use std::io::Write;
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

pub fn create_temp_file(name: &str) -> (PathBuf, impl FnOnce() -> Result<(), std::io::Error>) {
    let name_path = std::env::temp_dir().join(format!("{name}_directory"));
    std::fs::create_dir_all(&name_path).expect("could not create dirs for test");
    let path_clone = name_path.clone();
    name_path.join(name);
    let mut file =
        std::fs::File::create(name_path.clone()).expect("could not create file for test");
    let destructor = move || std::fs::remove_dir_all(path_clone);
    (name_path, destructor)
}
