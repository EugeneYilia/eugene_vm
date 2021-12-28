use std::cell::RefMut;

use crate::constants::instruction_constants;
use crate::runtime::thread::Thread;

pub fn execute_instruction(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_mut();
    let instruction_op_code = stack_frame.code_reader.read_u8();
    let instruction_fn = instruction_constants::get_instruction_fn(instruction_op_code);
    println!("opcode: {:02X}", instruction_op_code);
    instruction_fn(thread)
}