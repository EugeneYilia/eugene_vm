use std::path::Path;

fn is_file_exist(file_path: &str) -> bool {
    Path::new(file_path).exists()
}
