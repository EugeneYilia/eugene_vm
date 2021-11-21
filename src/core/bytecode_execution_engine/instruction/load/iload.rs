use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::stack::variable_slot::VariableSlot;

fn _iload(stack_frame: &mut StackFrame, variable_index: usize) {
    let variable_slot = stack_frame.local_variable_table.get_variable_slot(variable_index);
    match variable_slot {
        VariableSlot::I32(value) => stack_frame.operand_stack.push_i32(*value),
        _ => panic!("variable slot type not match: {:?}", variable_slot)
    }
}

pub fn iload_0(){}