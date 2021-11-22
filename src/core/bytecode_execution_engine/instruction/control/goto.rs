use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::thread::Thread;

pub fn goto(code_reader: &mut CodeReader, _thread: &mut Thread) -> InstructionExecuteResult {
    let original_pc = code_reader.pc;
    let offset = code_reader.read_i16() as isize;
    InstructionExecuteResult {
        new_pc: (original_pc as isize + offset) as usize
    }
}