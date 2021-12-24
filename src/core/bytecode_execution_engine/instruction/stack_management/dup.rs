use std::num::Wrapping;

use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::thread::Thread;

pub fn dup(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

#[test]
fn test_wrapping() {
    let a = Wrapping(i32::MAX);
    let b = Wrapping(i32::MAX);
    let c = a + b;
    println!("{}", c);

    let d = Wrapping(i64::MAX);
    let e = Wrapping(i64::MAX);
    let f = d + e;
    println!("{}", f);

    let g = Wrapping(3);
    let h = Wrapping(4);
    println!("{}", g > h);
    println!("{}", g <= h);
    println!("{}", g + h);
}