use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::thread::Thread;
use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use std::collections::HashMap;
use crate::core::bytecode_execution_engine::instruction::nop::{nop, nop2};
    pub static  CODE_FN_MAP:HashMap<u8, Box<dyn Fn(&mut CodeReader, &mut Thread) -> InstructionExecuteResult>> = {
        let mut map = HashMap::new();
        map.insert(0x00u8,Box::new(nop));
        // map.insert(0x01u8,Box::new(nop2));
        map
    };

#[test]
fn test_code_fn_map() {
    // let code_fn_map:HashMap<u8,Box<dyn Fn(CodeReader, &mut Thread) -> InstructionExecuteResult>> = {
    //     let mut map : HashMap<u8,Box<dyn Fn(CodeReader, &mut Thread) -> InstructionExecuteResult>> = HashMap::new();
    //     map.insert(0x00,Box::new(nop));
    //     map
    // };

    let function = CODE_FN_MAP.get(&0x00).unwrap();
    let exec_result: InstructionExecuteResult = function(&mut CodeReader::new(vec![], 0), &mut Thread::new(None));
    println!("{:?}", exec_result);
}