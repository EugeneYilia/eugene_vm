use std::cell::{RefCell, RefMut};
use std::num::Wrapping;
use std::ops::Deref;
use std::rc::Rc;

use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::bytecode_execution_engine::instruction::tests::mock_stack_frame;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;
use crate::util::instruction_util::variable_slot_type_is_kind_one;

pub fn swap(code_reader: &mut CodeReader, mut thread: RefMut<Thread>) -> InstructionExecuteResult {
    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;

    let first = operand_stack.pop();
    let second = operand_stack.pop();

    if variable_slot_type_is_kind_one(&first) && variable_slot_type_is_kind_one(&second) {
        operand_stack.extend_with_slice(&[first, second]);
    } else {
        panic!("swap error: variable_slot first: {:?}  variable_slot second: {:?}", first, second)
    }

    InstructionExecuteResult {
        new_pc: code_reader.pc
    }
}

#[test]
fn test_swap() {
    let mut stack_frame = mock_stack_frame();
    //         second  first
    // stack bottom 6 9 stack head
    //
    //  after
    //        second: 9   first: 6
    stack_frame.operand_stack.push_i32(Wrapping(6i32));
    stack_frame.operand_stack.push_i32(Wrapping(9i32));
    let thread = Rc::new(RefCell::new(Thread::new(None)));
    thread.deref().borrow_mut().push_stack_frame(stack_frame);
    let instruction_execute_result = swap(&mut CodeReader::new(Rc::new(vec![]), 1usize), thread.deref().borrow_mut());
    let mut operand_stack = thread.deref().borrow_mut().pop_stack_frame().operand_stack;
    let first = operand_stack.pop_i32();
    let second = operand_stack.pop_i32();
    assert_eq!(instruction_execute_result.new_pc, 1usize);
    assert_eq!(first.0, 6i32);
    assert_eq!(second.0, 9i32);
}