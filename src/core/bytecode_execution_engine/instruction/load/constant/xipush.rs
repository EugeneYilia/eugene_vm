use std::cell::RefMut;
use std::num::Wrapping;
use std::ops::Deref;

use crate::runtime::thread::Thread;

pub fn bipush(thread: &mut RefMut<Thread>) {
    let value = thread.get_stack_frame_last().deref().borrow_mut().code_reader.read_i8();
    thread.get_stack_frame_last().deref().borrow_mut().operand_stack.push_i32(Wrapping(value as i32));
}

pub fn sipush(thread: &mut RefMut<Thread>) {
    let value = thread.get_stack_frame_last().deref().borrow_mut().code_reader.read_i16();
    thread.get_stack_frame_last().deref().borrow_mut().operand_stack.push_i32(Wrapping(value as i32));
}
