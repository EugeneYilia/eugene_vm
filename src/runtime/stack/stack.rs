use crate::runtime::stack::stack_frame::StackFrame;

pub struct Stack {
    max_size: usize,
    stack_frame_vec: Vec<StackFrame>,
}

impl Stack {
    pub fn new(max_size: usize) -> Stack {
        let stack_frame_vec: Vec<StackFrame> = Vec::with_capacity(max_size);
        Stack { max_size, stack_frame_vec }
    }

    pub fn push(&mut self, stack_frame: StackFrame) {
        if self.is_full() {
            panic!("Stack overflow: {:?}", self.stack_frame_vec);
        } else {
            self.stack_frame_vec.push(stack_frame);
        }
    }

    pub fn pop(&mut self) -> StackFrame {
        if self.is_empty() {
            panic!("Stack is empty");
        } else {
            self.stack_frame_vec.pop().unwrap()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack_frame_vec.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.stack_frame_vec.len() == self.max_size
    }
}