use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn imul(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i32();
    let second = operand_stack.pop_i32();
    let result = first * second;
    operand_stack.push_i32(result);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn lmul(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i64();
    let second = operand_stack.pop_i64();
    let result = first * second;
    operand_stack.push_i64(result);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn fmul(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_f32();
    let second = operand_stack.pop_f32();
    let result = first * second;
    operand_stack.push_f32(result);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn dmul(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_f64();
    let second = operand_stack.pop_f64();
    let result = first * second;
    operand_stack.push_f64(result);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

#[cfg(test)]
mod tests {
    use crate::core::bytecode_execution_engine::instruction::math::mul::{dmul, fmul, imul, lmul};
    use crate::core::bytecode_execution_engine::instruction::tests::mock_stack_frame;
    use crate::core::code_reader::code_reader::CodeReader;
    use crate::runtime::thread::Thread;

    #[test]
    fn test_imul() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_i32(2i32);
        stack_frame.operand_stack.push_i32(3i32);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = imul(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result, 6i32);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_lmul() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_i64(1234567890i64);
        stack_frame.operand_stack.push_i64(2997924580i64);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = lmul(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_i64();
        assert_eq!(result, 3701141423109736200i64);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_fmul() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_f32(2.71828182845f32);
        stack_frame.operand_stack.push_f32(3.1415926535897926f32);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = fmul(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_f32();
        assert_eq!(result, 8.53973422264514888498427947f32);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_dmul() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_f64(2.71828182845f64);
        stack_frame.operand_stack.push_f64(3.1415926535897926f64);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = dmul(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_f64();
        assert_eq!(result, 8.53973422264514888498427947f64);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }
}