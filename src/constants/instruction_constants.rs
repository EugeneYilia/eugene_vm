use crate::core::bytecode_execution_engine::instruction::control::goto::goto;
use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::bytecode_execution_engine::instruction::load::constant::xipush::{bipush, sipush};
use crate::core::bytecode_execution_engine::instruction::load::iload::{iload_0, iload_1, iload_2, iload_3};
use crate::core::bytecode_execution_engine::instruction::math::add::{dadd, fadd, iadd, ladd};
use crate::core::bytecode_execution_engine::instruction::math::and::{iand, land};
use crate::core::bytecode_execution_engine::instruction::math::inc::{iinc};
use crate::core::bytecode_execution_engine::instruction::nop::nop;
use crate::core::bytecode_execution_engine::instruction::store::istore::{istore_0, istore_1, istore_2, istore_3};
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::thread::Thread;
use crate::core::bytecode_execution_engine::instruction::math::mul::{imul, lmul, fmul, dmul};
use crate::core::bytecode_execution_engine::instruction::math::neg::{dneg, fneg, ineg, lneg};
use crate::core::bytecode_execution_engine::instruction::load::constant::ldc::{ldc, ldc_w, ldc2_w};

pub fn get_instruction_fn(instruction_op_code: u8) -> fn(&mut CodeReader, &mut Thread) -> InstructionExecuteResult {
    match instruction_op_code {
        0x00 => nop,
        0x10 => bipush,
        0x11 => sipush,
        0x12 => ldc,
        0x13 => ldc_w,
        0x14 => ldc2_w,
        0x1a => iload_0,
        0x1b => iload_1,
        0x1c => iload_2,
        0x1d => iload_3,
        0x3b => istore_0,
        0x3c => istore_1,
        0x3d => istore_2,
        0x3e => istore_3,
        0x60 => iadd,
        0x61 => ladd,
        0x62 => fadd,
        0x63 => dadd,
        0x68 => imul,
        0x69 => lmul,
        0x6a => fmul,
        0x6b => dmul,
        0x74 => ineg,
        0x75 => lneg,
        0x76 => fneg,
        0x77 => dneg,
        0x7e => iand,
        0x7f => land,
        0x84 => iinc,
        0xa7 => goto,
        _ => panic!("illegal instruction op code: {}", instruction_op_code)
    }
}

#[test]
fn test_get_instruction_fn() {
    let function = get_instruction_fn(0x00);
    let exec_result: InstructionExecuteResult = function(&mut CodeReader::new(vec![], 0), &mut Thread::new(None));
    println!("{:?}", exec_result);
}