use std::collections::HashMap;
use crate::core::class::class::Class;
use crate::core::classpath::classpath::ClassPath;

pub struct ClassLoader {
    classpath: ClassPath,
    class_map: HashMap<String, Class>,
}

impl ClassLoader {
    pub fn new(classpath: ClassPath) -> ClassLoader {
        ClassLoader {
            classpath,
            class_map:HashMap::new()
        }
    }
}