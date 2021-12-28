use std::cell::RefMut;

use crate::constants::instruction_constants::OP_CODE_LENGTH;
use crate::runtime::thread::Thread;

/// Execution proceeds at that offset from the address of the opcode of this goto instruction
pub fn goto(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    // original_pc is current - instruction_op_code
    let original_pc = stack_frame.code_reader.pc - OP_CODE_LENGTH;
    let offset = stack_frame.code_reader.read_i16() as isize;
    stack_frame.code_reader.set_pc((original_pc as isize + offset) as usize);
}

