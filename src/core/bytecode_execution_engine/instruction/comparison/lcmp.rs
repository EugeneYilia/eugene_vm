use std::cell::RefMut;
use std::num::Wrapping;
use std::ops::Deref;

use crate::runtime::thread::Thread;

pub fn lcmp(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let second = stack_frame.operand_stack.pop_i64();
    let first = stack_frame.operand_stack.pop_i64();
    if first > second {
        stack_frame.operand_stack.push_i32(Wrapping(1i32));
    } else if first == second {
        stack_frame.operand_stack.push_i32(Wrapping(0i32));
    } else {
        stack_frame.operand_stack.push_i32(Wrapping(-1i32));
    }
}
