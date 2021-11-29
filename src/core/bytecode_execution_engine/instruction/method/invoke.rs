use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::thread::Thread;

/// 调用实例方法
pub fn invoke_virtual(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    todo!()
}

