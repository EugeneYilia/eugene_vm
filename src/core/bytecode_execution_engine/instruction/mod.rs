pub mod nop;
pub mod load;
pub mod store;
pub mod control;
pub mod comparison;
pub mod math;
pub mod method;
pub mod stack_management;
pub mod object;
pub mod instruction_execute_result;

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::rc::Rc;

    use crate::bootstrap::bootstrap_option::BootstrapOption;
    use crate::core::class_loader::class_loader::ClassLoader;
    use crate::core::classfile::member_info::MemberInfo;
    use crate::core::classpath::classpath::ClassPath;
    use crate::runtime::method_area::class::class::Class;
    use crate::runtime::method_area::class::method::Method;
    use crate::runtime::method_area::constant_pool::constant_pool::ConstantPool;
    use crate::runtime::stack::stack_frame::StackFrame;
    use crate::runtime::stack::variables_table::VariableTable;
    use crate::runtime::thread::Thread;

    #[allow(dead_code)]
    fn mock_classpath() -> ClassPath {
        let user_classpath = Some("eugene_test/src_code/mine".to_owned());
        let boot_classpath = Some("eugene_test/src_code/eugene_rt".to_owned());
        let bootstrap_option = BootstrapOption::new("", user_classpath, boot_classpath, vec![]);
        ClassPath::parse_classpath(bootstrap_option.boot_classpath_option, bootstrap_option.user_classpath_option)
    }

    #[allow(dead_code)]
    pub fn mock_rc_method() -> Rc<Method> {
        Rc::new(Method::new(&MemberInfo {
            access_flags: 0u16,
            name: "".to_string(),
            name_index: 0u16,
            descriptor_index: 0u16,
            descriptor: "".to_string(),
            attributes: Vec::new(),
        }))
    }

    #[allow(dead_code)]
    pub fn mock_rc_class() -> Rc<Class> {
        Rc::new(Class {
            access_flags: 0u16,
            class_name: "".to_string(),
            constant_pool: ConstantPool {
                constant_info_map: BTreeMap::new()
            },
            fields: Vec::new(),
            methods: Vec::new(),
            super_class: None,
            next_instance_slot_id: 0usize,
            next_static_slot_id: 0usize,
            static_variable_table: VariableTable::new(),
            class_loader: Some(Rc::new(RefCell::new(ClassLoader::new(mock_classpath(), Rc::new(RefCell::new(Thread::new(None)))))))
        })
    }

    #[allow(dead_code)]
    pub fn mock_class() -> Class {
        Class {
            access_flags: 0u16,
            class_name: "".to_string(),
            constant_pool: ConstantPool {
                constant_info_map: BTreeMap::new()
            },
            fields: Vec::new(),
            methods: Vec::new(),
            super_class: None,
            next_instance_slot_id: 0usize,
            next_static_slot_id: 0usize,
            static_variable_table: VariableTable::new(),
            class_loader: Some(Rc::new(RefCell::new(ClassLoader::new(mock_classpath(), Rc::new(RefCell::new(Thread::new(None)))))))
        }
    }

    #[allow(dead_code)]
    pub fn mock_stack_frame() -> StackFrame {
        StackFrame::new(mock_rc_class(), mock_rc_method(), None)
    }
}
