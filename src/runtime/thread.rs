use std::cell::{RefCell, RefMut};
use std::ops::Deref;
use std::rc::Rc;

use crate::constants::vm_constants::DEFAULT_MAX_STACK_SIZE;
use crate::core::bytecode_execution_engine::engine;
use crate::runtime::method_area::class::class::Class;
use crate::runtime::method_area::class::method::Method;
use crate::runtime::stack::stack::Stack;
use crate::runtime::stack::stack_frame::StackFrame;
use crate::runtime::stack::variable_slot::VariableSlot;

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

    pub fn push_stack_frame(&mut self, stack_frame: Rc<RefCell<StackFrame>>) {
        self.stack.push(stack_frame)
    }

    pub fn pop_stack_frame(&mut self) -> Rc<RefCell<StackFrame>> {
        self.stack.pop()
    }

    /// 判断是否还有stack_frame
    pub fn has_stack_frame(&self) -> bool {
        !self.stack.is_empty()
    }

    pub fn get_stack_frame_last(&self) -> Rc<RefCell<StackFrame>> {
        self.stack.get_last()
    }

    pub fn get_stack_size(&self) -> usize {
        self.stack.get_size()
    }

    // stack bottom  method: A  pc: 13                       stack head
    // stack bottom  method: A  pc: 13     method: B  pc: 2  stack head
    pub fn start_thread(class: Rc<Class>, method: Rc<Method>, thread: Rc<RefCell<Thread>>, args: Option<Vec<VariableSlot>>) {
        let stack_frame = Rc::new(RefCell::new(StackFrame::new(class, method, args)));
        thread.deref().borrow_mut().push_stack_frame(stack_frame);
        engine::execute_instruction(&mut thread.deref().borrow_mut());
    }

    // todo: 补充方法参数传入  将方法参数存放到局部变量表上local_variable_table
    pub fn invoke_method(class: Rc<Class>, method: Rc<Method>, thread: &mut RefMut<Thread>, args: Option<Vec<VariableSlot>>) {
        let stack_frame = Rc::new(RefCell::new(StackFrame::new(class, method, args)));
        thread.push_stack_frame(stack_frame);
        engine::execute_instruction(thread);
    }
}