use std::cell::RefMut;
use std::num::Wrapping;

use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

pub fn bipush(code_reader: &mut CodeReader, mut thread: RefMut<Thread>) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_i32(Wrapping(code_reader.read_i8() as i32));
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

pub fn sipush(code_reader: &mut CodeReader, mut thread: RefMut<Thread>) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    operand_stack.push_i32(Wrapping(code_reader.read_i16() as i32));
    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    use crate::core::bytecode_execution_engine::instruction::load::constant::xipush::{bipush, sipush};
    use crate::core::bytecode_execution_engine::instruction::tests::mock_stack_frame;
    use crate::core::code_reader::code_reader::CodeReader;
    use crate::runtime::thread::Thread;

    #[test]
    fn test_bipush() {
        let thread = Rc::new(RefCell::new(Thread::new(None)));
        thread.deref().borrow_mut().push_stack_frame(mock_stack_frame());
        let instruction_execute_result = bipush(&mut CodeReader::new(vec![12u8, 13u8, 14u8], 1usize), thread.deref().borrow_mut());
        let result = thread.deref().borrow_mut().pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result.0, 13i32);
        assert_eq!(instruction_execute_result.new_pc, 2usize);
    }

    #[test]
    fn test_sipush() {
        let thread = Rc::new(RefCell::new(Thread::new(None)));
        thread.deref().borrow_mut().push_stack_frame(mock_stack_frame());
        let instruction_execute_result = sipush(&mut CodeReader::new(vec![12u8, 13u8, 14u8], 1usize), thread.deref().borrow_mut());
        let result = thread.deref().borrow_mut().pop_stack_frame().operand_stack.pop_i32();
        assert_eq!(result.0, 13 * 256 + 14);
        assert_eq!(instruction_execute_result.new_pc, 3usize);
    }
}