use std::cell::RefMut;
use std::ops::Deref;

use crate::runtime::thread::Thread;
use crate::util::instruction_util::variable_slot_type_is_kind_one;

pub fn swap(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();


    let first = stack_frame.operand_stack.pop();
    let second = stack_frame.operand_stack.pop();

    if variable_slot_type_is_kind_one(&first) && variable_slot_type_is_kind_one(&second) {
        stack_frame.operand_stack.extend_with_slice(&[first, second]);
    } else {
        panic!("swap error: variable_slot first: {:?}  variable_slot second: {:?}", first, second)
    }
}
