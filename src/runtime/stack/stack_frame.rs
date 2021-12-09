use std::rc::Rc;

use crate::runtime::method_area::class::class::Class;
use crate::runtime::method_area::class::method::Method;
use crate::runtime::stack::operand_stack::OperandStack;
use crate::runtime::stack::variables_table::VariableTable;

#[derive(Debug)]
pub struct StackFrame {
    pub local_variable_table: VariableTable,
    pub operand_stack: OperandStack,
    pub method: Rc<Method>,
    pub class: Rc<Class>,
}

impl StackFrame {
    pub fn new(class: Rc<Class>, method: Rc<Method>) -> StackFrame {
        let Method {
            max_stack,
            ..
        } = *method;

        let local_variable_table = VariableTable::new();
        let operand_stack = OperandStack::new(max_stack);

        StackFrame {
            local_variable_table,
            operand_stack,
            method,
            class,
        }
    }
}

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
    use crate::runtime::stack::operand_stack::OperandStack;
    use crate::runtime::stack::stack_frame::StackFrame;
    use crate::runtime::stack::variable_slot::VariableSlot;
    use crate::runtime::stack::variables_table::VariableTable;

     fn mock_classpath() -> ClassPath {
        let user_classpath = Some("eugene_test/src_code/mine".to_owned());
        let boot_classpath = Some("eugene_test/src_code/eugene_rt".to_owned());
        let bootstrap_option = BootstrapOption::new("", user_classpath, boot_classpath, vec![]);
        ClassPath::parse_classpath(bootstrap_option.boot_classpath_option, bootstrap_option.user_classpath_option)
    }

    #[test]
    fn test_create_frame() {
        let method_ref = Rc::new(Method::new(&MemberInfo {
            access_flags: 0u16,
            name: "".to_string(),
            name_index: 0u16,
            descriptor_index: 0u16,
            descriptor: "".to_string(),
            attributes: Vec::new(),
        }));
        let class_ref = Rc::new(Class {
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
            class_loader: Some(Rc::new(RefCell::new(ClassLoader::new(mock_classpath()))))
        });
        let frame = StackFrame::new(class_ref, method_ref);
        check_local_variable_table(frame.local_variable_table);
        check_operand_stack(frame.operand_stack);
    }

    fn check_local_variable_table(mut local_variable_table: VariableTable) {
        local_variable_table.set_variable_slot(0, VariableSlot::I32(100));
        local_variable_table.set_variable_slot(1, VariableSlot::I32(-100));
        match local_variable_table.get_variable_slot(0) {
            VariableSlot::I32(value) => {
                assert_eq!(*value, 100)
            }
        }

        match local_variable_table.get_variable_slot(1) {
            VariableSlot::I32(value) => {
                assert_eq!(*value, -100)
            }
        }
    }

    fn check_operand_stack(mut operand_stack: OperandStack) {
        operand_stack.push_i32(100i32);
        operand_stack.push_f64(2.71828182845f64);
        operand_stack.push_i32(-100i32);
        operand_stack.push_i64(2997924580i64);
        operand_stack.push_f32(3.1415926f32);

        let f32_value = operand_stack.pop_f32();
        assert_eq!(f32_value, 3.1415926f32);
        let i64_value = operand_stack.pop_i64();
        assert_eq!(i64_value, 2997924580i64);
        let i32_value = operand_stack.pop_i32();
        assert_eq!(i32_value, -100i32);
        let f64_value = operand_stack.pop_f64();
        assert_eq!(f64_value, 2.71828182845f64);
        let i32_value = operand_stack.pop_i32();
        assert_eq!(i32_value, 100i32);
    }
}