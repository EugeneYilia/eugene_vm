use std::cell::RefMut;

use crate::constants::instruction_constants;
use crate::runtime::thread::Thread;
use crate::util::instruction_util::is_terminal_instruction;

pub fn execute_instruction(thread: &mut RefMut<Thread>) {
    loop {
        let instruction_op_code = thread.get_stack_frame_mut().code_reader.read_u8();
        let instruction_fn = instruction_constants::get_instruction_fn(instruction_op_code);
        println!("opcode: {:02X}", instruction_op_code);
        instruction_fn(thread);

        if is_terminal_instruction(&instruction_op_code) {
            break;
        }
    }
}