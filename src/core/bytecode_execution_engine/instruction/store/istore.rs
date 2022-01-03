use std::cell::RefMut;
use std::ops::Deref;

use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::stack::variable_slot::VariableSlot;
use crate::runtime::thread::Thread;

/// 将操作数栈上的数据 存储到 局部变量表上

fn _istore(stack_frame: &mut StackFrame, variable_index: usize) {
    stack_frame.local_variable_table.set_variable_slot(variable_index, VariableSlot::I32(stack_frame.operand_stack.pop_i32()));
}

pub fn istore_0(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    _istore(&mut stack_frame, 0);
}

pub fn istore_1(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    _istore(&mut stack_frame, 1);
}

pub fn istore_2(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    _istore(&mut stack_frame, 2);
}

pub fn istore_3(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    _istore(&mut stack_frame, 3);
}