use crate::core::classpath::classpath_entry::ClasspathEntry;

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

    }

    fn parse_user_classpath(user_classpath: Option<String>) -> ClasspathEntry {
        Cla
    }
}