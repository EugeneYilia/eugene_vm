use std::cell::RefMut;

use crate::constants::instruction_constants::OP_CODE_LENGTH;
use crate::core::bytecode_execution_engine::instruction::instruction_execute_result::InstructionExecuteResult;
use crate::core::code_reader::code_reader::CodeReader;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;

/// Execution then proceeds at that offset from the address of the opcode of this if<cond> instruction.
/// Otherwise, execution proceeds at the address of the instruction following this if<cond> instruction.

pub fn ifeq(code_reader: &mut CodeReader, mut thread: RefMut<Thread>) -> InstructionExecuteResult {
    // original_pc is current - instruction_op_code
    let original_pc = code_reader.pc - OP_CODE_LENGTH;

    let offset = code_reader.read_u16() as isize;

    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i32();
    if first.0 == 0 {
        InstructionExecuteResult {
            new_pc: (original_pc as isize + offset) as usize
        }
    } else {
        InstructionExecuteResult {
            new_pc: code_reader.pc
        }
    }
}

pub fn ifne(code_reader: &mut CodeReader, mut thread: RefMut<Thread>) -> InstructionExecuteResult {
    // original_pc is current - instruction_op_code
    let original_pc = code_reader.pc - OP_CODE_LENGTH;

    let offset = code_reader.read_u16() as isize;

    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i32();
    if first.0 != 0 {
        InstructionExecuteResult {
            new_pc: (original_pc as isize + offset) as usize
        }
    } else {
        InstructionExecuteResult {
            new_pc: code_reader.pc
        }
    }
}

pub fn iflt(code_reader: &mut CodeReader, mut thread: RefMut<Thread>) -> InstructionExecuteResult {
    // original_pc is current - instruction_op_code
    let original_pc = code_reader.pc - OP_CODE_LENGTH;

    let offset = code_reader.read_u16() as isize;

    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i32();
    if first.0 < 0 {
        InstructionExecuteResult {
            new_pc: (original_pc as isize + offset) as usize
        }
    } else {
        InstructionExecuteResult {
            new_pc: code_reader.pc
        }
    }
}

pub fn ifge(code_reader: &mut CodeReader, mut thread: RefMut<Thread>) -> InstructionExecuteResult {
    // original_pc is current - instruction_op_code
    let original_pc = code_reader.pc - OP_CODE_LENGTH;

    let offset = code_reader.read_u16() as isize;

    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i32();
    if first.0 >= 0 {
        InstructionExecuteResult {
            new_pc: (original_pc as isize + offset) as usize
        }
    } else {
        InstructionExecuteResult {
            new_pc: code_reader.pc
        }
    }
}

pub fn ifgt(code_reader: &mut CodeReader, mut thread: RefMut<Thread>) -> InstructionExecuteResult {
    // original_pc is current - instruction_op_code
    let original_pc = code_reader.pc - OP_CODE_LENGTH;

    let offset = code_reader.read_u16() as isize;

    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i32();
    if first.0 > 0 {
        InstructionExecuteResult {
            new_pc: (original_pc as isize + offset) as usize
        }
    } else {
        InstructionExecuteResult {
            new_pc: code_reader.pc
        }
    }
}

pub fn ifle(code_reader: &mut CodeReader, mut thread: RefMut<Thread>) -> InstructionExecuteResult {
    // original_pc is current - instruction_op_code
    let original_pc = code_reader.pc - OP_CODE_LENGTH;

    let offset = code_reader.read_u16() as isize;

    let stack_frame = thread.get_stack_frame_mut();
    let StackFrame { operand_stack, .. } = stack_frame;
    let first = operand_stack.pop_i32();
    if first.0 <= 0 {
        InstructionExecuteResult {
            new_pc: (original_pc as isize + offset) as usize
        }
    } else {
        InstructionExecuteResult {
            new_pc: code_reader.pc
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::num::Wrapping;
    use std::ops::Deref;
    use std::rc::Rc;

    use crate::core::bytecode_execution_engine::instruction::control::r#if::ifeq;
    use crate::core::bytecode_execution_engine::instruction::tests::mock_stack_frame;
    use crate::core::code_reader::code_reader::CodeReader;
    use crate::runtime::thread::Thread;

    #[test]
    fn test_ifeq_success() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_i32(Wrapping(0i32));
        let thread = Rc::new(RefCell::new(Thread::new(None)));
        thread.deref().borrow_mut().push_stack_frame(stack_frame);
        let instruction_execute_result = ifeq(&mut CodeReader::new(vec![21u8, 1u8, 2u8], 1usize), thread.deref().borrow_mut());
        assert_eq!(instruction_execute_result.new_pc, 258usize);
    }

    #[test]
    fn test_ifeq_fail() {
        let mut stack_frame = mock_stack_frame();
        stack_frame.operand_stack.push_i32(Wrapping(1i32));
        let thread = Rc::new(RefCell::new(Thread::new(None)));
        thread.deref().borrow_mut().push_stack_frame(stack_frame);
        let instruction_execute_result = ifeq(&mut CodeReader::new(vec![21u8, 1u8, 2u8], 1usize), thread.deref().borrow_mut());
        assert_eq!(instruction_execute_result.new_pc, 3usize);
    }
}