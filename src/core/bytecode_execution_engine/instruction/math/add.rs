use std::cell::RefMut;

use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn iadd(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i32();
    let second = operand_stack.pop_i32();
    let result = first + second;
    operand_stack.push_i32(result);
}

pub fn ladd(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i64();
    let second = operand_stack.pop_i64();
    let result = first + second;
    operand_stack.push_i64(result);
}

pub fn fadd(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_f32();
    let second = operand_stack.pop_f32();
    let result = first + second;
    operand_stack.push_f32(result);
}

pub fn dadd(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_f64();
    let second = operand_stack.pop_f64();
    let result = first + second;
    operand_stack.push_f64(result);
}
