use crate::runtime::thread::Thread;
use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;

pub fn execute_instruction(thread:&mut Thread,pc:usize) -> InstructionExecuteResult{
    let stack_frame = thread.pop_stack_frame();
    let mut code_reader = CodeReader::new(stack_frame.method.code.clone(),pc);
    let instuction_byte_code = code_reader.read_u8();

    todo!()
}