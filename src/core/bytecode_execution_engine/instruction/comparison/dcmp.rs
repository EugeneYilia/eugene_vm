use std::cell::RefMut;
use std::num::Wrapping;

use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn dcmpl(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let second = operand_stack.pop_f64();
    let first = operand_stack.pop_f64();

    if first > second {
        operand_stack.push_i32(Wrapping(1i32));
    } else if first == second {
        operand_stack.push_i32(Wrapping(0i32));
    } else if first < second {
        operand_stack.push_i32(Wrapping(-1i32));
    } else {
        operand_stack.push_i32(Wrapping(-1i32));
    }


}

pub fn dcmpg(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let second = operand_stack.pop_f64();
    let first = operand_stack.pop_f64();
    if first > second {
        operand_stack.push_i32(Wrapping(1i32));
    } else if first == second {
        operand_stack.push_i32(Wrapping(0i32));
    } else if first < second {
        operand_stack.push_i32(Wrapping(-1i32));
    } else {
        operand_stack.push_i32(Wrapping(1i32));
    }


}