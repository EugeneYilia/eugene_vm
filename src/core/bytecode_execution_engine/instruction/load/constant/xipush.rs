use std::cell::RefMut;
use std::num::Wrapping;

use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn bipush(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_i32(Wrapping(stack_frame.code_reader.read_i8() as i32));
}

pub fn sipush(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_i32(Wrapping(stack_frame.code_reader.read_i16() as i32));
}
