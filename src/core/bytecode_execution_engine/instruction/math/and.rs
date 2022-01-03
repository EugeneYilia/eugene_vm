use std::cell::RefMut;
use std::ops::Deref;

use crate::runtime::thread::Thread;

pub fn iand(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let first = stack_frame.operand_stack.pop_i32();
    let second = stack_frame.operand_stack.pop_i32();
    let result = first & second;
    stack_frame.operand_stack.push_i32(result);
}

pub fn land(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let first = stack_frame.operand_stack.pop_i64();
    let second = stack_frame.operand_stack.pop_i64();
    let result = first & second;
    stack_frame.operand_stack.push_i64(result);
}
