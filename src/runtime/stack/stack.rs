use std::cell::RefCell;
use std::rc::Rc;

use crate::runtime::stack::stack_frame::StackFrame;

#[derive(Debug)]
pub struct Stack {
    max_size: usize,
    stack_frame_vec: Vec<Rc<RefCell<StackFrame>>>,
}

impl Stack {
    pub fn new(max_size: usize) -> Stack {
        let stack_frame_vec = Vec::with_capacity(max_size);
        Stack { max_size, stack_frame_vec }
    }

    pub fn push(&mut self, stack_frame: Rc<RefCell<StackFrame>>) {
        if self.is_full() {
            panic!("Stack overflow: {:?}", self.stack_frame_vec);
        } else {
            self.stack_frame_vec.push(stack_frame);
        }
    }

    pub fn pop(&mut self) -> Rc<RefCell<StackFrame>> {
        if self.is_empty() {
            panic!("Stack is empty");
        } else {
            self.stack_frame_vec.pop().unwrap()
        }
    }

    pub fn get_last(&self) -> Rc<RefCell<StackFrame>> {
        if self.is_empty() {
            panic!("Stack is empty");
        } else {
            // index: 0 1 2 3
            // value: 8 6 7 9
            // len: 4
            // index: 3  value: 9
            Rc::clone(&self.stack_frame_vec[self.stack_frame_vec.len() - 1])
        }
    }

    pub fn is_empty(&self) -> bool {
        self.stack_frame_vec.is_empty()
    }

    pub fn is_full(&self) -> bool {
        self.stack_frame_vec.len() == self.max_size
    }

    pub fn get_size(&self) -> usize {
        self.stack_frame_vec.len()
    }
}