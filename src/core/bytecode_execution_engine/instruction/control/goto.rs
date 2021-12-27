use std::cell::RefMut;

use crate::constants::instruction_constants::OP_CODE_LENGTH;
use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::thread::Thread;

/// Execution proceeds at that offset from the address of the opcode of this goto instruction
pub fn goto(code_reader: &mut CodeReader, mut _thread: RefMut<Thread>) -> InstructionExecuteResult {
    // original_pc is current - instruction_op_code
    let original_pc = code_reader.pc - OP_CODE_LENGTH;
    let offset = code_reader.read_i16() as isize;
    InstructionExecuteResult {
        new_pc: (original_pc as isize + offset) as usize
    }
}