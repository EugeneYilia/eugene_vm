use crate::runtime::thread::Thread;
use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::constants::instruction_constants;

pub fn execute_instruction(thread: &mut Thread, pc: usize) -> InstructionExecuteResult {
    let mut code_reader: CodeReader;
    {
        let stack_frame = thread.get_stack_frame();
        code_reader = CodeReader::new(stack_frame.method.code.clone(), pc);
    }
    let instruction_byte_code = code_reader.read_u8();

    let instruction_fn = instruction_constants::get_instruction_fn(instruction_byte_code);
    instruction_fn(&mut code_reader, thread)
}