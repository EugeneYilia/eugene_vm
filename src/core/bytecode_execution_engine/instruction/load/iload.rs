use std::cell::RefMut;
use std::ops::Deref;

use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::stack::variable_slot::VariableSlot;
use crate::runtime::thread::Thread;

/// 将局部变量表上的数据 读取到 操作数栈上

fn _iload(stack_frame: &mut StackFrame, variable_index: usize) {
    if let VariableSlot::I32(value) = stack_frame.local_variable_table.get_variable_slot(variable_index) {
        stack_frame.operand_stack.push_i32(*value)
    } else {
        panic!("variable_index: {} not point to VariableSlot::I32", variable_index);
    }
}

pub fn iload_0(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    _iload(&mut stack_frame, 0);
}

pub fn iload_1(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    _iload(&mut stack_frame, 1);
}

pub fn iload_2(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    _iload(&mut stack_frame, 2);
}

pub fn iload_3(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    _iload(&mut stack_frame, 3);
}