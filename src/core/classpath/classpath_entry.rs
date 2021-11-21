use std::fs::{File, read_dir};
use std::io::{Error, ErrorKind, Read};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum ClasspathEntry {
    // 文件夹路径
    Dir { path_buf: PathBuf },
    // jar包路径
    Zip { path_buf: PathBuf },
    // 对应通配符路径下所有jar包的path_buf的集合
    Wildcard { path_buf_vec: Vec<PathBuf> },
}

impl ClasspathEntry {
    pub fn new(path: &str) -> ClasspathEntry {
        if path.ends_with("*") {
            let base_path = &path[..(path.len() - 1)];

            let path_buf_vec = read_dir(base_path)
                .unwrap()
                .map(|entry_result| entry_result.unwrap())
                .map(|entry| entry.path())
                .filter(|path_buf| {
                    path_buf
                        .extension()
                        // 只保留后缀为jar的文件
                        .map(|ext_str_option| ext_str_option.to_str().unwrap() == "jar")
                        .unwrap_or(false)
                })
                .collect();
            ClasspathEntry::Wildcard {
                path_buf_vec
            }
        } else if path.ends_with(".jar") {
            ClasspathEntry::Zip {
                path_buf: Path::new(path).to_owned()
            }
        } else {
            ClasspathEntry::Dir {
                path_buf: Path::new(path).to_owned()
            }
        }
    }

    pub fn read_class(&self, class_file_name: &str) -> Result<Vec<u8>, std::io::Error> {
        match self {
            ClasspathEntry::Dir { path_buf } => {
                let file_path = Path::new(path_buf).join(class_file_name);
                let mut file = File::open(file_path)?;
                let file_meta_data = file.metadata()?;
                let mut file_bytes_buf = Vec::<u8>::with_capacity(file_meta_data.len() as usize);
                file.read_to_end(&mut file_bytes_buf)?;
                Ok(file_bytes_buf)
            }
            ClasspathEntry::Zip { path_buf } => {
                let zip_file = File::open(path_buf)?;
                let mut zip_file = zip::ZipArchive::new(zip_file)?;
                let mut class_file = zip_file.by_name(class_file_name)?;
                let mut file_bytes_buf = Vec::<u8>::with_capacity(class_file.size() as usize);
                class_file.read_to_end(&mut file_bytes_buf)?;
                Ok(file_bytes_buf)
            }
            ClasspathEntry::Wildcard { path_buf_vec } => {
                path_buf_vec
                    .iter()
                    // 得到ClasspathEntry::Zip   即为jar包的ClasspathEntry
                    .map(|path_buf| ClasspathEntry::new(path_buf.to_str().unwrap()))
                    .map(|classpath_entry_zip| classpath_entry_zip.read_class(class_file_name))
                    .find(|result| result.is_ok())
                    .unwrap_or(Err(Error::new(ErrorKind::Other, "Class not found")))
            }
        }
    }
}