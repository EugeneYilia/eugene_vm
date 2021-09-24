use std::path::{PathBuf, Path};
use std::collections::hash_map::Entry;
use std::fs::read_dir;

#[derive(Debug)]
pub enum ClasspathEntry {
    Dir { path_buf: PathBuf },
    Zip { path_buf: PathBuf },
    Wildcard { path_buf_vec: Vec<PathBuf> },
}

impl ClasspathEntry {
    fn new(path: &str) -> ClasspathEntry {
        if path.ends_with("*") {
            let base_path = &path[..(path.len() - 1)];

            let path_buf_vec = read_dir(base_path)
                .unwrap()
                .map(|entry_result| entry_result.unwrap())
                .map(|entry| entry.path())
                .filter(|path_buf| {
                    path_buf
                        .extension()
                        .map(|ext_str_option| ext_str_option.to_str().unwrap())
                })
            ClasspathEntry::Wildcard {
                path_buf_vec:vec![]
            }
        } else if path.ends_with(".jar") {
            ClasspathEntry::Zip {
                path_buf:Path::new(path).to_owned()
            }
        } else {
            ClasspathEntry::Dir {
                path_buf:Path::new(path).to_owned()
            }
        }
    }
}