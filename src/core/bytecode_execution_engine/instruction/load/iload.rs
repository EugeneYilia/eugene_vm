use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::stack::variable_slot::VariableSlot;
use crate::runtime::thread::Thread;

/// 将局部变量表上的数据 读取到 操作数栈上

fn _iload(stack_frame: &mut StackFrame, variable_index: usize) {
    let variable_slot = stack_frame.local_variable_table.get_variable_slot(variable_index);
    match variable_slot {
        VariableSlot::I32(value) => stack_frame.operand_stack.push_i32(*value),
    }
}

pub fn iload_0(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    _iload(stack_frame, 0);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn iload_1(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let mut stack_frame = thread.get_stack_frame_mut();
    _iload(&mut stack_frame, 1);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn iload_2(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let mut stack_frame = thread.get_stack_frame_mut();
    _iload(&mut stack_frame, 2);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn iload_3(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let mut stack_frame = thread.get_stack_frame_mut();
    _iload(&mut stack_frame, 3);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}