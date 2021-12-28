use std::cell::{RefCell, RefMut};
use std::ops::Deref;
use std::rc::Rc;

use crate::core::bytecode_execution_engine::instruction::comparison::dcmp::{dcmpg, dcmpl};
use crate::core::bytecode_execution_engine::instruction::comparison::fcmp::{fcmpg, fcmpl};
use crate::core::bytecode_execution_engine::instruction::comparison::lcmp::lcmp;
use crate::core::bytecode_execution_engine::instruction::control::goto::goto;
use crate::core::bytecode_execution_engine::instruction::control::if_icmp::{if_icmpeq, if_icmpge, if_icmpgt, if_icmple, if_icmplt, if_icmpne};
use crate::core::bytecode_execution_engine::instruction::control::r#if::*;
use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::bytecode_execution_engine::instruction::load::constant::ldc::{ldc, ldc2_w, ldc_w};
use crate::core::bytecode_execution_engine::instruction::load::constant::xconst::{aconst_null, dconst_0, dconst_1, fconst_0, fconst_1, fconst_2, iconst_0, iconst_1, iconst_2, iconst_3, iconst_4, iconst_5, iconst_m1, lconst_0, lconst_1};
use crate::core::bytecode_execution_engine::instruction::load::constant::xipush::{bipush, sipush};
use crate::core::bytecode_execution_engine::instruction::load::get::get_static;
use crate::core::bytecode_execution_engine::instruction::load::iload::{iload_0, iload_1, iload_2, iload_3};
use crate::core::bytecode_execution_engine::instruction::math::add::{dadd, fadd, iadd, ladd};
use crate::core::bytecode_execution_engine::instruction::math::and::{iand, land};
use crate::core::bytecode_execution_engine::instruction::math::inc::iinc;
use crate::core::bytecode_execution_engine::instruction::math::mul::{dmul, fmul, imul, lmul};
use crate::core::bytecode_execution_engine::instruction::math::neg::{dneg, fneg, ineg, lneg};
use crate::core::bytecode_execution_engine::instruction::method::invoke::invoke_virtual;
use crate::core::bytecode_execution_engine::instruction::method::xreturn::r#return;
use crate::core::bytecode_execution_engine::instruction::nop::nop;
use crate::core::bytecode_execution_engine::instruction::stack_management::dup::{dup, dup2, dup2_x1, dup2_x2, dup_x1, dup_x2};
use crate::core::bytecode_execution_engine::instruction::stack_management::swap::swap;
use crate::core::bytecode_execution_engine::instruction::store::istore::{istore_0, istore_1, istore_2, istore_3};
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::thread::Thread;

pub const OP_CODE_LENGTH: usize = 1usize;

pub fn get_instruction_fn(instruction_op_code: u8) -> fn(&mut CodeReader, RefMut<Thread>) -> InstructionExecuteResult {
    match instruction_op_code {
        0x00 => nop,
        0x01 => aconst_null,
        0x02 => iconst_m1,
        0x03 => iconst_0,
        0x04 => iconst_1,
        0x05 => iconst_2,
        0x06 => iconst_3,
        0x07 => iconst_4,
        0x08 => iconst_5,
        0x09 => lconst_0,
        0x0a => lconst_1,
        0x0b => fconst_0,
        0x0c => fconst_1,
        0x0d => fconst_2,
        0x0e => dconst_0,
        0x0f => dconst_1,
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
        0x59 => dup,
        0x5a => dup_x1,
        0x5b => dup_x2,
        0x5c => dup2,
        0x5d => dup2_x1,
        0x5e => dup2_x2,
        0x5f => swap,
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
        0x94 => lcmp,
        0x95 => fcmpl,
        0x96 => fcmpg,
        0x97 => dcmpl,
        0x98 => dcmpg,
        0x99 => ifeq,
        0x9a => ifne,
        0x9b => iflt,
        0x9c => ifge,
        0x9d => ifgt,
        0x9e => ifle,
        0x9f => if_icmpeq,
        0xa9 => if_icmpne,
        0xa1 => if_icmplt,
        0xa2 => if_icmpge,
        0xa3 => if_icmpgt,
        0xa4 => if_icmple,
        0xa7 => goto,
        0xb1 => r#return,
        0xb2 => get_static,
        0xb6 => invoke_virtual,
        _ => panic!("illegal instruction op code: {}", instruction_op_code)
    }
}

#[test]
fn test_get_instruction_fn() {
    let thread = Rc::new(RefCell::new(Thread::new(None)));
    let function = get_instruction_fn(0x00);
    let exec_result: InstructionExecuteResult = function(&mut CodeReader::new(Rc::new(vec![]), 0), thread.deref().borrow_mut());
    println!("{:?}", exec_result);
}