use std::cell::RefMut;
use std::num::Wrapping;

use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::stack::variable_slot::VariableSlot;
use crate::runtime::thread::Thread;

pub fn iinc(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame {
        local_variable_table,
        ..
    } = stack_frame;
    let local_variable_index = stack_frame.code_reader.read_u8() as usize;
    let change_value = stack_frame.code_reader.read_u8() as i32;

    if let VariableSlot::I32(value) = local_variable_table.get_variable_slot_mut(local_variable_index) {
        *value += Wrapping(change_value);
    } else {
        panic!("variable_index: {} not point to VariableSlot::I32", local_variable_index);
    }
}
