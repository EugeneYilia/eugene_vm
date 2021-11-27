use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn dcmpl(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let second = operand_stack.pop_f64();
    let first = operand_stack.pop_f64();

    if first > second {
        operand_stack.push_i32(1i32);
    } else if first == second {
        operand_stack.push_i32( 0i32);
    } else if first < second {
        operand_stack.push_i32(-1i32);
    } else {
        operand_stack.push_i32(-1i32);
    }

    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn dcmpg(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame{operand_stack,..} =stack_frame;
    let second= operand_stack.pop_f64();
    let first = operand_stack.pop_f64();
    if first > second {
        operand_stack.push_i32(1i32);
    } else if first == second {
        operand_stack.push_i32(0i32);
    } else if first < second {
        operand_stack.push_i32(-1i32);
    } else {
        operand_stack.push_i32(1i32);
    }

    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

#[cfg(test)]
mod tests {
    use crate::core::bytecode_execution_engine::instruction::tests::mock_stack_frame;
    use crate::core::code_reader::code_reader::CodeReader;
    use crate::runtime::thread::Thread;
    use crate::core::bytecode_execution_engine::instruction::comparison::dcmp::{dcmpl, dcmpg};

    #[test]
    fn test_dcmpl() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_f64(f64::NAN);
        stack_frame.operand_stack.push_f64(33.3f64);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = dcmpl(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result, -1i32);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_dcmpg() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_f64(f64::NAN);
        stack_frame.operand_stack.push_f64(33.3f64);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = dcmpg(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result, 1i32);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }
}