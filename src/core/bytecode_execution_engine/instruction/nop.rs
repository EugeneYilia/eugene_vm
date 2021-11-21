use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::thread::Thread;
use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;

pub fn nop(code_reader: &mut CodeReader, _thread: &mut Thread) -> InstructionExecuteResult {
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}