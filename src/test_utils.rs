use std::path::Path;

pub fn remove_dirs(path: &Path) {
    std::fs::remove_dir_all(path).expect(&format!("Could not delete dir at location: {:?}", path));
}
