use std::cell::{RefCell, RefMut};
use std::ops::Deref;
use std::rc::Rc;

use crate::constants::instruction_constants;
use crate::runtime::stack::stack::Stack;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::thread::Thread;
use crate::util::instruction_util::is_terminal_instruction;

pub fn execute_instruction(thread: Rc<RefCell<Thread>>) {
    loop {
        let instruction_op_code = thread.deref().borrow_mut().get_stack_frame_mut().code_reader.read_u8();
        let instruction_fn = instruction_constants::get_instruction_fn(instruction_op_code);
        debug!("opcode: {:02X}", instruction_op_code);
        instruction_fn(&mut thread.deref().borrow_mut());

        if is_terminal_instruction(&instruction_op_code) {
            break;
        }
    }
}