use std::collections::HashMap;
use std::rc::Rc;
use crate::core::class_loader::class_reader::ClassReader;
use crate::core::classfile::classfile::ClassFile;
use crate::core::classpath::classpath::ClassPath;
use crate::runtime::method_area::class::class::Class;

// TODO: 加入双亲委派机制
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

    pub fn load_class(&mut self, class_name: String) -> Rc<Class> {
        if self.class_map.contains_key(&class_name) {
            Rc::clone(self.class_map.get(&class_name).unwrap())
        } else {
            let byte_code = self.read_class(&class_name);
            let (class_loader, class_ref) = ClassLoader::define_class(self, byte_code);
            class_loader.class_map.insert(class_name, Rc::clone(&class_ref));
            class_ref
        }
    }

    fn read_class(&self, class_name: &str) -> Vec<u8> {
        self.classpath
            .read_class(class_name)
            .expect(format!("Class not found: {}", class_name).as_str())
    }

    fn define_class(class_loader: &mut ClassLoader, bytes_code: Vec<u8>) -> (&mut ClassLoader, Rc<Class>) {
        let class_file = bytes_code.parse();
        let ClassFile {
            constant_pool,
            access_flags,
            fields,
            methods,
            ..
        } = class_file;

    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn test_string_to_string_hashmap() {
        let mut hashmap = HashMap::<String, String>::new();
        hashmap.insert("a".to_string(), "b".to_string());
        let result = hashmap.get(&"a".to_string());
        println!("{:?}", result);
    }
}