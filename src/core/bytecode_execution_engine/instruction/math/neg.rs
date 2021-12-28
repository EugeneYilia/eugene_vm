use std::cell::RefMut;

use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn ineg(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let original_value = operand_stack.pop_i32();
    operand_stack.push_i32(-original_value);
}

pub fn lneg(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let original_value = operand_stack.pop_i64();
    operand_stack.push_i64(-original_value);
}

pub fn fneg(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let original_value = operand_stack.pop_f32();
    operand_stack.push_f32(-original_value);
}

pub fn dneg(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let original_value = operand_stack.pop_f64();
    operand_stack.push_f64(-original_value);
}
