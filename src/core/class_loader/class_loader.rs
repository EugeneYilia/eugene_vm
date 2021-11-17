use std::collections::HashMap;
use std::rc::Rc;
use crate::core::class_loader::class_reader::ClassReader;
use crate::core::classpath::classpath::ClassPath;
use crate::runtime::method_area::class::class::Class;

pub struct ClassLoader {
    classpath: ClassPath,
    class_map: HashMap<String, Rc<Class>>,
}

impl ClassLoader {
    pub fn new(classpath: ClassPath) -> ClassLoader {
        ClassLoader {
            classpath,
            class_map: HashMap::new(),
        }
    }

    pub fn load(&mut self, name: String) -> Rc<Class> {

    }

    pub fn define(class_loader: ClassLoader, bytes_code: Vec<u8>) -> Rc<Class> {
        let class_file = bytes_code.parse();

    }
}