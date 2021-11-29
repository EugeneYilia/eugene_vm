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

    // 返回栈顶元素
    pub fn get_stack_frame_mut(&mut self) -> &mut StackFrame {
        self.stack.get_mut()
    }

    /// 判断是否还有stack_frame
    pub fn has_stack_frame(&self) -> bool {
        !self.stack.is_empty()
    }

    pub fn get_stack_size(&self) -> usize {
        self.stack.get_size()
    }
}