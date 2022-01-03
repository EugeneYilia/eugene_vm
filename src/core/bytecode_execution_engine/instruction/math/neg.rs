use std::cell::RefMut;
use std::ops::Deref;

use crate::runtime::thread::Thread;

pub fn ineg(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let original_value = stack_frame.operand_stack.pop_i32();
    stack_frame.operand_stack.push_i32(-original_value);
}

pub fn lneg(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let original_value = stack_frame.operand_stack.pop_i64();
    stack_frame.operand_stack.push_i64(-original_value);
}

pub fn fneg(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let original_value = stack_frame.operand_stack.pop_f32();
    stack_frame.operand_stack.push_f32(-original_value);
}

pub fn dneg(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let original_value = stack_frame.operand_stack.pop_f64();
    stack_frame.operand_stack.push_f64(-original_value);
}
