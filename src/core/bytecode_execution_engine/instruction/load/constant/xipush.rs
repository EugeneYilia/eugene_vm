use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::thread::Thread;
use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::runtime::stack::stack_frame::StackFrame;

pub fn bipush(code_reader:&mut CodeReader,thread:&mut Thread)->InstructionExecuteResult{
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame{operand_stack,..} = stack_frame;

}

pub fn sipush(code_reader:&mut CodeReader,thread:&mut Thread)->InstructionExecuteResult {

}

#[cfg(test)]
mod tests {

}