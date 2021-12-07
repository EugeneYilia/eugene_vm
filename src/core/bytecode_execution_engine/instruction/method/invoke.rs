use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

/// 调用实例方法
pub fn invoke_virtual(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;

    let virtual_method_index = code_reader.read_u16();
    // println!("{}",virtual_method_index);

    let value = operand_stack.pop_i32();
    println!("{}", value);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

