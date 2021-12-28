use std::cell::RefMut;

use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn iand(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i32();
    let second = operand_stack.pop_i32();
    let result = first & second;
    operand_stack.push_i32(result);
}

pub fn land(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i64();
    let second = operand_stack.pop_i64();
    let result = first & second;
    operand_stack.push_i64(result);
}
