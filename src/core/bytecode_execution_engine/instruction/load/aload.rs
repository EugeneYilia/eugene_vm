use std::cell::RefMut;
use std::ops::Deref;
use std::rc::Rc;

use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::stack::variable_slot::VariableSlot;
use crate::runtime::thread::Thread;

fn _aload(stack_frame: &mut StackFrame, variable_index: usize) {
    if let VariableSlot::ObjectReference(value) = stack_frame.local_variable_table.get_variable_slot(variable_index) {
        stack_frame.operand_stack.push(VariableSlot::ObjectReference(Rc::clone(value)));
    } else {
        panic!("variable_index: {} not point to VariableSlot::ObjectReference", variable_index);
    }
}

pub fn aload_0(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    _aload(&mut stack_frame, 0);
}

pub fn aload_1(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    _aload(&mut stack_frame, 1);
}

pub fn aload_2(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    _aload(&mut stack_frame, 2);
}

pub fn aload_3(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    _aload(&mut stack_frame, 3);
}
