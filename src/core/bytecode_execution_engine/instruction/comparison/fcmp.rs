use std::num::Wrapping;

use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn fcmpl(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let second = operand_stack.pop_f32();
    let first = operand_stack.pop_f32();
    if first > second {
        operand_stack.push_i32(Wrapping(1i32));
    } else if first == second {
        operand_stack.push_i32(Wrapping(0i32));
    } else if first < second {
        operand_stack.push_i32(Wrapping(-1i32));
    } else {
        // 其中有一个数值为NAN时 将-1压入栈顶
        operand_stack.push_i32(Wrapping(-1i32));
    }
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn fcmpg(code_reader: &mut CodeReader, thread: &mut Thread) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let second = operand_stack.pop_f32();
    let first = operand_stack.pop_f32();
    if first > second {
        operand_stack.push_i32(Wrapping(1i32));
    } else if first == second {
        operand_stack.push_i32(Wrapping(0i32));
    } else if first < second {
        operand_stack.push_i32(Wrapping(-1i32));
    } else {
        // 其中有一个数值为NAN时 将1压入栈顶
        operand_stack.push_i32(Wrapping(1i32));
    }

    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

#[cfg(test)]
mod tests {
    use crate::core::bytecode_execution_engine::instruction::comparison::fcmp::{fcmpg, fcmpl};
    use crate::core::bytecode_execution_engine::instruction::tests::mock_stack_frame;
    use crate::core::code_reader::code_reader::CodeReader;
    use crate::runtime::thread::Thread;

    #[test]
    fn test_fcmpl() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_f32(f32::NAN);
        stack_frame.operand_stack.push_f32(33.3f32);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = fcmpl(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result.0, -1i32);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_fcmpg() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_f32(f32::NAN);
        stack_frame.operand_stack.push_f32(33.3f32);
        let mut thread = Thread::new(None);
        thread.push_stack_frame(stack_frame);
        let instruction_execute_result = fcmpg(&mut CodeReader::new(vec![], 1usize), &mut thread);
        let result = thread.pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result.0, 1i32);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }
}

#[test]
fn test_compare_with_nan() {
    // 和NAN比较结果都为false
    println!("{}", 1.0 > f32::NAN);
    println!("{}", 1.0 < f32::NAN);
    println!("{}", 1.0 == f32::NAN);
}