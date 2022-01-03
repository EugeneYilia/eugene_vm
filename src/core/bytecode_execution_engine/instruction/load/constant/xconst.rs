use std::cell::RefMut;
use std::num::Wrapping;
use std::ops::Deref;

use crate::runtime::stack::variable_slot::VariableSlot;
use crate::runtime::thread::Thread;

pub fn iconst_m1(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    stack_frame.operand_stack.push_i32(Wrapping(-1i32));
}

pub fn iconst_0(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    stack_frame.operand_stack.push_i32(Wrapping(0i32));
}

pub fn iconst_1(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    stack_frame.operand_stack.push_i32(Wrapping(1i32));
}

pub fn iconst_2(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    stack_frame.operand_stack.push_i32(Wrapping(2i32));
}

pub fn iconst_3(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    stack_frame.operand_stack.push_i32(Wrapping(3i32));
}

pub fn iconst_4(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    stack_frame.operand_stack.push_i32(Wrapping(4i32));
}

pub fn iconst_5(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    stack_frame.operand_stack.push_i32(Wrapping(5i32));
}

pub fn lconst_0(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    stack_frame.operand_stack.push_i64(Wrapping(0i64));
}

pub fn lconst_1(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    stack_frame.operand_stack.push_i64(Wrapping(1i64));
}

pub fn fconst_0(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    stack_frame.operand_stack.push_f32(0f32);
}

pub fn fconst_1(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    stack_frame.operand_stack.push_f32(1f32);
}

pub fn fconst_2(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    stack_frame.operand_stack.push_f32(2f32);
}

pub fn dconst_0(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    stack_frame.operand_stack.push_f64(0f64);
}

pub fn dconst_1(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    stack_frame.operand_stack.push_f64(1f64);
}

pub fn aconst_null(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    stack_frame.operand_stack.push(VariableSlot::NullReference);
}