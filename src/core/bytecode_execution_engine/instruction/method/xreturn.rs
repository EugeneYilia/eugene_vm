use std::cell::RefMut;
use std::ops::Deref;

use crate::runtime::stack::variable_slot::VariableSlot;
use crate::runtime::thread::Thread;

/// return
pub fn r#return(thread: &mut RefMut<Thread>) {
    thread.pop_stack_frame();
}

pub fn ireturn(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.pop_stack_frame();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    let return_variable_slot = stack_frame.operand_stack.pop();
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    if let VariableSlot::I32(_) = return_variable_slot {
        stack_frame.operand_stack.push(return_variable_slot);
    } else {
        panic!("ireturn error:  variable_slot is {:?}", return_variable_slot);
    }
}

pub fn lreturn(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.pop_stack_frame();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    let return_variable_slot = stack_frame.operand_stack.pop();
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    if let VariableSlot::I64(_) = return_variable_slot {
        stack_frame.operand_stack.push(return_variable_slot);
    } else {
        panic!("lreturn error:  variable_slot is {:?}", return_variable_slot);
    }
}

pub fn freturn(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.pop_stack_frame();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    let return_variable_slot = stack_frame.operand_stack.pop();
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    if let VariableSlot::F32(_) = return_variable_slot {
        stack_frame.operand_stack.push(return_variable_slot);
    } else {
        panic!("freturn error:  variable_slot is {:?}", return_variable_slot);
    }
}

pub fn dreturn(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.pop_stack_frame();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    let return_variable_slot = stack_frame.operand_stack.pop();
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    if let VariableSlot::F64(_) = return_variable_slot {
        stack_frame.operand_stack.push(return_variable_slot);
    } else {
        panic!("dreturn error:  variable_slot is {:?}", return_variable_slot);
    }
}

pub fn areturn(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.pop_stack_frame();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    let return_variable_slot = stack_frame.operand_stack.pop();
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    match return_variable_slot {
        VariableSlot::ObjectReference(_) | VariableSlot::ArrayReference(_) | VariableSlot::NullReference => {
            stack_frame.operand_stack.push(return_variable_slot);
        }
        _ => panic!("areturn error:  variable_slot is {:?}", return_variable_slot)
    }
}