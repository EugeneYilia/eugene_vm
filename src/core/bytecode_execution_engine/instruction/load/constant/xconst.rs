use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn iconst_m1(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_i32(-1i32);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn iconst_0(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_i32(0i32);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn iconst_1(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_i32(1i32);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn iconst_2(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_i32(2i32);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn iconst_3(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_i32(3i32);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn iconst_4(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_i32(4i32);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn iconst_5(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_i32(5i32);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn lconst_0(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_i64(0i64);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn lconst_1(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_i64(1i64);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn fconst_0(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_f32(0f32);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn fconst_1(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_f32(1f32);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn fconst_2(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_f32(2f32);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn dconst_0(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_f64(0f64);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn dconst_1(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_f64(1f64);
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

#[cfg(test)]
mod tests {
    use crate::core::bytecode_execution_engine::instruction::load::constant::xconst::{dconst_0, dconst_1, iconst_m1, lconst_0};
    use crate::core::bytecode_execution_engine::instruction::tests::mock_stack_frame;
    use crate::core::code_reader::code_reader::CodeReader;
    use crate::runtime::thread::Thread;

    #[test]
    fn test_dconst_0() {
        let stack_frame = mock_stack_frame();
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = dconst_0(&mut CodeReader::new(vec![21u8, 32u8, 1u8, 2u8], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_f64();
        assert_eq!(result, 0f64);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_dconst_1() {
        let stack_frame = mock_stack_frame();
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = dconst_1(&mut CodeReader::new(vec![21u8, 32u8, 1u8, 2u8], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_f64();
        assert_eq!(result, 1f64);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_iconst_m1() {
        let stack_frame = mock_stack_frame();
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = iconst_m1(&mut CodeReader::new(vec![21u8, 32u8, 1u8, 2u8], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result, -1i32);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_lconst_0() {
        let stack_frame = mock_stack_frame();
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = lconst_0(&mut CodeReader::new(vec![21u8, 32u8, 1u8, 2u8], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_i64();
        assert_eq!(result, 0i64);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }
}