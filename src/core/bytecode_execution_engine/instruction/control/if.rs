use std::cell::RefMut;
use std::ops::Deref;

use crate::constants::instruction_constants::OP_CODE_LENGTH;
use crate::runtime::thread::Thread;

/// Execution then proceeds at that offset from the address of the opcode of this if<cond> instruction.
/// Otherwise, execution proceeds at the address of the instruction following this if<cond> instruction.

pub fn ifeq(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    // original_pc is current - instruction_op_code
    let original_pc = stack_frame.code_reader.pc - OP_CODE_LENGTH;

    let offset = stack_frame.code_reader.read_u16() as isize;

    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let first = stack_frame.operand_stack.pop_i32();
    if first.0 == 0 {
        stack_frame.code_reader.set_pc((original_pc as isize + offset) as usize);
    }
}

pub fn ifne(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    // original_pc is current - instruction_op_code
    let original_pc = stack_frame.code_reader.pc - OP_CODE_LENGTH;

    let offset = stack_frame.code_reader.read_u16() as isize;

    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let first = stack_frame.operand_stack.pop_i32();
    if first.0 != 0 {
        stack_frame.code_reader.set_pc((original_pc as isize + offset) as usize);
    }
}

pub fn iflt(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    // original_pc is current - instruction_op_code
    let original_pc = stack_frame.code_reader.pc - OP_CODE_LENGTH;

    let offset = stack_frame.code_reader.read_u16() as isize;

    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let first = stack_frame.operand_stack.pop_i32();
    if first.0 < 0 {
        stack_frame.code_reader.set_pc((original_pc as isize + offset) as usize);
    }
}

pub fn ifge(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    // original_pc is current - instruction_op_code
    let original_pc = stack_frame.code_reader.pc - OP_CODE_LENGTH;

    let offset = stack_frame.code_reader.read_u16() as isize;

    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let first = stack_frame.operand_stack.pop_i32();
    if first.0 >= 0 {
        stack_frame.code_reader.set_pc((original_pc as isize + offset) as usize);
    }
}

pub fn ifgt(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    // original_pc is current - instruction_op_code
    let original_pc = stack_frame.code_reader.pc - OP_CODE_LENGTH;

    let offset = stack_frame.code_reader.read_u16() as isize;

    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let first = stack_frame.operand_stack.pop_i32();
    if first.0 > 0 {
        stack_frame.code_reader.set_pc((original_pc as isize + offset) as usize);
    }
}

pub fn ifle(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();
    // original_pc is current - instruction_op_code
    let original_pc = stack_frame.code_reader.pc - OP_CODE_LENGTH;

    let offset = stack_frame.code_reader.read_u16() as isize;

    let stack_frame = thread.get_stack_frame_last();
    let mut stack_frame = stack_frame.deref().borrow_mut();

    let first = stack_frame.operand_stack.pop_i32();
    if first.0 <= 0 {
        stack_frame.code_reader.set_pc((original_pc as isize + offset) as usize);
    }
}