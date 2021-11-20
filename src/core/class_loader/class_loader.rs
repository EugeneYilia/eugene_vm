use std::collections::HashMap;
use std::rc::Rc;
use crate::constants::class_constants::ROOT_CLASS_NAME;
use crate::core::class_loader::class_reader::ClassReader;
use crate::core::classfile::classfile::ClassFile;
use crate::core::classpath::classpath::ClassPath;
use crate::runtime::method_area::class::class::Class;
use crate::runtime::method_area::class::field::Field;
use crate::runtime::method_area::class::method::Method;
use crate::runtime::method_area::constant_pool::constant_pool::ConstantPool;
use crate::runtime::stack::variables_table::VariableTable;
use crate::constants::access_flags::{ACCESS_STATIC, ACCESS_FINAL};
use crate::constants::field_descriptor::*;
use crate::runtime::method_area::constant_pool::constant_info::ConstantInfo;
use crate::runtime::stack::variable_slot::VariableSlot;
use crate::util::converter;

///       下一个实例字段slot_id  下一个静态字段slot_id  static变量表  常量池
type SlotIdAccumulator = (usize, usize, VariableTable, ConstantPool);

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

        let class_name = class_file.get_class_name().to_owned();
        let super_class = if class_name != ROOT_CLASS_NAME {
            let super_class_name = class_file.get_super_class_name();
            Some(class_loader.load_class(super_class_name.to_owned()))
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


        fn calc_instance_slot_id(slot_id_accumulator: SlotIdAccumulator, field: &Field) -> SlotIdAccumulator {
            let (next_instance_slot_id, next_static_slot_id, mut static_variable_table, constant_pool) = slot_id_accumulator;
            let used_slot_amount = if field.is_need_two_slot() { 2usize } else { 1usize };
            if field.get_access_flags() & ACCESS_STATIC != 0 {
                if field.constant_value_index.is_none() {
                    // 此时应该是static final variable_name = primitive value  会被编译为常量 内联到使用的地方 不会更新static variable table 和 next_static_slot_id
                    (next_instance_slot_id, next_static_slot_id, static_variable_table, constant_pool)
                } else {
                    // 添加到static variable table中
                    let constant_value_index = field.constant_value_index.unwrap();
                    match field.get_descriptor() {
                        BYTE_FIELD_DESCRIPTOR | CHAR_FIELD_DESCRIPTOR | INT_FIELD_DESCRIPTOR | SHORT_FIELD_DESCRIPTOR | BOOLEAN_FIELD_DESCRIPTOR | OBJ_FIELD_DESCRIPTOR => {
                            match constant_pool.get(constant_value_index) {
                                ConstantInfo::Integer(value) => {
                                    static_variable_table.set_variable_slot(next_static_slot_id, VariableSlot::I32(*value))
                                }
                                _ => panic!("constant_value_index: {} is not ConstantInfo::Integer")
                            }
                        }
                        DOUBLE_FIELD_DESCRIPTOR => {
                            match constant_pool.get(constant_value_index) {
                                ConstantInfo::Double(value) => {
                                    let [first, second] = converter::f64_to_i32seq(*value);
                                    static_variable_table.set_variable_slot(next_static_slot_id, VariableSlot::I32(first));
                                    static_variable_table.set_variable_slot(next_static_slot_id + 1, VariableSlot::I32(second));
                                }
                                _ => panic!("constant_value_index: {} is not ConstantInfo::Double")
                            }
                        }
                        FLOAT_FIELD_DESCRIPTOR => {
                            match constant_pool.get(constant_value_index) {
                                ConstantInfo::Float(value) => {
                                    static_variable_table.set_variable_slot(next_static_slot_id, VariableSlot::I32(converter::f32_to_i32(*value)))
                                }
                                _ => panic!("constant_value_index: {} is not ConstantInfo::Float")
                            }
                        }
                        LONG_FIELD_DESCRIPTOR => {
                            match constant_pool.get(constant_value_index) {
                                ConstantInfo::Long(value) => {
                                    let [first, second] = converter::i64_to_i32seq(*value);
                                    static_variable_table.set_variable_slot(next_static_slot_id, VariableSlot::I32(first));
                                    static_variable_table.set_variable_slot(next_static_slot_id + 1, VariableSlot::I32(second));
                                }
                                _ => panic!("constant_value_index: {} is not ConstantInfo::Long")
                            }
                        }
                        _ => {
                            panic!("Invalid descriptor type: {}", field.get_descriptor())
                        }
                    }
                    (next_instance_slot_id, next_static_slot_id + used_slot_amount, static_variable_table, constant_pool)
                }
            } else {
                (next_instance_slot_id + used_slot_amount, next_static_slot_id, static_variable_table, constant_pool)
            }
        }

        let next_instance_slot_id = super_class
            .map(|class| class.next_instance_slot_id)
            .unwrap_or(0);

        let slot_id_accumulator: SlotIdAccumulator = (next_instance_slot_id, 0usize, VariableTable::new(), constant_pool);

        let (next_instance_slot_id, next_static_slot_id, static_variable_table, constant_pool) = fields.iter().fold(slot_id_accumulator, calc_instance_slot_id);

        let class_ref = Rc::new(Class {
            access_flags,
            constant_pool,
            class_name,
            fields,
            methods,
            super_class,
            next_instance_slot_id,
            next_static_slot_id,
            static_variable_table
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