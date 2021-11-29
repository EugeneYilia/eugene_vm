pub mod nop;
pub mod load;
pub mod store;
pub mod control;
pub mod comparison;
pub mod math;
pub mod method;
pub mod instruction_execute_result;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::rc::Rc;
    use crate::core::classfile::member_info::MemberInfo;
    use crate::runtime::method_area::class::class::Class;
    use crate::runtime::method_area::class::method::Method;
    use crate::runtime::method_area::constant_pool::constant_pool::ConstantPool;
    use crate::runtime::stack::stack_frame::StackFrame;
    use crate::runtime::stack::variables_table::VariableTable;

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
        })
    }

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
        }
    }

    pub fn mock_stack_frame() -> StackFrame {
        StackFrame::new(mock_rc_class(), mock_rc_method())
    }
}
