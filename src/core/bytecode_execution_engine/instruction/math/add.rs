use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn iadd(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i32();
    let second = operand_stack.pop_i32();
    let result = first + second;
    operand_stack.push_i32(result);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn dadd(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_f64();
    let second = operand_stack.pop_f64();
    let result = first + second;
    operand_stack.push_f64(result);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn ladd(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i64();
    let second = operand_stack.pop_i64();
    let result = first + second;
    operand_stack.push_i64(result);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn fadd(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first =
}