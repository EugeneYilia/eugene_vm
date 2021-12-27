use std::cell::RefMut;

use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::thread::Thread;

pub fn nop(code_reader: &mut CodeReader, _thread: RefMut<Thread>) -> InstructionExecuteResult {
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}