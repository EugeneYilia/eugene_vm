use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::stack::variable_slot::VariableSlot;
use crate::runtime::thread::Thread;

/// 将操作数栈上的数据 存储到 局部变量表上

fn _istore(stack_frame: &mut StackFrame, variable_index: usize) {
    stack_frame.local_variable_table.set_variable_slot(variable_index, VariableSlot::I32(stack_frame.operand_stack.pop_i32()));
}

pub fn istore_0(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame();
    _istore(stack_frame, 0);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn istore_1(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame();
    _istore(stack_frame, 1);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn istore_2(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame();
    _istore(stack_frame, 2);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn istore_3(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame();
    _istore(stack_frame, 3);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}