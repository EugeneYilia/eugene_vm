use std::cell::RefMut;
use std::ops::Deref;

use crate::constants::instruction_constants;
use crate::runtime::thread::Thread;
use crate::util::instruction_util::is_terminal_instruction;

/// 每次只执行一个线程中同一栈帧的字节码指令列表
pub fn execute_instruction(thread: &mut RefMut<Thread>) {
    let stack_frame = thread.get_stack_frame_last();
    loop {
        let instruction_op_code = stack_frame.deref().borrow_mut().code_reader.read_u8();
        let instruction_fn = instruction_constants::get_instruction_fn(instruction_op_code);
        debug!("opcode: {:02X}", instruction_op_code);
        instruction_fn(thread);

        if is_terminal_instruction(&instruction_op_code) {
            break;
        }
    }
}