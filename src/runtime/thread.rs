use crate::constants::vm_constants::DEFAULT_MAX_STACK_SIZE;
use crate::runtime::stack::stack::Stack;
use crate::runtime::stack::stack_frame::StackFrame;

pub struct Thread {
    stack: Stack,
}

impl Thread {
    pub fn new(stack_size_option: Option<usize>) -> Thread {
        match stack_size_option {
            Some(stack_size) => Thread { stack: Stack::new(stack_size) },
            None => Thread { stack: Stack::new(DEFAULT_MAX_STACK_SIZE) }
        }
    }

    pub fn push_stack_frame(&mut self, stack_frame: StackFrame) {
        self.stack.push(stack_frame)
    }

    pub fn pop_stack_frame(&mut self) -> StackFrame {
        self.stack.pop()
    }
}