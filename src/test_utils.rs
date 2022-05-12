use std::path::PathBuf;

/// Creates a directory at location and returns the full path of the directory + function to
/// destory if after use
pub fn db_temp_dir(db_name: &str) -> (PathBuf, impl FnOnce() -> Result<(), std::io::Error>) {
    let path = std::env::temp_dir().join(db_name);
    let path_clone = path.clone();
    std::fs::create_dir_all(path.clone());
    let closure = move || std::fs::remove_dir_all(path_clone);
    (path, closure)
}
