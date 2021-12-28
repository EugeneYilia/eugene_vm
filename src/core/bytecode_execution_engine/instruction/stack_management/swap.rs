use std::cell::RefMut;

use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;
use crate::util::instruction_util::variable_slot_type_is_kind_one;

pub fn swap(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;

    let first = operand_stack.pop();
    let second = operand_stack.pop();

    if variable_slot_type_is_kind_one(&first) && variable_slot_type_is_kind_one(&second) {
        operand_stack.extend_with_slice(&[first, second]);
    } else {
        panic!("swap error: variable_slot first: {:?}  variable_slot second: {:?}", first, second)
    }
}
