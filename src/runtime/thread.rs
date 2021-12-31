use std::cell::{RefCell, RefMut};
use std::ops::Deref;
use std::rc::Rc;

use crate::constants::vm_constants::DEFAULT_MAX_STACK_SIZE;
use crate::core::bytecode_execution_engine::engine;
use crate::runtime::method_area::class::class::Class;
use crate::runtime::method_area::class::method::Method;
use crate::runtime::stack::stack::Stack;
use crate::runtime::stack::stack_frame::StackFrame;

#[derive(Debug)]
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

    pub fn get_stack_frame(&self) -> &StackFrame {
        self.stack.get()
    }

    /// 判断是否还有stack_frame
    pub fn has_stack_frame(&self) -> bool {
        !self.stack.is_empty()
    }

    pub fn get_stack_size(&self) -> usize {
        self.stack.get_size()
    }

    // stack bottom  method: A  pc: 13                       stack head
    // stack bottom  method: A  pc: 13     method: B  pc: 2  stack head
    pub fn start_thread(class: Rc<Class>, method: Rc<Method>, thread: Rc<RefCell<Thread>>) {
        let stack_frame = StackFrame::new(class, method);
        thread.deref().borrow_mut().push_stack_frame(stack_frame);
        engine::execute_instruction(thread);
    }

    pub fn invoke_method(class: Rc<Class>, method: Rc<Method>, thread: &mut RefMut<Thread>) {
        let stack_frame = StackFrame::new(class, method);
        thread.push_stack_frame(stack_frame);
        thread.pop_stack_frame();
    }
}