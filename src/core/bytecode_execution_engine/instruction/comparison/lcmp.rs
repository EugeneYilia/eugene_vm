use std::cell::RefMut;
use std::num::Wrapping;

use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn lcmp(code_reader: &mut CodeReader, mut thread: RefMut<Thread>) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let second = operand_stack.pop_i64();
    let first = operand_stack.pop_i64();
    if first > second {
        operand_stack.push_i32(Wrapping(1i32));
    } else if first == second {
        operand_stack.push_i32(Wrapping(0i32));
    } else {
        operand_stack.push_i32(Wrapping(-1i32));
    }
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}


#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::num::Wrapping;
    use std::ops::Deref;
    use std::rc::Rc;

    use crate::core::bytecode_execution_engine::instruction::comparison::lcmp::lcmp;
    use crate::core::bytecode_execution_engine::instruction::tests::mock_stack_frame;
    use crate::core::code_reader::code_reader::CodeReader;
    use crate::runtime::thread::Thread;

    #[test]
    fn test_lcmp_gt() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_i64(Wrapping(9223372036854775807i64));
        stack_frame.operand_stack.push_i64(Wrapping(9223372036854775806i64));
        let thread = Rc::new(RefCell::new(Thread::new(None)));
        thread.deref().borrow_mut().push_stack_frame(stack_frame);
        let instruction_execute_result = lcmp(&mut CodeReader::new(Rc::new(vec![]), 1usize), thread.deref().borrow_mut());
        let result = thread.deref().borrow_mut().pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result.0, 1i32);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_lcmp_lt() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_i64(Wrapping(-9223372036854775806i64));
        stack_frame.operand_stack.push_i64(Wrapping(9223372036854775807i64));
        let thread = Rc::new(RefCell::new(Thread::new(None)));
        thread.deref().borrow_mut().push_stack_frame(stack_frame);
        let instruction_execute_result = lcmp(&mut CodeReader::new(Rc::new(vec![]), 1usize), thread.deref().borrow_mut());
        let result = thread.deref().borrow_mut().pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result.0, -1i32);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }

    #[test]
    fn test_lcmp_eq() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_i64(Wrapping(-9223372036854775806i64));
        stack_frame.operand_stack.push_i64(Wrapping(-9223372036854775806i64));
        let thread = Rc::new(RefCell::new(Thread::new(None)));
        thread.deref().borrow_mut().push_stack_frame(stack_frame);
        let instruction_execute_result = lcmp(&mut CodeReader::new(Rc::new(vec![]), 1usize), thread.deref().borrow_mut());
        let result = thread.deref().borrow_mut().pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result.0, 0i32);
        assert_eq!(instruction_execute_result.new_pc, 1usize);
    }
}