use std::cell::RefMut;
use std::num::Wrapping;
use std::ops::Deref;

use crate::runtime::stack::variable_slot::VariableSlot;
use crate::runtime::thread::Thread;

pub fn iinc(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let local_variable_index = stack_frame.code_reader.read_u8() as usize;
    let change_value = stack_frame.code_reader.read_u8() as i32;

    if let VariableSlot::I32(value) = stack_frame.local_variable_table.get_variable_slot_mut(local_variable_index) {
        *value += Wrapping(change_value);
    } else {
        panic!("variable_index: {} not point to VariableSlot::I32", local_variable_index);
    }
}
