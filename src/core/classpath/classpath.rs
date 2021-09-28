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
        let real_boot_classpath:&str;
        match boot_classpath {
            Some(path) if is_path_exist(&path)=>{
                real_boot_classpath = path.as_str();
            }
            // None   and  (Some if false)
            _ =>{
                if is_path_exist("jre") {
                    real_boot_classpath = "jre";
                } else {
                    match std::env::var_os("JAVA_HOME") {
                        Some(java_home)=>{
                            real_boot_classpath = Path::new(&java_home)
                                .join("jre")
                                .to_str()
                                .unwrap()
                        }
                        None=>{
                            panic!("Can't find JRE directory.")
                        }
                    }
                }
            }
        }

        let jre_lib_path = Path::new(real_boot_classpath)
            .join("lib")
            .join("*")
            .to_str()
            .unwrap();

        ClasspathEntry::new(jre_lib_path)
    }

    fn parse_user_classpath(user_classpath: Option<String>) -> ClasspathEntry {
        let transformed_user_classpath = user_classpath.unwrap_or(".".to_owned());
        ClasspathEntry::new(&transformed_user_classpath)
    }
}