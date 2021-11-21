use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::thread::Thread;
use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::bytecode_execution_engine::instruction::nop::nop;
use crate::core::bytecode_execution_engine::instruction::load::iload::{iload_0, iload_2, iload_1};

pub fn get_instruction_fn(instruction_code: u8) -> fn(&mut CodeReader, &mut Thread) -> InstructionExecuteResult {
    match instruction_code {
        0x00 => nop,
        0x1a => iload_0,
        0x1b => iload_1,
        0x1c => iload_2,
        _ => panic!("illegal instruction code: {}", instruction_code)
    }
}

#[test]
fn test_get_instruction_fn(){
    let function = get_instruction_fn(0x00);
    let exec_result: InstructionExecuteResult = function(&mut CodeReader::new(vec![], 0), &mut Thread::new(None));
    println!("{:?}", exec_result);
}