use crate::core::classpath::classpath_entry::ClasspathEntry;
use crate::util::file_util::is_path_exist;

use std::path::{PathBuf, Path};


#[derive(Debug)]
pub struct ClassPath {
    boot_classpath: ClasspathEntry,
    user_classpath: ClasspathEntry,
}

impl ClassPath {
    pub fn parse_classpath(boot_classpath: Option<String>, user_classpath: Option<String>) -> ClassPath {
        ClassPath {
            boot_classpath: ClassPath::parse_boot_classpath(boot_classpath),
            user_classpath: ClassPath::parse_user_classpath(user_classpath),
        }
    }

    fn parse_boot_classpath(boot_classpath: Option<String>) -> ClasspathEntry {
        fn build_classpath_entry(boot_classpath: &str) -> ClasspathEntry {
            ClasspathEntry::new(
                Path::new(boot_classpath)
                    .join("lib")
                    .join("*")
                    .to_str()
                    .unwrap()
            )
        }

        match boot_classpath {
            Some(ref path) if is_path_exist(&path) => {
                build_classpath_entry(path.as_str())
            }
            // None   and  (Some if false)
            _ => {
                if is_path_exist("jre") {
                    build_classpath_entry("jre")
                } else {
                    match std::env::var_os("JAVA_HOME") {
                        Some(java_home) => {
                            build_classpath_entry(
                                Path::new(&java_home)
                                    .join("jre")
                                    .to_str()
                                    .unwrap()
                            )
                        }
                        None => {
                            panic!("Can't find JRE directory.")
                        }
                    }
                }
            }
        }
    }

    fn parse_user_classpath(user_classpath: Option<String>) -> ClasspathEntry {
        let transformed_user_classpath = user_classpath.unwrap_or(".".to_owned());
        ClasspathEntry::new(&transformed_user_classpath)
    }

    pub fn read_class(&self, class_name: &str) -> Result<Vec<u8>, std::io::Error> {
        let class_file_name = class_name.to_owned() + ".class";
        self.boot_classpath.read_class(&class_file_name)
            .or_else(|_|self.user_classpath.read_class(&class_file_name))
    }
}