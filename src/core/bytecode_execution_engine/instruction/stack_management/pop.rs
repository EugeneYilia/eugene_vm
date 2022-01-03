use std::cell::RefMut;
use std::ops::Deref;

use crate::runtime::thread::Thread;
use crate::util::instruction_util::variable_slot_type_is_kind_one;

pub fn pop(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let variable_slot = stack_frame.operand_stack.pop();
    if !variable_slot_type_is_kind_one(&variable_slot) {
        panic!("pop error:  variable_slot is {:?}", variable_slot);
    }
}

pub fn pop2(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let variable_slot = stack_frame.operand_stack.pop();
    if variable_slot_type_is_kind_one(&variable_slot) {
        let next_variable_slot = stack_frame.operand_stack.pop();
        if !variable_slot_type_is_kind_one(&next_variable_slot) {
            panic!("pop2 error:  variable_slot is {:?}", variable_slot);
        }
    }
}