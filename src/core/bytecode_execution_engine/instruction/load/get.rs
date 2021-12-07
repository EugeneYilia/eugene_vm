use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::thread::Thread;

pub fn get_static(code_reader: &mut CodeReader, _thread: &mut Thread) -> InstructionExecuteResult {
    let static_field_index = code_reader.read_u16();
    // println!("{}", static_field_index);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}