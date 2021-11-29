use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn bipush(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_i32(code_reader.read_i8() as i32);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn sipush(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_i32(code_reader.read_i16() as i32);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

#[cfg(test)]
mod tests {
    use crate::core::bytecode_execution_engine::instruction::load::constant::xipush::{bipush, sipush};
    use crate::core::bytecode_execution_engine::instruction::tests::mock_stack_frame;
    use crate::core::code_reader::code_reader::CodeReader;
    use crate::runtime::thread::Thread;

    #[test]
    fn test_bipush() {
        let mut thread = Thread::new(None);
        thread.push_stack_frame(mock_stack_frame());
        let instruction_execute_result = bipush(&mut CodeReader::new(vec![12u8, 13u8, 14u8], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result, 13i32);
        assert_eq!(instruction_execute_result.new_pc, 2usize);
    }

    #[test]
    fn test_sipush() {
        let mut thread = Thread::new(None);
        thread.push_stack_frame(mock_stack_frame());
        let instruction_execute_result = sipush(&mut CodeReader::new(vec![12u8, 13u8, 14u8], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result, 13 * 256 + 14);
        assert_eq!(instruction_execute_result.new_pc, 3usize);
    }
}