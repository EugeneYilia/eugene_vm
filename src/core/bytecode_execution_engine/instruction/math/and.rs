use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn iand(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i32();
    let second = operand_stack.pop_i32();
    let result = first & second;
    operand_stack.push_i32(result);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn land(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i64();
    let second = operand_stack.pop_i64();
    let result = first & second;
    operand_stack.push_i64(result);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

#[cfg(test)]
mod tests {
    use std::num::Wrapping;

    use crate::core::bytecode_execution_engine::instruction::math::and::{iand, land};
    use crate::core::bytecode_execution_engine::instruction::tests::mock_stack_frame;
    use crate::core::code_reader::code_reader::CodeReader;
    use crate::runtime::thread::Thread;

    #[test]
    fn test_iand() {
        let mut stack_frame = mock_stack_frame();
        //  101011110
        stack_frame.operand_stack.push_i32(Wrapping(350i32));
        // 1010100110
        stack_frame.operand_stack.push_i32(Wrapping(678i32));
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        // 6
        let instruction_execute_result = iand(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result.0, 6i32);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_land() {
        let mut stack_frame = mock_stack_frame();
        //   31   28    25  23   20        12      6  5  = 32 + 64 + 4096 + 1048576 + 8388608 + 33554432 + 268435456 + 2147483648  = 2458914912
        // 1011011111110111000001110001111001
        //   10110010101100001011001011100100
        stack_frame.operand_stack.push_i64(Wrapping(12345678969i64));
        stack_frame.operand_stack.push_i64(Wrapping(2997924580i64));
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = land(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_i64();
        assert_eq!(result.0, 2458914912i64);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }
}
