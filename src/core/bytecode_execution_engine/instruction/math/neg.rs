use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn ineg(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let original_value = operand_stack.pop_i32();
    operand_stack.push_i32(-original_value);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn lneg(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let original_value = operand_stack.pop_i64();
    operand_stack.push_i64(-original_value);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn fneg(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let original_value = operand_stack.pop_f32();
    operand_stack.push_f32(-original_value);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn dneg(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let original_value = operand_stack.pop_f64();
    operand_stack.push_f64(-original_value);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}


#[cfg(test)]
mod tests {
    use std::num::Wrapping;

    use crate::core::bytecode_execution_engine::instruction::math::neg::{dneg, fneg, ineg, lneg};
    use crate::core::bytecode_execution_engine::instruction::tests::mock_stack_frame;
    use crate::core::code_reader::code_reader::CodeReader;
    use crate::runtime::thread::Thread;

    #[test]
    fn test_ineg() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_i32(Wrapping(234556i32));
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = ineg(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result.0, -234556i32);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_lneg() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_i64(Wrapping(-54875845748435i64));
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = lneg(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_i64();
        assert_eq!(result.0, 54875845748435i64);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_fneg() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_f32(-100.7678f32);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = fneg(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_f32();
        assert_eq!(result, 100.7678f32);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_fneg_max_min() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_f32(f32::MAX);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = fneg(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_f32();
        assert_eq!(result, f32::MIN);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_dneg() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_f64(2f64);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = dneg(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_f64();
        assert_eq!(result, -2f64);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_dneg_zero() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_f64(0f64);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = dneg(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_f64();
        assert_eq!(result, -0f64);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_dneg_minus_zero() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_f64(-0f64);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = dneg(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_f64();
        assert_eq!(result, 0f64);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_dneg_inf() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_f64(f64::INFINITY);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = dneg(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_f64();
        assert_eq!(result, f64::NEG_INFINITY);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_divide_zero() {
        println!("{}", 3.0 / 0.0);
        println!("{}", 0.0 / 0.0);
        println!("{}", -3.0 / 0.0);
    }
}