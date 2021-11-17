use std::collections::HashMap;
use std::rc::Rc;
use crate::runtime::method_area::class::class::Class;
use crate::runtime::method_area::class::method::Method;
use crate::runtime::stack::local_variables_table::VariableTable;
use crate::runtime::stack::operand_stack::OperandStack;

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
    use std::rc::Rc;
    use crate::runtime::method_area::class::method::Method;
    use crate::core::classfile::member_info::MemberInfo;

    #[test]
    fn test_create_frame() {
        let method_ref = Rc::new(Method::new(MemberInfo {

        }));
    }
}