use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::num::Wrapping;
use std::ops::Deref;
use std::rc::Rc;

use crate::constants::access_flags::ACCESS_STATIC;
use crate::constants::class_constants::ROOT_CLASS_NAME;
use crate::constants::descriptor::*;
use crate::core::class_loader::class_reader::ClassReader;
use crate::core::classfile::classfile::ClassFile;
use crate::core::classpath::classpath::ClassPath;
use crate::runtime::heap::object_field::ObjectField;
use crate::runtime::method_area::class::class::Class;
use crate::runtime::method_area::class::field::Field;
use crate::runtime::method_area::class::method::Method;
use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
use crate::runtime::method_area::constant_pool::constant_pool::ConstantPool;
use crate::runtime::stack::variable_slot::VariableSlot;
use crate::runtime::stack::variables_table::VariableTable;
use crate::runtime::thread::Thread;
use crate::util::converter;
use crate::util::instruction_util::get_default_variable_slot;

// TODO: 加入双亲委派机制
#[derive(Debug)]
pub struct ClassLoader {
    classpath: ClassPath,
    class_map: HashMap<String, Rc<Class>>,
    thread: Rc<RefCell<Thread>>,
}

impl ClassLoader {
    pub fn new(classpath: ClassPath, thread: Rc<RefCell<Thread>>) -> ClassLoader {
        ClassLoader {
            classpath,
            class_map: HashMap::new(),
            thread,
        }
    }

    pub fn load_class(class_loader: Rc<RefCell<ClassLoader>>, class_name: String, mut thread: &mut RefMut<Thread>) -> Rc<Class> {
        if class_loader.deref().borrow().class_map.contains_key(&class_name) {
            Rc::clone(class_loader.deref().borrow().class_map.get(&class_name).unwrap())
        } else {
            let byte_code = class_loader.deref().borrow().read_class(&class_name);
            let (class_loader, class_ref) = ClassLoader::define_class(class_loader, byte_code, &mut thread);
            class_loader.deref().borrow_mut().class_map.insert(class_name, Rc::clone(&class_ref));
            // load完之后执行clinit方法
            if let Some(method_ref) = class_ref.get_method("<clinit>", Some("()V"), Some(vec![ACCESS_STATIC])) {
                Thread::invoke_method(Rc::clone(&class_ref), method_ref, &mut thread, None);
            }
            class_ref
        }
    }

    fn read_class(&self, class_name: &str) -> Vec<u8> {
        self.classpath
            .read_class(class_name)
            .expect(format!("Class not found: {}", class_name).as_str())
    }

    fn define_class(class_loader: Rc<RefCell<ClassLoader>>, bytes_code: Vec<u8>, mut thread: &mut RefMut<Thread>) -> (Rc<RefCell<ClassLoader>>, Rc<Class>) {
        let class_file = bytes_code.parse();

        let class_name = class_file.get_class_name().to_owned();
        let super_class = if class_name != ROOT_CLASS_NAME {
            let super_class_name = class_file.get_super_class_name();
            Some(ClassLoader::load_class(Rc::clone(&class_loader), super_class_name.to_owned(), &mut thread))
        } else {
            None
        };

        let ClassFile {
            constant_pool,
            access_flags,
            fields,
            methods,
            ..
        } = class_file;

        let fields: Vec<Field> = fields
            .iter()
            .map(|member_info| Field::new(member_info))
            .collect();

        let methods: Vec<Rc<Method>> = methods
            .iter()
            .map(|member_info| Rc::new(Method::new(member_info)))
            .collect();


        let mut static_variable_table = HashMap::<String, ObjectField>::new();
        // 构建class的静态字段表
        fields.iter().for_each(|field| {
            if field.get_access_flags() & ACCESS_STATIC != 0 {
                if field.constant_value_index.is_none() {
                    // 普通静态变量  需要在clinit中赋值  先构建初始值存储
                    static_variable_table.insert(field.get_name().to_owned(), ObjectField::new(field.get_class_member(), get_default_variable_slot(field.get_descriptor())));
                } else {
                    // 直接将初始化好的静态字段添加到static variable table中
                    let constant_value_index = field.constant_value_index.unwrap();
                    match field.get_descriptor() {
                        BYTE_DESCRIPTOR | CHAR_DESCRIPTOR | INT_DESCRIPTOR | SHORT_DESCRIPTOR | BOOLEAN_DESCRIPTOR => {
                            match constant_pool.get(constant_value_index) {
                                ConstantInfo::Integer(value) => {
                                    static_variable_table.insert(field.get_name().to_owned(), ObjectField::new(field.get_class_member(), VariableSlot::I32(Wrapping(*value))));
                                }
                                _ => panic!("constant_value_index: {} is not ConstantInfo::Integer", constant_value_index)
                            }
                        }
                        DOUBLE_DESCRIPTOR => {
                            match constant_pool.get(constant_value_index) {
                                ConstantInfo::Double(value) => {
                                    static_variable_table.insert(field.get_name().to_owned(), ObjectField::new(field.get_class_member(), VariableSlot::F64(*value)));
                                }
                                _ => panic!("constant_value_index: {} is not ConstantInfo::Double", constant_value_index)
                            }
                        }
                        FLOAT_DESCRIPTOR => {
                            match constant_pool.get(constant_value_index) {
                                ConstantInfo::Float(value) => {
                                    static_variable_table.insert(field.get_name().to_owned(), ObjectField::new(field.get_class_member(), VariableSlot::F32(*value)));
                                }
                                _ => panic!("constant_value_index: {} is not ConstantInfo::Float", constant_value_index)
                            }
                        }
                        LONG_DESCRIPTOR => {
                            match constant_pool.get(constant_value_index) {
                                ConstantInfo::Long(value) => {
                                    static_variable_table.insert(field.get_name().to_owned(), ObjectField::new(field.get_class_member(), VariableSlot::I64(Wrapping(*value))));
                                }
                                _ => panic!("constant_value_index: {} is not ConstantInfo::Long", constant_value_index)
                            }
                        }
                        OBJ_DESCRIPTOR => {
                            if let ConstantInfo::String(utf8_index) = constant_pool.get(constant_value_index) {
                                if let ConstantInfo::ModifiedUTF8(value) = constant_pool.get(*utf8_index as usize) {
                                    warn!("value: {}", value);
                                } else {
                                    panic!("constant_value_index: {}  utf8_index: {} is not ConstantInfo::ModifiedUTF8", constant_value_index, utf8_index);
                                }
                            } else {
                                panic!("constant_value_index: {} is not ConstantInfo::String", constant_value_index);
                            }
                        }
                        _ => {
                            panic!("invalid descriptor type: {} name: {}", field.get_descriptor(), field.get_name())
                        }
                    }
                }
            }
        });


        let class_ref = Rc::new(Class {
            access_flags,
            constant_pool,
            class_name,
            fields,
            methods,
            super_class,
            static_variable_table,
            class_loader: Some(Rc::clone(&class_loader)),
        });

        (class_loader, class_ref)
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

    #[test]
    fn test_fold() {
        let source = vec![1, 2, 3, 4];
        let result = source.iter().fold(0i32, |acc, value| { acc + value });
        println!("{}", result)
    }
}