use std::cell::RefMut;

use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::thread::Thread;

pub fn swap(code_reader: &mut CodeReader, mut thread: RefMut<Thread>) -> InstructionExecuteResult {
    todo!("完成处理逻辑");

    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}